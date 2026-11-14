# openai-client-base 0.33 → 0.34.0: typed Responses input and instructions

This release generates with `timvw-rust` 0.30.1 (runtime 0.4, unchanged) and the current OpenAI spec. The census
is `api-diff-0.33.0-to-0.34.0.json`. Until now, some fields of the Responses, Completions and Moderations APIs were
plain strings. This release types them as the spec describes, and it makes optional a few fields that the spec marks
required but that the API leaves out. A real `/responses` reply then decodes, and item input can be sent.

## What changes

- **`CreateResponse.input`**: `Option<String>` → `Option<models::InputParam>`
  (`TextInput(String)` | `InputItemList(Vec<InputItem>)`).
- **`Response.instructions`**: `String` → `Option<models::ResponseAllOfInstructions>` (text or a list of input
  items). `null`, which the API sends when there are no instructions, decodes as `None`. Before, it failed to decode.
- **`CreateCompletionRequest.prompt`**: `String` → `Option<models::CreateCompletionRequestPrompt>`
  (`Text` | `ArrayOfStrings` | `ArrayOfIntegers` | `ArrayOfArrays`).
- **`CreateModerationRequest.input`**: `String` → `models::CreateModerationRequestInput` (a string, a list of
  strings, or multimodal input).
- **Optional because the API leaves them out**, and no longer arguments of `new()`:
  - `logprobs`, `Vec<..>` → `Option<Vec<..>>`, in the output text and text events:
    - `OutputTextContent`;
    - `ResponseTextDeltaEvent` and `ResponseTextDoneEvent`;
    - their WebSocket twins `ResponseTextWsDelta`, `ResponseTextWsDone` and `ResponsesWebSocketStreamEvent`;
    - the `Beta*` counterparts of all of these.
  - `cache_write_tokens`, `i64` → `Option<i64>`, in `ResponseUsageInputTokensDetails`,
    `BetaResponseUsageInputTokensDetails` and `InputTokensDetailsResource`.
  - `Response.access_programs` and `BetaResponse.access_programs`: `Option<T>` → `Option<Option<T>>`, an optional
    field that can be `null`.
- **`ResponsesClientEventResponseCreate.input`** (the Responses WebSocket `response.create` event):
  `Option<String>` → `Option<models::InputParam>`, like `CreateResponse.input`.
- **`Unknown` variants**: the input-item models are now reachable from a response, so 46 of their enums and tagged
  unions gain an `Unknown` variant. A value the API adds later then decodes instead of failing. Exhaustive `match`es
  on them need a wildcard arm.
- **`BetaResponse.incomplete_details`** has its own type, `BetaResponseAllOfIncompleteDetails`, instead of sharing
  `ResponseAllOfIncompleteDetails`. The fields are the same.
- **Constructors**: the `new()` of each model above takes the new types, or no longer takes the fields that became
  optional (19 constructors; see the census).

## Upgrading

<!-- not compiled: before/after fragments -->
```rust
// Responses: text or item input
CreateResponse { input: Some(InputParam::TextInput("Hi".into())), ..Default::default() };
CreateResponse { input: Some(InputParam::InputItemList(items)), ..Default::default() };

// Completions and moderations
CreateCompletionRequest::new(model, Some(CreateCompletionRequestPrompt::Text(prompt)));
CreateModerationRequest::new(CreateModerationRequestInput::Text(input));

// Reading a response
if let Some(ResponseAllOfInstructions::Text(text)) = &response.instructions { /* ... */ }
let cache_writes = usage.input_tokens_details.cache_write_tokens.unwrap_or(0);
```

Add a `_ =>` arm to `match`es on the item enums that gained `Unknown`.
