//! Server-sent events: what a method returns for a `text/event-stream` response.
//!
//! [`EventStream`] decodes the `data:` payload of every event to the operation's event type. A union picks its
//! variant from the payload (its `type` field); the SSE `event:` name is not used. A `data: [DONE]` event or the end
//! of the body ends the stream. [`Events`] is the same stream undecoded, and [`Decoder`] the parser underneath, for
//! callers that read a body themselves.

use std::collections::VecDeque;
use std::fmt;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;
use serde::de::DeserializeOwned;

use super::Error;

/// The `data:` value that ends a stream.
pub const DONE: &str = "[DONE]";

/// One server-sent event, before its data is decoded.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Event {
    /// The `event:` field, when the event has one.
    pub event: Option<String>,
    /// The `data:` lines of the event, joined with `\n`.
    pub data: String,
    /// The last `id:` field seen on the stream, when there was one.
    pub id: Option<String>,
}

/// An incremental `text/event-stream` parser, following the HTML standard: lines end with CR, LF or CRLF, a line
/// starting with `:` is a comment, an empty line dispatches the event, and an event without `data:` is not
/// dispatched. Feed it the body as it arrives.
#[derive(Clone, Debug, Default)]
pub struct Decoder {
    buffer: Vec<u8>,
    after_cr: bool,
    started: bool,
    event: Option<String>,
    data: String,
    has_data: bool,
    id: Option<String>,
}

impl Decoder {
    /// A parser at the start of a stream.
    pub fn new() -> Self {
        Self::default()
    }

    /// Takes the next chunk of the body and appends the events it completes to `events`.
    pub fn feed(&mut self, chunk: &[u8], events: &mut VecDeque<Event>) {
        let mut chunk = chunk;
        if !self.started {
            // A UTF-8 byte order mark at the start of the stream is skipped.
            let pending = self.buffer.len() + chunk.len();
            let mut head = self.buffer.clone();
            head.extend_from_slice(&chunk[..chunk.len().min(3)]);
            if pending < 3 && b"\xEF\xBB\xBF".starts_with(&head) {
                self.buffer.extend_from_slice(chunk);
                return;
            }
            self.started = true;
            let mut all = std::mem::take(&mut self.buffer);
            all.extend_from_slice(chunk);
            let rest = all.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(&all).to_vec();
            self.feed_lines(&rest, events);
            return;
        }
        if self.after_cr {
            if let Some(rest) = chunk.strip_prefix(b"\n") {
                chunk = rest;
            }
            self.after_cr = false;
        }
        self.feed_lines(chunk, events);
    }

    fn feed_lines(&mut self, chunk: &[u8], events: &mut VecDeque<Event>) {
        let mut start = 0;
        let mut i = 0;
        while i < chunk.len() {
            match chunk[i] {
                b'\n' | b'\r' => {
                    let mut line = std::mem::take(&mut self.buffer);
                    line.extend_from_slice(&chunk[start..i]);
                    self.line(&line, events);
                    if chunk[i] == b'\r' {
                        if i + 1 < chunk.len() {
                            if chunk[i + 1] == b'\n' {
                                i += 1;
                            }
                        } else {
                            self.after_cr = true;
                        }
                    }
                    start = i + 1;
                }
                _ => {}
            }
            i += 1;
        }
        self.buffer.extend_from_slice(&chunk[start..]);
    }

    fn line(&mut self, line: &[u8], events: &mut VecDeque<Event>) {
        if line.is_empty() {
            if self.has_data {
                events.push_back(Event {
                    event: self.event.take(),
                    data: std::mem::take(&mut self.data),
                    id: self.id.clone(),
                });
            }
            self.event = None;
            self.data.clear();
            self.has_data = false;
            return;
        }
        if line[0] == b':' {
            return;
        }
        let line = String::from_utf8_lossy(line);
        let (field, value) = match line.find(':') {
            Some(colon) => {
                let value = &line[colon + 1..];
                (&line[..colon], value.strip_prefix(' ').unwrap_or(value))
            }
            None => (&line[..], ""),
        };
        match field {
            "data" => {
                if self.has_data {
                    self.data.push('\n');
                }
                self.data.push_str(value);
                self.has_data = true;
            }
            "event" => self.event = Some(value.to_owned()),
            "id" if !value.contains('\0') => self.id = Some(value.to_owned()),
            _ => {}
        }
    }

    /// The body ended: an event not completed by an empty line is discarded, as the standard requires.
    pub fn finish(&mut self) {
        self.buffer.clear();
        self.event = None;
        self.data.clear();
        self.has_data = false;
    }
}

