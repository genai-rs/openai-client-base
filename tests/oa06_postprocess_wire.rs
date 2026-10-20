use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use openai_client_base::apis::{
    audio_api, configuration::Configuration, default_api, files_api, realtime_api,
};
use openai_client_base::models::{self, FileExpirationAfter};
use url::Url;

#[path = "../src/apis/multipart_helper.rs"]
mod multipart_helper_contract;

static TEMP_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

// Compile-time compatibility assertion for the public webhook import path.
#[allow(unused_imports)]
use openai_client_base::apis::default_api::agent_environment_expired_post;

struct CapturedRequest {
    request_line: String,
    headers: String,
    body: Vec<u8>,
}

struct MultipartPart {
    headers: String,
    body: Vec<u8>,
}

fn request_once(
    status: u16,
    response_content_type: &'static str,
    response_body: &'static str,
) -> (String, Receiver<CapturedRequest>, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback mock");
    listener
        .set_nonblocking(true)
        .expect("set nonblocking loopback listener");
    let address = listener.local_addr().expect("mock address").to_string();
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        let accept_deadline = Instant::now() + Duration::from_secs(10);
        let (mut stream, _) = loop {
            match listener.accept() {
                Ok(connection) => break connection,
                Err(error)
                    if error.kind() == std::io::ErrorKind::WouldBlock
                        && Instant::now() < accept_deadline =>
                {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => panic!("accept request before deadline: {error}"),
            }
        };
        stream
            .set_read_timeout(Some(Duration::from_secs(3)))
            .expect("set read timeout");
        let request = read_request(&mut stream);
        let reason = if status < 400 { "OK" } else { "Bad Request" };
        let response = format!(
            "HTTP/1.1 {status} {reason}\r\nContent-Type: {response_content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response_body}",
            response_body.len()
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
            let body_length = header_text
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().expect("content length"))
                })
                .unwrap_or(0);
            if bytes.len() >= end + 4 + body_length {
                break (end, body_length);
            }
        }
    };
    let header_text = String::from_utf8_lossy(&bytes[..header_end]).to_string();
    let request_line = header_text
        .lines()
        .next()
        .expect("request line")
        .to_string();
    let body = bytes[header_end + 4..header_end + 4 + content_length].to_vec();
    CapturedRequest {
        request_line,
        headers: header_text,
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

fn multipart_parts(request: &CapturedRequest) -> Vec<MultipartPart> {
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
        assert_eq!(request.body.get(start..start + 2), Some(&b"\r\n"[..]));
        start += 2;
        let headers_end =
            find_bytes(&request.body, b"\r\n\r\n", start).expect("part headers terminator");
        let headers = String::from_utf8_lossy(&request.body[start..headers_end]).to_string();
        let body_start = headers_end + 4;
        let next_marker = find_bytes(&request.body, &marker, body_start).expect("next boundary");
        let body_end = next_marker.saturating_sub(2);
        parts.push(MultipartPart {
            headers,
            body: request.body[body_start..body_end].to_vec(),
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
    configuration.bearer_access_token = Some("oa06-synthetic-token".to_string());
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(2))
        .timeout(Duration::from_secs(6))
        .build()
        .expect("build timed loopback HTTP client");
    configuration.client = reqwest_middleware::ClientBuilder::new(client).build();
    configuration
}

struct TempFileCleanup(PathBuf);

impl Drop for TempFileCleanup {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn temp_file(extension: &str, bytes: &[u8]) -> PathBuf {
    for _ in 0..100 {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let sequence = TEMP_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "oa06-postprocess-{}-{nonce}-{sequence}.{extension}",
            std::process::id()
        ));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut file) => {
                if let Err(error) = file.write_all(bytes) {
                    let _ = fs::remove_file(&path);
                    panic!("write uniquely owned upload fixture: {error}");
                }
                return path;
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => panic!("create uniquely owned upload fixture: {error}"),
        }
    }
    panic!("could not allocate a unique temporary upload fixture")
}

