//! What the mock answers: a [`Reply`], scripted per operation.

use std::convert::Infallible;
use std::time::Duration;

use axum::body::{Body, Bytes};
use axum::response::Response;
use futures_util::stream::{self, StreamExt};
use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use serde::Serialize;
use serde_json::{json, Value};

/// One scripted answer: a status, headers, a body and an optional delay.
///
/// A reply of status `200` (`201` for `create_realtime_call`) without extra headers goes through the generated
/// server's typed response: its JSON must decode as the operation's model, so a fixture that does not match the
/// spec fails the request (a `500` naming the operation and the decoding error) instead of passing silently. Any
/// other reply, and a [`Stream`] with wire options, is written as scripted.
#[derive(Clone, Debug)]
pub struct Reply {
    pub(crate) status: StatusCode,
    pub(crate) headers: HeaderMap,
    pub(crate) body: ReplyBody,
    pub(crate) delay: Option<Duration>,
}

#[derive(Clone, Debug)]
pub(crate) enum ReplyBody {
    Json(Value),
    Stream(Stream),
    Bytes { content_type: String, bytes: Bytes },
    Text { content_type: String, text: String },
    Empty,
}

impl Reply {
    fn new(status: StatusCode, body: ReplyBody) -> Self {
        Reply {
            status,
            headers: HeaderMap::new(),
            body,
            delay: None,
        }
    }

    /// `200` with this JSON body (a model, or a `serde_json::Value`; see [`fixtures`](crate::fixtures)).
    ///
    /// # Panics
    ///
    /// When `body` does not serialize.
    pub fn json(body: impl Serialize) -> Self {
        let value = serde_json::to_value(body).expect("the scripted body serializes");
        Reply::new(StatusCode::OK, ReplyBody::Json(value))
    }

    /// `200` with this event stream (`text/event-stream`).
    pub fn stream(stream: Stream) -> Self {
        Reply::new(StatusCode::OK, ReplyBody::Stream(stream))
    }

    /// `200` with these bytes, of this media type (a file's content, speech audio).
    pub fn bytes(content_type: &str, bytes: impl Into<Bytes>) -> Self {
        Reply::new(
            StatusCode::OK,
            ReplyBody::Bytes {
                content_type: content_type.to_owned(),
                bytes: bytes.into(),
            },
        )
    }

    /// `200` with this text (`text/plain`; a transcription in `text` format).
    pub fn text(text: impl Into<String>) -> Self {
        Reply::new(
            StatusCode::OK,
            ReplyBody::Text {
                content_type: "text/plain; charset=utf-8".to_owned(),
                text: text.into(),
            },
        )
    }

    /// `201` with this SDP answer (`application/sdp`): what `create_realtime_call` answers.
    pub fn sdp(answer: impl Into<String>) -> Self {
        let answer: String = answer.into();
        Reply::new(
            StatusCode::CREATED,
            ReplyBody::Bytes {
                content_type: "application/sdp".to_owned(),
                bytes: Bytes::from(answer),
            },
        )
    }

    /// No body, with this status.
    pub fn empty(status: u16) -> Self {
        Reply::new(status_code(status), ReplyBody::Empty)
    }

    /// An error in OpenAI's envelope, `{"error": {"message", "type", "param", "code"}}`, with this status. The
    /// `type` follows the status (`invalid_request_error`, `authentication_error`, `permission_error`,
    /// `rate_limit_error`, `server_error`); change it, or set `param` and `code`, with [`Reply::json`] and
    /// [`fixtures::error`](crate::fixtures::error).
    pub fn error(status: u16, message: impl Into<String>) -> Self {
        let status = status_code(status);
        Reply::new(
            status,
            ReplyBody::Json(crate::fixtures::error(
                error_type(status),
                message,
                None,
                None,
            )),
        )
    }

    /// `429` with `retry-after` (seconds, rounded up) and `retry-after-ms`, as OpenAI rate limits.
    pub fn rate_limited(retry_after: Duration) -> Self {
        let seconds = retry_after.as_millis().div_ceil(1000).max(1);
        Reply::error(429, "Rate limit reached. Please try again later.")
            .header("retry-after", &seconds.to_string())
            .header("retry-after-ms", &retry_after.as_millis().to_string())
    }

    /// The same body with this status.
    pub fn status(mut self, status: u16) -> Self {
        self.status = status_code(status);
        self
    }

