# openai-client-base 0.26.0 → 0.27.0: request bodies by reference, headers on errors, validation

This release generates with `timvw-rust` 0.21.0. The OpenAI spec (`stainless.yaml`) and openapi-generator (7.27.0) are
unchanged. The machine-readable census is `api-diff-0.26.0-to-0.27.0.json` in this folder (`compare_public_api.py`
against 0.26.0).

`timvw-rust` 0.21.0 makes the renderings this crate already switched on (`boxOnlyCycles`, `defaultOnlyWhereValid`,
`enumFromStr`, `noStructDisplay`, `integersAsI64`, `qualifiedInlineNames`, `modelUnions`, `unionDispatch`, `openEnums`,
`schemaDefaultDocs`, `nullableUnionFields` and `boxResponseError`) the only rendering, so they are no longer listed in
`scripts/generator-config.json`. That part changes nothing in the crate.

## What stays the same

- Every model, field and enum. No type is added, removed or renamed (except the new `models::validation` module below).
- Every operation keeps its module and function name, and every parameter that is not a request body keeps its type
  (except the two vault `metadata` parameters below).
- The JSON bodies sent and received.

## What changes

**Request bodies are borrowed.** 164 functions take their JSON body by reference: `item: models::Item` becomes
`item: &models::Item`, an optional body `Option<&models::Item>`. This covers every method of an operation, the
`_stream` methods of `typedEventStreams` and the per-media-type methods of `contentTypeMethods`. Multipart and form
fields, files, strings, numbers and booleans are unchanged.

**Errors carry the response headers.** `apis::ResponseContent<T>` gets `headers: reqwest::header::HeaderMap`, filled on
every error status, so `retry-after`, `x-request-id` and the `x-ratelimit-*` headers can be read from
`Error::ResponseError(response)`. `{:?}` of an error now prints these headers; `Display` is unchanged.
`Error::ResponseError` already held `Box<ResponseContent<T>>` and still does.

**The error variant follows the status.** `entity` holds the variant of the response with the status's exact code, else
of its range, else `DefaultResponse`, decoded as that response's type; a body that does not decode as it gives
`UnknownValue`. Before, the body went into the first variant whose type accepted it. The error enums do not change.

**Vault `metadata` filters are typed.** `vaults_api::list_vaults` and `vaults_api::list_vault_credentials` take
`metadata: Option<HashMap<String, String>>` instead of `Option<&str>` and send it as `metadata[key]=value`, as the spec
declares (`style: deepObject`). The string was sent as one `metadata=<text>` parameter before.

**Audit log `effective_at` is sent as `effective_at[gt]=...`.** `audit_logs_api::list_audit_logs` keeps its signature;
the filter went out as flat `gt=...` keys before.

**Every model implements `models::validation::Validate`.** `validate()` checks the spec's `minLength`, `maxLength`,
`pattern`, `minimum`, `maximum`, `minItems`, `maxItems`, `uniqueItems`, ... recursively and returns the first
`models::validation::ValidationError` (path, rule, value). Nothing calls it for you. The crate now depends on `regex`.

**A nested union prints itself.** A union variant that holds another union (for example
`LiveResponsesDelegationSettingsInputParamToolChoice::OneOf`) is printed with that union's `Display` instead of its
JSON.

## Migrating

- Pass request bodies by reference: `.create_response(&request)`, `.create_embedding_request(&request)`,
  `.create_chat_completion_request(&request)` on the function's builder, and `&body` for an optional body's setter
  (`Some(&body)` where the `maybe_` setter is used). The value stays yours to reuse.
- Code that builds a `ResponseContent` with a struct literal adds `headers: reqwest::header::HeaderMap::new()`.
- Code that matches `Status…` variants of an error enum gets the variant of the actual status; check matches that relied
  on another status's body decoding.
- Vault listing: `.metadata(HashMap::from([("team".to_string(), "a".to_string())]))` instead of a string.
- To check a request before sending it: `use openai_client_base::models::validation::Validate;` then
  `request.validate()?`.
