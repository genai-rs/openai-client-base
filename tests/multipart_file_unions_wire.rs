//! What the multipart unions with file variants put on the wire, captured against a local server. A path variant is an
//! upload, never the text of the path, and every other variant keeps its text or JSON form.

use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use openai_client_base::apis::{configuration::Configuration, skills_api, videos_api, Error};
use openai_client_base::models;

/// Not valid UTF-8 and no JSON, so nothing but the file's bytes can pass for them.
const PNG: &[u8] = b"\x89PNG\r\n\x00\xff first";
const ZIP: &[u8] = b"PK\x03\x04\x00\xfe second";

static TEMP_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct CapturedRequest {
    request_line: String,
    headers: String,
    body: Vec<u8>,
}

#[derive(Debug)]
struct Part {
    headers: String,
    body: Vec<u8>,
}

impl Part {
    fn content_disposition(&self) -> &str {
        header_value(&self.headers, "content-disposition").expect("part content disposition")
    }

    fn content_type(&self) -> Option<&str> {
        header_value(&self.headers, "content-type")
    }

    fn has_name(&self, name: &str) -> bool {
        self.content_disposition()
            .split(';')
            .any(|attribute| attribute.trim() == format!("name=\"{name}\""))
    }

    fn file_name(&self) -> Option<&str> {
        self.content_disposition().split(';').find_map(|attribute| {
            attribute
                .trim()
                .strip_prefix("filename=\"")?
                .strip_suffix('"')
        })
    }
}

fn request_once() -> (String, Receiver<CapturedRequest>, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback mock");
    listener
        .set_nonblocking(true)
        .expect("set nonblocking loopback listener");
    let address = listener.local_addr().expect("mock address").to_string();
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(10);
        let (mut stream, _) = loop {
            match listener.accept() {
                Ok(connection) => break connection,
                Err(error)
                    if error.kind() == std::io::ErrorKind::WouldBlock
                        && Instant::now() < deadline =>
                {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => panic!("accept request before deadline: {error}"),
            }
        };
        stream
            .set_nonblocking(false)
            .expect("set blocking loopback stream");
        stream
            .set_read_timeout(Some(Duration::from_secs(3)))
            .expect("set read timeout");
        let request = read_request(&mut stream);
        // The tests look at the request, so the response only has to end the call.
        let body = "{}";
        let response = format!(
            "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream
            .write_all(response.as_bytes())
            .expect("write mock response");
        stream.flush().expect("flush mock response");
        sender.send(request).expect("send captured request");
    });
    (address, receiver, handle)
}

fn read_request(stream: &mut TcpStream) -> CapturedRequest {
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 8192];
    let (header_end, content_length) = loop {
        let count = stream.read(&mut chunk).expect("read request");
        assert!(count > 0, "connection closed before complete request");
        bytes.extend_from_slice(&chunk[..count]);
        if let Some(end) = find_bytes(&bytes, b"\r\n\r\n", 0) {
            let header_text = String::from_utf8_lossy(&bytes[..end]);
            let body_length = header_value(&header_text, "content-length")
                .map(|value| value.parse::<usize>().expect("content length"))
                .unwrap_or(0);
            if bytes.len() >= end + 4 + body_length {
                break (end, body_length);
            }
        }
    };
    let headers = String::from_utf8_lossy(&bytes[..header_end]).to_string();
    let request_line = headers.lines().next().expect("request line").to_string();
    let body = bytes[header_end + 4..header_end + 4 + content_length].to_vec();
    CapturedRequest {
        request_line,
        headers,
        body,
    }
}

fn find_bytes(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    if needle.is_empty() || start > haystack.len() || needle.len() > haystack.len() - start {
        return None;
    }
    haystack[start..]
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|offset| start + offset)
}