    /// Adds a response header (`x-request-id`, `openai-processing-ms`, `x-ratelimit-remaining-requests`, ...).
    ///
    /// # Panics
    ///
    /// When `name` or `value` is not a valid header name or value.
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.append(
            HeaderName::try_from(name).expect("a valid header name"),
            HeaderValue::try_from(value).expect("a valid header value"),
        );
        self
    }

    /// Waits this long before answering: for timeouts and cancellation.
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    /// The JSON body, when the reply is a plain success the generated server can type.
    pub(crate) fn plain_json(&self, success: StatusCode) -> Option<Value> {
        match &self.body {
            ReplyBody::Json(value) if self.status == success && self.headers.is_empty() => {
                Some(value.clone())
            }
            _ => None,
        }
    }

    /// The events, when the reply is a plain stream the generated server can type.
    pub(crate) fn plain_stream(&self) -> Option<Vec<Value>> {
        match &self.body {
            ReplyBody::Stream(stream)
                if self.status == StatusCode::OK
                    && self.headers.is_empty()
                    && stream.is_plain() =>
            {
                Some(stream.events.clone())
            }
            _ => None,
        }
    }

    /// The bytes and their media type, when the reply is a plain success the generated server can type.
    pub(crate) fn plain_bytes(&self, success: StatusCode) -> Option<(Bytes, String)> {
        match &self.body {
            ReplyBody::Bytes {
                content_type,
                bytes,
            } if self.status == success && self.headers.is_empty() => {
                Some((bytes.clone(), content_type.clone()))
            }
            _ => None,
        }
    }

    /// The text, when the reply is a plain `200` the generated server can type.
    pub(crate) fn plain_text(&self) -> Option<String> {
        match &self.body {
            ReplyBody::Text { text, .. }
                if self.status == StatusCode::OK && self.headers.is_empty() =>
            {
                Some(text.clone())
            }
            _ => None,
        }
    }

    /// The reply as scripted, written by the mock instead of the generated server.
    pub(crate) fn into_response(self) -> Response {
        let (content_type, body) = match self.body {
            ReplyBody::Json(value) => (
                Some("application/json".to_owned()),
                Body::from(serde_json::to_vec(&value).unwrap_or_default()),
            ),
            ReplyBody::Stream(stream) => (Some("text/event-stream".to_owned()), stream.into_body()),
            ReplyBody::Bytes {
                content_type,
                bytes,
            } => (Some(content_type), Body::from(bytes)),
            ReplyBody::Text { content_type, text } => (Some(content_type), Body::from(text)),
            ReplyBody::Empty => (None, Body::empty()),
        };
        let mut response = Response::new(body);
        *response.status_mut() = self.status;
        if let Some(content_type) = content_type {
            if let Ok(value) = HeaderValue::try_from(content_type) {
                response
                    .headers_mut()
                    .insert(http::header::CONTENT_TYPE, value);
            }
        }
        response.headers_mut().extend(self.headers);
        response
    }
}

/// An event stream: the events, and how they are written. By default each event is a `data:` line (with an `event:`
/// line naming it by its `type`, when it has one, as OpenAI writes a Responses stream), and the stream ends with
/// `data: [DONE]`.
///
/// The wire options reproduce what clients meet in practice: keep-alive comments, unnamed events, an error after
/// the stream started, a stream cut before `[DONE]`, a slow stream.
#[derive(Clone, Debug)]
pub struct Stream {
    pub(crate) events: Vec<Value>,
    names: Option<bool>,
    keep_alive: bool,
    error: Option<(usize, StreamError)>,
    done: bool,
    interval: Option<Duration>,
}

/// An error sent after the stream started, as the stream's last event.
#[derive(Clone, Debug)]
pub enum StreamError {
    /// `data: {"error": {"message": ..., "type": "server_error", ...}}`, then `data: [DONE]`: how a chat completions
    /// stream fails.
    Data(String),
    /// `event: error` with `{"type": "error", "message": ..., "code": "server_error", "sequence_number": n}`: how a
    /// Responses stream fails.
    Event(String),
}

impl Stream {
    /// These events (models, or `serde_json::Value`s; see [`fixtures`](crate::fixtures)), ending with `[DONE]`.
    ///
    /// # Panics
    ///
    /// When an event does not serialize.
    pub fn new<T: Serialize>(events: impl IntoIterator<Item = T>) -> Self {
        Stream {
            events: events
                .into_iter()
                .map(|event| serde_json::to_value(event).expect("the scripted event serializes"))
                .collect(),
            names: None,
            keep_alive: false,
            error: None,
            done: true,
            interval: None,
        }
    }

    /// Writes an `event:` line naming every event by its `type` (`true`), or none (`false`). By default only events
    /// with a `type` are named.
    pub fn event_names(mut self, named: bool) -> Self {
        self.names = Some(named);
        self
    }

    /// Writes a `: keep-alive` comment before every event, as proxies and OpenAI do on a slow stream.
    pub fn keep_alive(mut self) -> Self {
        self.keep_alive = true;
        self
    }

