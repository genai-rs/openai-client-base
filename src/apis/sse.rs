//! Server-sent events: what a method returns for a `text/event-stream` response.
//!
//! [`EventStream`] decodes the `data:` payload of every event to the operation's event type. A union picks its
//! variant from the payload (its `type` field); the SSE `event:` name is not used. A `data: [DONE]` event or the end
//! of the body ends the stream. [`Events`] is the same stream undecoded, and [`Decoder`] the parser underneath, for
//! callers that read a body themselves.

use std::fmt;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;
use serde::de::DeserializeOwned;

use super::Error;

pub use timvw_rust_runtime::sse::{Decoder, Event, Events, DONE};

/// The events of a `text/event-stream` response, each `data:` payload decoded to `T`.
///
/// A payload that does not decode to `T` yields `Err(Error::Serde(_))`, and the stream goes on with the next event; an
/// event with an empty payload yields nothing. A failure reading the body yields `Err(Error::Http(_))` and ends the
/// stream. `E` is the error type of the operation: the method checked the status before it returned the stream, so the
/// stream yields only these two errors. [`EventStream::into_events`] gives the events undecoded.
pub struct EventStream<T, E> {
    events: Events,
    types: PhantomData<fn() -> (T, E)>,
}

impl<T, E> EventStream<T, E> {
    /// The events of a response body. The status and the content type are not checked.
    pub fn from_response(response: timvw_rust_runtime::Response) -> Self {
        Self::from_events(Events::from_response(response))
    }

    /// The events of a body read as a stream of chunks.
    pub fn from_bytes<S, B>(bytes: S) -> Self
    where
        S: Stream<Item = Result<B, timvw_rust_runtime::Error>> + Send + 'static,
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