/// The events of a `text/event-stream` body, as they come. A `data: [DONE]` event ends it, as does the end of the body
/// or a failure reading it (yielded as `Err` first); the body is dropped then, which frees the connection.
pub struct Events {
    bytes: Option<Body>,
    decoder: Decoder,
    ready: VecDeque<Event>,
}

type Chunk = Box<dyn AsRef<[u8]> + Send>;
type Body = Pin<Box<dyn Stream<Item = Result<Chunk, reqwest::Error>> + Send>>;

impl Events {
    /// The events of a response body. The status and the content type are not checked.
    pub fn from_response(response: reqwest::Response) -> Self {
        Self::from_bytes(response.bytes_stream())
    }

    /// The events of a body read as a stream of chunks.
    pub fn from_bytes<S, B>(bytes: S) -> Self
    where
        S: Stream<Item = Result<B, reqwest::Error>> + Send + 'static,
        B: AsRef<[u8]> + Send + 'static,
    {
        Self {
            bytes: Some(Box::pin(Chunks(Box::pin(bytes)))),
            decoder: Decoder::new(),
            ready: VecDeque::new(),
        }
    }
}

impl Stream for Events {
    type Item = Result<Event, reqwest::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = &mut *self;
        loop {
            if let Some(event) = this.ready.pop_front() {
                if event.data == DONE {
                    this.ready.clear();
                    this.bytes = None;
                    return Poll::Ready(None);
                }
                return Poll::Ready(Some(Ok(event)));
            }
            let Some(bytes) = this.bytes.as_mut() else {
                return Poll::Ready(None);
            };
            match bytes.as_mut().poll_next(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Some(Ok(chunk))) => {
                    this.decoder.feed((*chunk).as_ref(), &mut this.ready)
                }
                Poll::Ready(Some(Err(error))) => {
                    this.bytes = None;
                    this.decoder.finish();
                    return Poll::Ready(Some(Err(error)));
                }
                Poll::Ready(None) => {
                    this.bytes = None;
                    this.decoder.finish();
                }
            }
        }
    }
}

impl fmt::Debug for Events {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Events")
            .field("open", &self.bytes.is_some())
            .field("ready", &self.ready)
            .finish_non_exhaustive()
    }
}

/// A byte stream whose chunks are boxed, so that [`Events`] holds any body.
struct Chunks<S>(Pin<Box<S>>);

impl<S, B> Stream for Chunks<S>
where
    S: Stream<Item = Result<B, reqwest::Error>>,
    B: AsRef<[u8]> + Send + 'static,
{
    type Item = Result<Chunk, reqwest::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0
            .as_mut()
            .poll_next(cx)
            .map(|item| item.map(|chunk| chunk.map(|bytes| Box::new(bytes) as Chunk)))
    }
}

/// The events of a `text/event-stream` response, each `data:` payload decoded to `T`.
///
/// A payload that does not decode to `T` yields `Err(Error::Serde(_))`, and the stream goes on with the next event; an
/// event with an empty payload yields nothing. A failure reading the body yields `Err(Error::Reqwest(_))` and ends the
/// stream. `E` is the error type of the operation: the method checked the status before it returned the stream, so the
/// stream yields only these two errors. [`EventStream::into_events`] gives the events undecoded.
pub struct EventStream<T, E> {
    events: Events,
    types: PhantomData<fn() -> (T, E)>,
}

impl<T, E> EventStream<T, E> {
    /// The events of a response body. The status and the content type are not checked.
    pub fn from_response(response: reqwest::Response) -> Self {
        Self::from_events(Events::from_response(response))
    }

    /// The events of a body read as a stream of chunks.
    pub fn from_bytes<S, B>(bytes: S) -> Self
    where
        S: Stream<Item = Result<B, reqwest::Error>> + Send + 'static,
        B: AsRef<[u8]> + Send + 'static,
    {
        Self::from_events(Events::from_bytes(bytes))
    }

    /// Decodes these events.
    pub fn from_events(events: Events) -> Self {
        Self {
            events,
            types: PhantomData,
        }
    }

    /// The events not yet read, undecoded: their `event:` names, ids and raw `data:`.
    pub fn into_events(self) -> Events {
        self.events
    }
}

impl<T: DeserializeOwned, E> Stream for EventStream<T, E> {
    type Item = Result<T, Error<E>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match Pin::new(&mut self.events).poll_next(cx) {
                Poll::Ready(Some(Ok(event))) if event.data.trim().is_empty() => {}
                Poll::Ready(Some(Ok(event))) => {
                    return Poll::Ready(Some(
                        serde_json::from_str(&event.data).map_err(Error::from),
                    ));
                }
                Poll::Ready(Some(Err(error))) => return Poll::Ready(Some(Err(Error::from(error)))),
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

impl<T, E> fmt::Debug for EventStream<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventStream")
            .field("events", &self.events)
            .finish()
    }
}