fn receive_one(captured: Receiver<CapturedRequest>, server: JoinHandle<()>) -> CapturedRequest {
    let result = captured.recv_timeout(Duration::from_secs(12));
    server
        .join()
        .expect("mock server thread completes and is reaped");
    result.expect("exactly one HTTP request captured before deadline")
}

fn container_list_response() -> &'static str {
    r#"{"object":"list","data":[{"id":"cntr_mock","object":"container","name":"fixture","created_at":1,"status":"active"}],"first_id":"cntr_mock","last_id":"cntr_mock","has_more":false}"#
}

#[tokio::test]
async fn restored_list_containers_sends_query_auth_and_decodes_response() {
    let (address, captured, server) =
        request_once(200, "application/json", container_list_response());
    let configuration = config(&address);
    let response_result = tokio::time::timeout(
        Duration::from_secs(8),
        default_api::list_containers()
            .configuration(&configuration)
            .limit(7)
            .order("desc")
            .after("after /?&")
            .name("batch run & 1")
            .call(),
    )
    .await;
    let request = receive_one(captured, server);
    let response = response_result
        .expect("list-containers request deadline")
        .expect("list containers response");

    assert!(request.request_line.starts_with("GET /containers?"));
    let target = request
        .request_line
        .split_whitespace()
        .nth(1)
        .expect("request target");
    let url = Url::parse(&format!("http://mock.local{target}")).expect("parse request URL");
    let query: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
    assert_eq!(query.get("limit").map(String::as_str), Some("7"));
    assert_eq!(query.get("order").map(String::as_str), Some("desc"));
    assert_eq!(query.get("after").map(String::as_str), Some("after /?&"));
    assert_eq!(query.get("name").map(String::as_str), Some("batch run & 1"));
    assert_eq!(
        header_value(&request.headers, "authorization"),
        Some("Bearer oa06-synthetic-token")
    );
    assert_eq!(response.first_id, "cntr_mock");
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].name, "fixture");
}

#[tokio::test]
async fn restored_list_containers_omits_absent_query_parameters() {
    let (address, captured, server) =
        request_once(200, "application/json", container_list_response());
    let configuration = config(&address);
    let response_result = tokio::time::timeout(
        Duration::from_secs(8),
        default_api::list_containers()
            .configuration(&configuration)
            .call(),
    )
    .await;
    let request = receive_one(captured, server);
    let _response = response_result
        .expect("list-containers request deadline")
        .expect("list containers response");
    assert_eq!(request.request_line, "GET /containers HTTP/1.1");
}

#[tokio::test]
async fn restored_list_containers_surfaces_non_success_response() {
    let (address, captured, server) = request_once(
        401,
        "application/json",
        r#"{"error":{"message":"synthetic unauthorized"}}"#,
    );
    let configuration = config(&address);
    let result = tokio::time::timeout(
        Duration::from_secs(8),
        default_api::list_containers()
            .configuration(&configuration)
            .call(),
    )
    .await;
    let request = receive_one(captured, server);
    assert!(result
        .expect("non-success list-containers request deadline")
        .is_err());
    assert_eq!(request.request_line, "GET /containers HTTP/1.1");
}