fn parts(request: &CapturedRequest) -> Vec<Part> {
    let content_type = header_value(&request.headers, "content-type").expect("content type");
    assert!(content_type.starts_with("multipart/form-data; boundary="));
    let boundary = content_type
        .split_once("boundary=")
        .expect("multipart boundary")
        .1
        .trim_matches('"');
    let marker = format!("--{boundary}").into_bytes();
    let mut parts = Vec::new();
    let mut cursor = 0;
    while let Some(marker_at) = find_bytes(&request.body, &marker, cursor) {
        let mut start = marker_at + marker.len();
        if request.body.get(start..start + 2) == Some(b"--") {
            break;
        }
        start += 2;
        let headers_end =
            find_bytes(&request.body, b"\r\n\r\n", start).expect("part headers terminator");
        let headers = String::from_utf8_lossy(&request.body[start..headers_end]).to_string();
        let body_start = headers_end + 4;
        let next_marker = find_bytes(&request.body, &marker, body_start).expect("next boundary");
        parts.push(Part {
            headers,
            body: request.body[body_start..next_marker - 2].to_vec(),
        });
        cursor = next_marker;
    }
    parts
}

fn header_value<'a>(headers: &'a str, name: &str) -> Option<&'a str> {
    headers.lines().find_map(|line| {
        let (candidate, value) = line.split_once(':')?;
        candidate.eq_ignore_ascii_case(name).then(|| value.trim())
    })
}

fn config(address: &str) -> Configuration {
    let mut configuration = Configuration::new();
    configuration.base_path = format!("http://{address}");
    configuration.user_agent = None;
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(2))
        .timeout(Duration::from_secs(6))
        .build()
        .expect("build timed loopback HTTP client");
    configuration.client = reqwest_middleware::ClientBuilder::new(client).build();
    configuration
}

fn receive_one(captured: Receiver<CapturedRequest>, server: JoinHandle<()>) -> CapturedRequest {
    let result = captured.recv_timeout(Duration::from_secs(12));
    server
        .join()
        .expect("mock server thread completes and is reaped");
    result.expect("exactly one HTTP request captured before deadline")
}

/// A file in a directory of its own, so the part's file name is known, removed when dropped.
struct Upload(PathBuf);

impl Upload {
    fn new(name: &str, bytes: &[u8]) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let sequence = TEMP_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "multipart-file-unions-{}-{nonce}-{sequence}",
            std::process::id()
        ));
        fs::create_dir(&dir).expect("create upload fixture directory");
        let path = dir.join(name);
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .expect("create upload fixture");
        file.write_all(bytes).expect("write upload fixture");
        Self(path)
    }

    fn path(&self) -> PathBuf {
        self.0.clone()
    }
}

impl Drop for Upload {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
        if let Some(dir) = self.0.parent() {
            let _ = fs::remove_dir(dir);
        }
    }
}

fn a_missing_file() -> PathBuf {
    std::env::temp_dir().join("multipart-file-unions-this-file-does-not-exist.zip")
}

/// Nothing listens here, and a call that fails on its file never gets this far.
fn config_without_server() -> Configuration {
    config("127.0.0.1:9")
}

fn assert_file_part(part: &Part, name: &str, file_name: &str, bytes: &[u8]) {
    assert!(part.has_name(name), "{part:?}");
    assert_eq!(part.file_name(), Some(file_name), "{part:?}");
    assert_eq!(part.body, bytes);
    // An ordinary file parameter sends no media type of its own.
    assert_eq!(part.content_type(), None, "{part:?}");
}

fn named<'a>(parts: &'a [Part], name: &str) -> Vec<&'a Part> {
    parts.iter().filter(|part| part.has_name(name)).collect()
}

#[tokio::test]
async fn create_video_uploads_the_input_reference_path() {
    let upload = Upload::new("reference.png", PNG);
    let (address, captured, server) = request_once();
    let configuration = config(&address);
    let _ = videos_api::create_video()
        .configuration(&configuration)
        .prompt("a prompt")
        .input_reference(models::CreateVideoMultipartBodyInputReference::Text(
            upload.path(),
        ))
        .call()
        .await;
    let request = receive_one(captured, server);
    assert!(request.request_line.starts_with("POST /videos HTTP/1.1"));
    let parts = parts(&request);
    assert_eq!(named(&parts, "input_reference").len(), 1);
    assert_file_part(
        named(&parts, "input_reference")[0],
        "input_reference",
        "reference.png",
        PNG,
    );
    assert_eq!(named(&parts, "prompt")[0].body, b"a prompt");
}