    /// Ends the stream without `data: [DONE]`, as an interrupted stream does.
    pub fn without_done(mut self) -> Self {
        self.done = false;
        self
    }

    /// After `events` events, sends `error` and ends the stream.
    pub fn error_after(mut self, events: usize, error: StreamError) -> Self {
        self.error = Some((events, error));
        self
    }

    /// Waits this long before every event.
    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = Some(interval);
        self
    }

    /// What the generated server's typed event stream writes the same way.
    fn is_plain(&self) -> bool {
        self.names.is_none()
            && !self.keep_alive
            && self.error.is_none()
            && self.done
            && self.interval.is_none()
    }

    /// The frames of the stream, in order.
    fn frames(self) -> Vec<String> {
        let mut frames = Vec::new();
        let limit = self
            .error
            .as_ref()
            .map_or(self.events.len(), |(at, _)| (*at).min(self.events.len()));
        for event in self.events.into_iter().take(limit) {
            let mut frame = String::new();
            if self.keep_alive {
                frame.push_str(": keep-alive\n\n");
            }
            let name = event.get("type").and_then(Value::as_str).map(str::to_owned);
            match (self.names, name) {
                (Some(false), _) | (None, None) => {}
                (_, Some(name)) => frame.push_str(&format!("event: {name}\n")),
                (Some(true), None) => frame.push_str("event: message\n"),
            }
            frame.push_str(&format!("data: {event}\n\n"));
            frames.push(frame);
        }
        match self.error {
            Some((_, StreamError::Data(message))) => {
                let error = crate::fixtures::error("server_error", message, None, None);
                frames.push(format!("data: {error}\n\n"));
                if self.done {
                    frames.push("data: [DONE]\n\n".to_owned());
                }
            }
            Some((at, StreamError::Event(message))) => {
                let error = json!({
                    "type": "error",
                    "code": "server_error",
                    "message": message,
                    "param": null,
                    "sequence_number": at,
                });
                frames.push(format!("event: error\ndata: {error}\n\n"));
            }
            None if self.done => frames.push("data: [DONE]\n\n".to_owned()),
            None => {}
        }
        frames
    }

    fn into_body(self) -> Body {
        let interval = self.interval;
        let frames = stream::iter(self.frames()).then(move |frame| async move {
            if let Some(interval) = interval {
                tokio::time::sleep(interval).await;
            }
            Ok::<_, Infallible>(Bytes::from(frame))
        });
        Body::from_stream(frames)
    }
}

fn status_code(status: u16) -> StatusCode {
    StatusCode::from_u16(status).expect("a valid HTTP status")
}

fn error_type(status: StatusCode) -> &'static str {
    match status.as_u16() {
        401 => "authentication_error",
        403 => "permission_error",
        429 => "rate_limit_error",
        500..=599 => "server_error",
        _ => "invalid_request_error",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plain_stream_names_typed_events_and_ends_with_done() {
        let frames = Stream::new([json!({"type": "a", "n": 1}), json!({"n": 2})]).frames();
        assert_eq!(
            frames,
            [
                "event: a\ndata: {\"n\":1,\"type\":\"a\"}\n\n",
                "data: {\"n\":2}\n\n",
                "data: [DONE]\n\n"
            ]
        );
    }

    #[test]
    fn wire_options() {
        let frames = Stream::new([json!({"type": "a"}), json!({"type": "b"})])
            .event_names(false)
            .keep_alive()
            .error_after(1, StreamError::Data("boom".into()))
            .frames();
        assert_eq!(frames.len(), 3);
        assert_eq!(frames[0], ": keep-alive\n\ndata: {\"type\":\"a\"}\n\n");
        assert!(
            frames[1].starts_with("data: {\"error\":{\"code\":null,\"message\":\"boom\""),
            "{}",
            frames[1]
        );
        assert_eq!(frames[2], "data: [DONE]\n\n");

        let frames = Stream::new([json!({"type": "a"})])
            .error_after(1, StreamError::Event("boom".into()))
            .frames();
        assert!(
            frames[1].starts_with("event: error\ndata: {"),
            "{}",
            frames[1]
        );
        assert_eq!(frames.len(), 2);

        let frames = Stream::new([json!({"n": 1})]).without_done().frames();
        assert_eq!(frames, ["data: {\"n\":1}\n\n"]);
    }

    #[test]
    fn rate_limited_sets_retry_after() {
        let reply = Reply::rate_limited(Duration::from_millis(1500));
        assert_eq!(reply.status, StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(reply.headers["retry-after"], "2");
        assert_eq!(reply.headers["retry-after-ms"], "1500");
    }
}