#[tokio::test]
async fn create_file_sends_file_object_fields_and_decodes_response() {
    let file_bytes = b"OA06-file\0fixture\xff\x80";
    let path = temp_file("bin", file_bytes);
    let (address, captured, server) = request_once(
        200,
        "application/json",
        r#"{"id":"file_mock","bytes":19,"created_at":1,"filename":"oa06","object":"file","purpose":"assistants","status":"processed"}"#,
    );
    let expires_after =
        FileExpirationAfter::new(models::file_expiration_after::Anchor::CreatedAt, 3600);
    let configuration = config(&address);
    let _cleanup = TempFileCleanup(path.clone());
    let response_result = tokio::time::timeout(
        Duration::from_secs(8),
        files_api::create_file()
            .configuration(&configuration)
            .file(path.clone())
            .purpose("assistants")
            .expires_after(expires_after)
            .call(),
    )
    .await;
    let request = receive_one(captured, server);
    let response = response_result
        .expect("create-file request deadline")
        .expect("file response");

    assert!(request.request_line.starts_with("POST /files HTTP/1.1"));
    assert_eq!(
        header_value(&request.headers, "authorization"),
        Some("Bearer oa06-synthetic-token")
    );
    let outer_type = header_value(&request.headers, "content-type").expect("outer content type");
    assert!(outer_type.starts_with("multipart/form-data; boundary="));
    let parts = multipart_parts(&request);
    let file = parts
        .iter()
        .find(|p| p.headers.contains("name=\"file\""))
        .expect("file part");
    assert!(file.headers.contains("filename=\"oa06-postprocess-"));
    assert_eq!(file.body, file_bytes);
    eprintln!(
        "file part content-type observed: {:?}",
        header_value(&file.headers, "content-type")
    );
    let purpose = parts
        .iter()
        .find(|p| p.headers.contains("name=\"purpose\""))
        .expect("purpose part");
    assert_eq!(String::from_utf8_lossy(&purpose.body), "assistants");
    let expires = parts
        .iter()
        .find(|p| p.headers.contains("name=\"expires_after\""))
        .expect("expiry part");
    assert_eq!(
        header_value(&expires.headers, "content-type"),
        Some("application/json")
    );
    let expiry: serde_json::Value =
        serde_json::from_slice(&expires.body).expect("expiry JSON object");
    assert_eq!(expiry["anchor"], "created_at");
    assert_eq!(expiry["seconds"], 3600);
    assert_eq!(response.id, "file_mock");
}

async fn transcription_request(
    timestamp_granularities: Option<Vec<String>>,
) -> (CapturedRequest, models::CreateTranscription200Response) {
    let file_bytes = b"OA06-audio\0fixture\xff\x81";
    let path = temp_file("wav", file_bytes);
    let (address, captured, server) = request_once(
        200,
        "application/json",
        r#"{"text":"mock transcript","task":"transcribe","duration":1.0,"segments":[],"language":"en"}"#,
    );
    let configuration = config(&address);
    let _cleanup = TempFileCleanup(path.clone());
    let response_result = match timestamp_granularities {
        Some(granularities) => {
            tokio::time::timeout(
                Duration::from_secs(8),
                audio_api::create_transcription()
                    .configuration(&configuration)
                    .file(path.clone())
                    .model("whisper-1")
                    .language("en")
                    .prompt("say hello")
                    .response_format(models::AudioResponseFormat::VerboseJson)
                    .timestamp_granularities(granularities)
                    .call(),
            )
            .await
        }
        None => {
            tokio::time::timeout(
                Duration::from_secs(8),
                audio_api::create_transcription()
                    .configuration(&configuration)
                    .file(path.clone())
                    .model("whisper-1")
                    .language("en")
                    .prompt("say hello")
                    .response_format(models::AudioResponseFormat::VerboseJson)
                    .call(),
            )
            .await
        }
    };
    let request = receive_one(captured, server);
    let response = response_result
        .expect("transcription request deadline")
        .expect("transcription response");

    assert!(request
        .request_line
        .starts_with("POST /audio/transcriptions HTTP/1.1"));
    assert_eq!(
        header_value(&request.headers, "authorization"),
        Some("Bearer oa06-synthetic-token")
    );
    assert!(header_value(&request.headers, "content-type")
        .unwrap()
        .starts_with("multipart/form-data; boundary="));
    let parts = multipart_parts(&request);
    let file = parts
        .iter()
        .find(|p| p.headers.contains("name=\"file\""))
        .expect("file part");
    assert!(file.headers.contains("filename=\"oa06-postprocess-"));
    assert_eq!(file.body, file_bytes);
    eprintln!(
        "transcription file part content-type observed: {:?}",
        header_value(&file.headers, "content-type")
    );
    let text = |name: &str| -> Vec<String> {
        parts
            .iter()
            .filter(|p| p.headers.contains(&format!("name=\"{name}\"")))
            .map(|p| String::from_utf8_lossy(&p.body).to_string())
            .collect()
    };
    assert_eq!(text("model"), vec!["whisper-1"]);
    assert_eq!(text("language"), vec!["en"]);
    assert_eq!(text("prompt"), vec!["say hello"]);
    assert_eq!(text("response_format"), vec!["verbose_json"]);
    eprintln!(
        "timestamp_granularities parts observed: {:?}",
        text("timestamp_granularities")
    );
    assert!(text("languages").is_empty());
    (request, response)
}

