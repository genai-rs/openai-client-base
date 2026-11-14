//! What an `<operation>_response` function returns (`responseFunctions`): the success with its status, its headers
//! and its body, still undecoded.
//!
//! Every operation's function decodes the whole body into the generated type and returns only that value. Its
//! `_response` twin sends the same request and fails the same way (an error status is still
//! [`Error::ResponseError`](super::Error::ResponseError) with the typed error entity), but hands out the success as
//! an [`ApiResponse`], so the caller chooses how much of the body to decode, and into what.

use std::fmt;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::Deserialize;

/// A success response with its body undecoded. `T` is the type the operation's own function decodes the body to.
///
/// - [`json`](Self::json): the body as `T`, what the operation's function returns.
/// - [`json_as`](Self::json_as): the body as a type of the caller's, often a subset of `T` with only the fields it
///   reads. serde skips every field the type does not name, so a subset allocates only what it keeps.
/// - [`json_borrowed`](Self::json_borrowed): the same, but the type may borrow strings from the body (`&str`,
///   `Cow<str>`), so it lives as long as the `ApiResponse`.
/// - [`text`](Self::text): the body as it arrived.
///
/// [`status`](Self::status) and [`headers`](Self::headers) are those of the response. Every method borrows it, so
/// the body can be decoded more than once, or in two ways. `ApiResponse<T>` is `Send`, `Sync` and `Clone` whatever
/// `T` is: it holds no `T`.
///
/// For a text success (`ApiResponse<String>`) use [`text`](Self::text): `json()` expects a JSON string literal
/// (`"..."`). For a success without content (`ApiResponse<()>`), `json()` expects `null`; use the status and headers.
///
/// ```
/// use std::borrow::Cow;
///
/// use openai_client_base::apis::ApiResponse;
/// use serde::Deserialize;
///
/// // What a `_response` function returns; built by hand here, it comes from the server in use.
/// let mut headers = timvw_rust_runtime::header::HeaderMap::new();
/// headers.insert("x-request-id", "req-1".parse().unwrap());
/// let response: ApiResponse<serde_json::Value> = ApiResponse::new(
///     timvw_rust_runtime::StatusCode::OK,
///     headers,
///     r#"{"id":"item-1","name":"first","tags":["a","b"]}"#.to_owned(),
/// );
///
/// // The whole body, as the operation's function decodes it.
/// let all: serde_json::Value = response.json()?;
/// assert_eq!(all["tags"][1], "b");
///
/// // Only the fields the caller reads, owned.
/// #[derive(Deserialize)]
/// struct Id {
///     id: String,
/// }
/// let id: Id = response.json_as()?;
/// assert_eq!(id.id, "item-1");
///
/// // Borrowed from the body: no copy of the string unless it has escapes.
/// #[derive(Deserialize)]
/// struct Name<'a> {
///     #[serde(borrow)]
///     name: Cow<'a, str>,
/// }
/// let name: Name = response.json_borrowed()?;
/// assert_eq!(name.name, "first");
///
/// assert_eq!(response.status, timvw_rust_runtime::StatusCode::OK);
/// assert_eq!(response.headers["x-request-id"], "req-1");
/// assert!(response.text().starts_with('{'));
/// # Ok::<(), serde_json::Error>(())
/// ```
pub struct ApiResponse<T> {
    /// The status of the response, a success (`2xx` or `3xx`).
    pub status: timvw_rust_runtime::StatusCode,
    /// The headers of the response.
    pub headers: timvw_rust_runtime::header::HeaderMap,
    body: String,
    _type: PhantomData<fn() -> T>,
}

impl<T> ApiResponse<T> {
    /// A response with this status, headers and body. The `_response` functions build it; a caller can too, for
    /// tests.
    pub fn new(
        status: timvw_rust_runtime::StatusCode,
        headers: timvw_rust_runtime::header::HeaderMap,
        body: String,
    ) -> Self {
        ApiResponse {
            status,
            headers,
            body,
            _type: PhantomData,
        }
    }

    /// The body, as it arrived.
    pub fn text(&self) -> &str {
        &self.body
    }

    /// Decodes the body as `U`, an owned type of the caller's: usually a subset of `T`, with only the fields the
    /// caller reads. Fields `U` does not name are skipped, not allocated.
    pub fn json_as<U: DeserializeOwned>(&self) -> Result<U, serde_json::Error> {
        serde_json::from_str(&self.body)
    }

    /// Decodes the body as `U`, which may borrow from it (`&'a str`, `Cow<'a, str>` with `#[serde(borrow)]`, or
    /// `&'a serde_json::value::RawValue`, which needs serde_json's `raw_value` feature in the caller's
    /// `Cargo.toml`). The value lives as long as this response.
    pub fn json_borrowed<'a, U: Deserialize<'a>>(&'a self) -> Result<U, serde_json::Error> {
        serde_json::from_str(&self.body)
    }

    /// The headers the spec declares for the operation's successes, typed: `H` is the operation's
    /// `<Operation>ResponseHeaders` struct in its API module, named because `T` does not tell the operation. Every field is an `Option`: `None` when the header is missing or does not
    /// parse. The same as `H::from(&response.headers)`.
    pub fn typed_headers<H>(&self) -> H
    where
        H: for<'h> From<&'h timvw_rust_runtime::header::HeaderMap>,
    {
        H::from(&self.headers)
    }
}

impl<T: DeserializeOwned> ApiResponse<T> {
    /// Decodes the body as `T`, the generated type, as the operation's own function does.
    pub fn json(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.body)
    }
}

impl<T> Clone for ApiResponse<T> {
    fn clone(&self) -> Self {
        ApiResponse {
            status: self.status,
            headers: self.headers.clone(),
            body: self.body.clone(),
            _type: PhantomData,
        }
    }
}

impl<T> fmt::Debug for ApiResponse<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiResponse")
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body", &self.body)
            .finish()
    }
}

/// The first value of the header `name`, trimmed, read by `parse`: `None` when it is missing, not UTF-8 or does
/// not parse. Used by the `<Operation>ResponseHeaders` and `<Operation>ErrorHeaders` structs.
pub(crate) fn header_value<T>(
    headers: &timvw_rust_runtime::header::HeaderMap,
    name: &str,
    parse: impl Fn(&str) -> Option<T>,
) -> Option<T> {
    let value = headers.get(name)?;
    parse(std::str::from_utf8(value.as_bytes()).ok()?.trim())
}
