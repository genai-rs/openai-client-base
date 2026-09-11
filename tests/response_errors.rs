use openai_client_base::apis::{Error, ResponseContent};
use std::mem::size_of;

#[test]
fn error_size_is_independent_of_response_schema() {
    assert_eq!(size_of::<Error<()>>(), size_of::<Error<[u8; 4096]>>());
    assert!(size_of::<Error<[u8; 4096]>>() < 128);
    assert!(size_of::<Error<openai_client_base::apis::evals_api::DeleteEvalError>>() < 128);
}

#[test]
fn response_error_preserves_payload_and_display() {
    let response = ResponseContent {
        status: reqwest::StatusCode::BAD_REQUEST,
        content: String::from("original body"),
        entity: Some([7_u8; 4096]),
    };
    // Into also compiles before boxing, so the size regression fails on the
    // original implementation rather than merely failing to compile.
    let error = Error::ResponseError(response.into());
    assert_eq!(
        error.to_string(),
        "error in response: status code 400 Bad Request"
    );
    assert!(std::error::Error::source(&error).is_none());
    match error {
        Error::ResponseError(response) => {
            assert_eq!(response.status, reqwest::StatusCode::BAD_REQUEST);
            assert_eq!(response.content, "original body");
            assert_eq!(response.entity, Some([7_u8; 4096]));
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn unrelated_io_errors_keep_their_source() {
    let error: Error<()> = std::io::Error::other("disk failure").into();
    assert_eq!(error.to_string(), "error in IO: disk failure");
    assert_eq!(
        std::error::Error::source(&error).unwrap().to_string(),
        "disk failure"
    );
}

#[tokio::test]
async fn generated_endpoint_preserves_typed_and_unparseable_http_errors() {
    use openai_client_base::apis::{configuration::Configuration, evals_api};
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::time::{Duration, Instant};

    for body in [
        r#"{"code":null,"message":"missing eval","param":null,"type":"not_found"}"#,
        "upstream returned a non-JSON error",
    ] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(5);
            let mut stream = loop {
                match listener.accept() {
                    Ok((stream, _)) => break stream,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        assert!(Instant::now() < deadline, "client did not connect");
                        std::thread::sleep(Duration::from_millis(10));
                    }
                    Err(error) => panic!("accept failed: {error}"),
                }
            };
            stream.set_nonblocking(false).unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut request = Vec::new();
            let mut buffer = [0; 1024];
            while !request.windows(4).any(|part| part == b"\r\n\r\n") {
                let count = stream.read(&mut buffer).unwrap();
                assert_ne!(count, 0);
                request.extend_from_slice(&buffer[..count]);
            }
            assert!(request.starts_with(b"DELETE /evals/missing HTTP/1.1\r\n"));
            write!(stream,
                "HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(), body).unwrap();
        });
        let client = reqwest::Client::builder()
            .no_proxy()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        let configuration = Configuration {
            base_path: format!("http://{address}"),
            client: reqwest_middleware::ClientBuilder::new(client).build(),
            ..Configuration::default()
        };
        let result = evals_api::delete_eval()
            .configuration(&configuration)
            .eval_id("missing")
            .call()
            .await;
        server.join().unwrap();
        match result.unwrap_err() {
            Error::ResponseError(response) => {
                assert_eq!(response.status, reqwest::StatusCode::NOT_FOUND);
                assert_eq!(response.content, body);
                if body.starts_with('{') {
                    match response.entity.unwrap() {
                        evals_api::DeleteEvalError::Status404(entity) => {
                            assert_eq!(entity.message, "missing eval");
                            assert_eq!(entity.r#type, "not_found");
                            assert_eq!(entity.code, None);
                            assert_eq!(entity.param, None);
                        }
                        other => panic!("unexpected entity: {other:?}"),
                    }
                } else {
                    assert!(response.entity.is_none());
                }
            }
            other => panic!("unexpected error: {other}"),
        }
    }
}