async fn transcription_chunking_request(
    chunking_strategy: models::CreateTranscriptionRequestChunkingStrategy,
) -> CapturedRequest {
    let path = temp_file("wav", b"OA06-chunking-fixture");
    let (address, captured, server) = request_once(
        200,
        "application/json",
        r#"{"text":"mock transcript","task":"transcribe","duration":1.0,"segments":[],"language":"en"}"#,
    );
    let configuration = config(&address);
    let _cleanup = TempFileCleanup(path.clone());
    let response_result = tokio::time::timeout(
        Duration::from_secs(8),
        audio_api::create_transcription()
            .configuration(&configuration)
            .file(path)
            .model("whisper-1")
            .response_format(models::AudioResponseFormat::VerboseJson)
            .chunking_strategy(chunking_strategy)
            .call(),
    )
    .await;
    let request = receive_one(captured, server);
    response_result
        .expect("chunking-strategy request deadline")
        .expect("chunking-strategy response");
    request
}

#[tokio::test]
async fn transcription_multipart_sends_declared_fields_and_decodes_response() {
    let (request, response) = transcription_request(None).await;
    assert!(request
        .request_line
        .starts_with("POST /audio/transcriptions HTTP/1.1"));
    assert_eq!(response.text, "mock transcript");
}

#[tokio::test]
async fn transcription_timestamp_granularities_use_repeated_multipart_names() {
    let (request, _response) =
        transcription_request(Some(vec!["word".to_string(), "segment".to_string()])).await;
    let observed: Vec<String> = multipart_parts(&request)
        .iter()
        .filter(|p| p.headers.contains("name=\"timestamp_granularities\""))
        .map(|p| String::from_utf8_lossy(&p.body).to_string())
        .collect();
    eprintln!("timestamp_granularities parts observed: {observed:?}");
    assert_eq!(observed, vec!["word", "segment"]);
}

#[tokio::test]
async fn transcription_timestamp_granularities_omits_empty_and_keeps_embedded_commas() {
    let (empty_request, _) = transcription_request(Some(Vec::new())).await;
    let empty_parts = multipart_parts(&empty_request);
    assert!(!empty_parts
        .iter()
        .any(|p| p.headers.contains("name=\"timestamp_granularities\"")));

    let (single_request, _) = transcription_request(Some(vec!["word".to_string()])).await;
    let single_parts: Vec<String> = multipart_parts(&single_request)
        .iter()
        .filter(|p| p.headers.contains("name=\"timestamp_granularities\""))
        .map(|p| String::from_utf8_lossy(&p.body).to_string())
        .collect();
    assert_eq!(single_parts, vec!["word"]);

    let (comma_request, _) = transcription_request(Some(vec!["word,segment".to_string()])).await;
    let comma_parts: Vec<String> = multipart_parts(&comma_request)
        .iter()
        .filter(|p| p.headers.contains("name=\"timestamp_granularities\""))
        .map(|p| String::from_utf8_lossy(&p.body).to_string())
        .collect();
    assert_eq!(comma_parts, vec!["word,segment"]);
}

#[tokio::test]
async fn transcription_chunking_strategy_sets_json_only_for_object_variant() {
    let object = models::CreateTranscriptionRequestChunkingStrategy::Vadconfig(
        models::VadConfig::new(models::vad_config::Type::ServerVad),
    );
    let object_request = transcription_chunking_request(object).await;
    let object_part = multipart_parts(&object_request)
        .into_iter()
        .find(|part| part.headers.contains("name=\"chunking_strategy\""))
        .expect("object chunking-strategy part");
    assert_eq!(
        header_value(&object_part.headers, "content-type"),
        Some("application/json")
    );
    let object_body: serde_json::Value =
        serde_json::from_slice(&object_part.body).expect("object chunking-strategy JSON");
    assert_eq!(object_body["type"], "server_vad");

    let scalar = models::CreateTranscriptionRequestChunkingStrategy::TextVariant(
        models::create_transcription_request_chunking_strategy::CreateTranscriptionRequestChunkingStrategyTextVariantEnum::Auto,
    );
    let scalar_request = transcription_chunking_request(scalar).await;
    let scalar_part = multipart_parts(&scalar_request)
        .into_iter()
        .find(|part| part.headers.contains("name=\"chunking_strategy\""))
        .expect("scalar chunking-strategy part");
    assert_eq!(header_value(&scalar_part.headers, "content-type"), None);
    assert_eq!(String::from_utf8_lossy(&scalar_part.body), "auto");
}