#[tokio::test]
async fn create_video_sends_an_object_input_reference_as_a_json_part() {
    let (address, captured, server) = request_once();
    let configuration = config(&address);
    let reference: models::ImageRefParam2 =
        serde_json::from_value(serde_json::json!({"image_url": "https://example.test/a.png"}))
            .expect("image reference");
    let _ = videos_api::create_video()
        .configuration(&configuration)
        .prompt("a prompt")
        .input_reference(models::CreateVideoMultipartBodyInputReference::ImageRefParam2(reference))
        .call()
        .await;
    let request = receive_one(captured, server);
    let parts = parts(&request);
    let part = named(&parts, "input_reference")[0];
    assert_eq!(part.file_name(), None, "{part:?}");
    assert_eq!(part.content_type(), Some("application/json"), "{part:?}");
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&part.body).expect("JSON part"),
        serde_json::json!({"image_url": "https://example.test/a.png"})
    );
}

#[tokio::test]
async fn create_video_sends_string_variants_as_plain_text() {
    let (address, captured, server) = request_once();
    let configuration = config(&address);
    let _ = videos_api::create_video()
        .configuration(&configuration)
        .prompt("a prompt")
        .model(models::VideoModel::from("sora-custom"))
        .size(models::VideoSize::Variant720x1280)
        .call()
        .await;
    let request = receive_one(captured, server);
    let parts = parts(&request);
    let model = named(&parts, "model")[0];
    assert_eq!(model.body, b"sora-custom");
    assert_eq!(model.file_name(), None, "{model:?}");
    assert_eq!(model.content_type(), None, "{model:?}");
    assert_eq!(named(&parts, "size")[0].body, b"720x1280");
}

#[tokio::test]
async fn create_video_edit_uploads_the_video_path() {
    let upload = Upload::new("source.mp4", PNG);
    let (address, captured, server) = request_once();
    let configuration = config(&address);
    let _ = videos_api::create_video_edit()
        .configuration(&configuration)
        .video(models::CreateVideoEditMultipartBodyVideo::Text(
            upload.path(),
        ))
        .prompt("a prompt")
        .call()
        .await;
    let request = receive_one(captured, server);
    assert!(request
        .request_line
        .starts_with("POST /videos/edits HTTP/1.1"));
    let parts = parts(&request);
    assert_eq!(named(&parts, "video").len(), 1);
    assert_file_part(named(&parts, "video")[0], "video", "source.mp4", PNG);
}

#[tokio::test]
async fn create_video_edit_sends_a_video_reference_as_a_json_part() {
    let (address, captured, server) = request_once();
    let configuration = config(&address);
    let _ = videos_api::create_video_edit()
        .configuration(&configuration)
        .video(
            models::CreateVideoEditMultipartBodyVideo::VideoReferenceInputParam(
                models::VideoReferenceInputParam::new("video_123".to_string()),
            ),
        )
        .prompt("a prompt")
        .call()
        .await;
    let request = receive_one(captured, server);
    let parts = parts(&request);
    let part = named(&parts, "video")[0];
    assert_eq!(part.file_name(), None, "{part:?}");
    assert_eq!(part.content_type(), Some("application/json"), "{part:?}");
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&part.body).expect("JSON part"),
        serde_json::json!({"id": "video_123"})
    );
}

#[tokio::test]
async fn create_video_extend_uploads_the_video_path() {
    let upload = Upload::new("source.mp4", PNG);
    let (address, captured, server) = request_once();
    let configuration = config(&address);
    let _ = videos_api::create_video_extend()
        .configuration(&configuration)
        .video(models::CreateVideoExtendMultipartBodyVideo::Text(
            upload.path(),
        ))
        .prompt("a prompt")
        .seconds(models::VideoSeconds::Variant4)
        .call()
        .await;
    let request = receive_one(captured, server);
    assert!(request
        .request_line
        .starts_with("POST /videos/extensions HTTP/1.1"));
    let parts = parts(&request);
    assert_file_part(named(&parts, "video")[0], "video", "source.mp4", PNG);
    assert_eq!(named(&parts, "seconds")[0].body, b"4");
}

