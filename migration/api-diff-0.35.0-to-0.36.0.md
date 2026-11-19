# openai-client-base 0.35 → 0.36.0: moderations answer with a union

This release generates with `timvw-rust` 0.38.1 (runtime 0.4, unchanged) from the OpenAI spec at
[`16cdf739`](https://github.com/openai/openai-openapi/blob/16cdf739aee9baa6191cdbab945dd5f7f9920f20/openapi.json)
(2026-11-19). The generator and its configuration are those of 0.35.0: regenerating 0.35.0 from its own spec gives
no difference. The census is `api-diff-0.35.0-to-0.36.0.json`.

## What changes

Breaking:

- **`moderations_api::create_moderation`** (and `create_moderation_response`) returns a
  `models::CreateModeration200Response` instead of a `models::CreateModerationResponse`. The spec's answer is now
  one of two shapes: `CreateModerationResponse(..)`, the classification as before, or
  `CreateCsamModerationResponse(..)`, the answer of the new `omni-moderation-csam-latest` model (`likely-csam` and
  `likely-csam/gen-ai` flags, scores and input types: `CsamModerationResult`, `CsamCategoryFlags`,
  `CsamCategoryScores`, `CsamCategoryAppliedInputTypes`). The union is untagged; it decodes by the categories, so an
  `omni-moderation-latest` answer is always the first variant.
- **`SessionTurnErrorCodeResource::SessionBudgetExceeded`** is removed; the spec replaces the code with
  `api_key_spend_limit_exceeded` (`ApiKeySpendLimitExceeded`). A `session_budget_exceeded` on the wire still
  decodes, as `Unknown`.
- **New public fields**, which break struct literals without `..Default::default()`:
  - `background_options: Option<BackgroundOptionsParam>` on `CreateResponse` and
    `ResponsesClientEventResponseCreate`, and `Option<BetaBackgroundOptionsParam>` on their `Beta*` twins;
  - `UpdateAgentSessionParams.vault_ids: Option<Option<Vec<String>>>` (omit to keep the session's vaults, `null` or
    `[]` to clear them).

Additive:

- **Background options** for the Responses API: `BackgroundOptionsParam` (`continuation_id`, `completion_window`)
  with `BackgroundCompletionWindowEnum` (`soonest`, `2h`, `8h`), and their `Beta*` twins.
- **Model ids** in the spec's lists: `gpt-6.2-astra`, `gpt-6.2-sol`, `gpt-6.2-astra-law`, and
  `gpt-4.1-mini-2025-04-14` for fine-tuning. These fields are strings in Rust; nothing changes in the types.
- Documentation comments.

Not changed in Rust: the spec drops the `enum` of a few `type`/`object` fields that keep their `const`
(`ImageEditCompletedEvent`, `ImageGenPartialImageEvent`, `Invite`, ...); the generated code is the same.

## Upgrading

<!-- not compiled: before/after fragments -->
```rust
// Before
let moderation = moderations_api::create_moderation().configuration(&config).create_moderation_request(&request).call().await?;
let flagged = moderation.results[0].flagged;

// After
use models::CreateModeration200Response as Moderation;
let flagged = match moderations_api::create_moderation().configuration(&config).create_moderation_request(&request).call().await? {
    Moderation::CreateModerationResponse(moderation) => moderation.results[0].flagged,
    Moderation::CreateCsamModerationResponse(moderation) => moderation.results[0].flagged,
};
```

Replace `SessionTurnErrorCodeResource::SessionBudgetExceeded` in `match`es with `ApiKeySpendLimitExceeded` (or the
wildcard arm), and add `..Default::default()` to struct literals of the models that gained a field.