#[tokio::test]
async fn realtime_call_applies_sdp_and_session_multipart_media_types() {
    let (address, captured, server) =
        request_once(200, "application/json", r#""v=synthetic-answer""#);
    let configuration = config(&address);
    let session = models::RealtimeSessionCreateRequestGa::new(
        models::realtime_session_create_request_ga::Type::Realtime,
    );
    let response_result = tokio::time::timeout(
        Duration::from_secs(8),
        realtime_api::create_realtime_call()
            .configuration(&configuration)
            .sdp("v=synthetic-offer")
            .session(session)
            .call(),
    )
    .await;
    let request = receive_one(captured, server);
    let response = response_result
        .expect("realtime-call request deadline")
        .expect("realtime-call response");
    let parts = multipart_parts(&request);
    let sdp = parts
        .iter()
        .find(|part| part.headers.contains("name=\"sdp\""))
        .expect("SDP part");
    assert_eq!(
        header_value(&sdp.headers, "content-type"),
        Some("application/sdp")
    );
    assert_eq!(String::from_utf8_lossy(&sdp.body), "v=synthetic-offer");
    let session = parts
        .iter()
        .find(|part| part.headers.contains("name=\"session\""))
        .expect("session part");
    assert_eq!(
        header_value(&session.headers, "content-type"),
        Some("application/json")
    );
    let session_body: serde_json::Value =
        serde_json::from_slice(&session.body).expect("session JSON");
    assert_eq!(session_body, serde_json::json!({ "type": "realtime" }));
    assert_eq!(response, "v=synthetic-answer");
}

#[tokio::test]
async fn explicit_form_style_keeps_joined_array_text_and_ignores_content_type() {
    let (address, captured, server) = request_once(200, "application/json", "{}");
    let form = multipart_helper_contract::add_schema_value_to_form(
        reqwest::multipart::Form::new(),
        "values",
        serde_json::Value::String("word,segment".to_string()),
        None,
    )
    .expect("build schema-aware part with style precedence applied");
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(6))
        .build()
        .expect("build direct multipart client");
    let response = client
        .post(format!("http://{address}/fixture"))
        .multipart(form)
        .send()
        .await
        .expect("send form-style mock request");
    assert!(response.status().is_success());
    let request = receive_one(captured, server);
    let value = multipart_parts(&request)
        .into_iter()
        .find(|part| part.headers.contains("name=\"values\""))
        .expect("joined values part");
    assert_eq!(header_value(&value.headers, "content-type"), None);
    assert_eq!(String::from_utf8_lossy(&value.body), "word,segment");
}

#[tokio::test]
async fn unreadable_upload_file_fails_before_any_http_dispatch() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback mock");
    listener
        .set_nonblocking(true)
        .expect("nonblocking listener");
    let address = listener.local_addr().expect("mock address").to_string();
    let missing = std::env::temp_dir().join(format!("oa06-no-such-file-{}", std::process::id()));
    let _ = fs::remove_file(&missing);
    let configuration = config(&address);
    let result = tokio::time::timeout(
        Duration::from_secs(8),
        files_api::create_file()
            .configuration(&configuration)
            .file(missing)
            .purpose("assistants")
            .call(),
    )
    .await
    .expect("unreadable-file request deadline");
    assert!(result.is_err(), "missing local file should fail");
    match listener.accept() {
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
        Ok(_) => panic!("unreadable file unexpectedly dispatched an HTTP request"),
        Err(error) => panic!("unexpected listener error: {error}"),
    }
}
