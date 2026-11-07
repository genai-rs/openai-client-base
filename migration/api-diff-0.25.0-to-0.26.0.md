# openai-client-base 0.25.0 → 0.26.0: no more text rewrites after generation

This release generates with `timvw-rust` 0.17.0 and turns off `legacyModelRewrites`, the last piece of the old
post-processing scripts (ported to Java in Milestone 2.4). The generator itself now decides everything the crate
contains. The OpenAI spec and openapi-generator (7.27.0) are unchanged. The machine-readable census is
`api-diff-0.25.0-to-0.26.0.json` in this folder.

## What the rewrites still did

With every Milestone 3 option on, only two things were left:

- **Unused imports.** They removed `use crate::models;` from model files that refer to no other model, and imports of
  `ContentType`, `de::Error as _` and the file stream types from API modules that do not use them. The generator now
  writes these imports only where they are used. The generated code is the same, file for file.
- **Two `content` fields as `serde_json::Value`.** A leftover `sed` in the old `patch_generated.rs.sh` retyped
  `CreateMessageRequest.content` and `PredictionContent.content` from `String` to `serde_json::Value`. This crate
  already generates typed unions for both of them, but nothing used them. The fields now use them.

## What changes

Two fields and their constructors. Nothing else in the public API changes:

| Field | 0.25.0 | 0.26.0 |
| --- | --- | --- |
| `CreateMessageRequest.content` | `serde_json::Value` | `models::CreateMessageRequestContent` |
| `PredictionContent.content` | `serde_json::Value` | `models::PredictionContentContent` |

`CreateMessageRequest::new` and `PredictionContent::new` take the new types.

- `CreateMessageRequestContent` is `TextContent(String)` or `ArrayOfContentParts(Vec<CreateMessageRequestContentOneOfInner>)`,
  where a part is `MessageContentImageFileObject`, `MessageContentImageUrlObject` or `MessageRequestContentTextObject`.
- `PredictionContentContent` is `TextContent(String)` or `ArrayOfContentParts(Vec<ChatCompletionRequestMessageContentPartText>)`.

Both are untagged, so the JSON on the wire is what the spec allows and what a correct `Value` produced before: a string
or an array of parts. A `Value` of any other shape, which the API rejects, can no longer be built.

## Migrating

- Text: `CreateMessageRequest::new(role, "Hello".into())`, or `CreateMessageRequestContent::from(text)` or
  `::new_text(text)`. The same works for `PredictionContent`.
- Parts: `CreateMessageRequestContent::ArrayOfContentParts(vec![...])` or `new_arrayofcontentparts(parts)`.
- If you built the `Value` from JSON you already have, `serde_json::from_value::<CreateMessageRequestContent>(value)?`
  converts it.
- Reading the field: match on the two variants instead of inspecting a `Value`.