#[tokio::test]
async fn create_skill_uploads_a_single_zip_as_files() {
    let upload = Upload::new("skill.zip", ZIP);
    let (address, captured, server) = request_once();
    let configuration = config(&address);
    let _ = skills_api::create_skill()
        .configuration(&configuration)
        .files(models::CreateSkillBodyFiles::Text(upload.path()))
        .call()
        .await;
    let request = receive_one(captured, server);
    assert!(request.request_line.starts_with("POST /skills HTTP/1.1"));
    let parts = parts(&request);
    assert_eq!(parts.len(), 1, "{parts:?}");
    assert_file_part(&parts[0], "files", "skill.zip", ZIP);
}

/// CreateSkillBody documents a directory upload as "multipart `files[]`".
#[tokio::test]
async fn create_skill_sends_one_files_brackets_part_per_file_of_an_array() {
    let first = Upload::new("SKILL.md", PNG);
    let second = Upload::new("helper.py", ZIP);
    let (address, captured, server) = request_once();
    let configuration = config(&address);
    let _ = skills_api::create_skill()
        .configuration(&configuration)
        .files(models::CreateSkillBodyFiles::ArrayOfStrings(vec![
            first.path(),
            second.path(),
        ]))
        .call()
        .await;
    let request = receive_one(captured, server);
    let parts = parts(&request);
    assert_eq!(parts.len(), 2, "{parts:?}");
    assert_file_part(&parts[0], "files[]", "SKILL.md", PNG);
    assert_file_part(&parts[1], "files[]", "helper.py", ZIP);
}

#[tokio::test]
async fn create_skill_version_uploads_a_single_zip_as_files() {
    let upload = Upload::new("skill.zip", ZIP);
    let (address, captured, server) = request_once();
    let configuration = config(&address);
    let _ = skills_api::create_skill_version()
        .configuration(&configuration)
        .skill_id("skill_123")
        .files(models::CreateSkillVersionBodyFiles::Text(upload.path()))
        .default(true)
        .call()
        .await;
    let request = receive_one(captured, server);
    assert!(request
        .request_line
        .starts_with("POST /skills/skill_123/versions HTTP/1.1"));
    let parts = parts(&request);
    assert_file_part(named(&parts, "files")[0], "files", "skill.zip", ZIP);
    assert_eq!(named(&parts, "default")[0].body, b"true");
}

/// CreateSkillVersionBody documents no other name for the array than its property's.
#[tokio::test]
async fn create_skill_version_sends_one_files_part_per_file_of_an_array() {
    let first = Upload::new("SKILL.md", PNG);
    let second = Upload::new("helper.py", ZIP);
    let (address, captured, server) = request_once();
    let configuration = config(&address);
    let _ = skills_api::create_skill_version()
        .configuration(&configuration)
        .skill_id("skill_123")
        .files(models::CreateSkillVersionBodyFiles::ArrayOfStrings(vec![
            first.path(),
            second.path(),
        ]))
        .call()
        .await;
    let request = receive_one(captured, server);
    let parts = parts(&request);
    assert_eq!(parts.len(), 2, "{parts:?}");
    assert_file_part(&parts[0], "files", "SKILL.md", PNG);
    assert_file_part(&parts[1], "files", "helper.py", ZIP);
}

#[tokio::test]
async fn a_missing_file_is_an_error_not_a_panic_or_a_request() {
    let configuration = config_without_server();

    let error = videos_api::create_video()
        .configuration(&configuration)
        .prompt("a prompt")
        .input_reference(models::CreateVideoMultipartBodyInputReference::Text(
            a_missing_file(),
        ))
        .call()
        .await
        .expect_err("optional file");
    assert!(matches!(error, Error::Io(_)), "{error:?}");

    let error = videos_api::create_video_edit()
        .configuration(&configuration)
        .video(models::CreateVideoEditMultipartBodyVideo::Text(
            a_missing_file(),
        ))
        .prompt("a prompt")
        .call()
        .await
        .expect_err("required file");
    assert!(matches!(error, Error::Io(_)), "{error:?}");

    let present = Upload::new("SKILL.md", PNG);
    let error = skills_api::create_skill()
        .configuration(&configuration)
        .files(models::CreateSkillBodyFiles::ArrayOfStrings(vec![
            present.path(),
            a_missing_file(),
        ]))
        .call()
        .await
        .expect_err("array with a missing file");
    assert!(matches!(error, Error::Io(_)), "{error:?}");
}
