# openai-client-base 0.34 → 0.35.0: Unix timestamps are `jiff::Timestamp`s

This release generates with `timvw-rust` 0.38.1 (runtime 0.4, unchanged) and `dateLibrary=jiff`. Every model field
the spec marks `format: unixtime` is a [`jiff::Timestamp`](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html)
instead of an `i64` (or an `f64`, on `Response`), and `Model.shutdown_date` is a `jiff::civil::Date` instead of a
`String`. The JSON is unchanged: a number of seconds. The census is `api-diff-0.34.4-to-0.35.0.json`.

## What changes

- **174 fields** across 125 models: `i64` → `jiff::Timestamp`, `Option<i64>` → `Option<jiff::Timestamp>`,
  `Option<Option<i64>>` → `Option<Option<jiff::Timestamp>>`. They are listed [below](#the-fields).
- **`Response` and `BetaResponse`**: `created_at` (`f64`) and `completed_at` (`Option<Option<f64>>`) are timestamps
  too. The spec types them `number`: a fraction of a second is kept, and a whole second is written as an integer.
- **`Model.shutdown_date`**: `Option<Option<String>>` → `Option<Option<jiff::civil::Date>>` (`2026-03-31`).
- **115 constructors** (`new()`) take the new types.
- **`models::unix_time`** is new: the serde functions the fields use (`#[serde(with = "...")]`), public for types of
  your own that hold a Unix timestamp.
- **Stricter decoding.** A value that is no time in seconds fails to decode, with the value in the error:
  milliseconds are past the year 9999, the last a `Timestamp` holds. A number with a fraction (`1700000000.0`) is
  read. OpenAI and Ollama's OpenAI-compatible API both send whole seconds as integers in these fields.
- **New dependency:** `jiff`, with its `std` and `serde` features only.

Not changed: timestamp fields the spec gives no `format: unixtime` stay `i64` (the Agents, Vaults and Automations
resources' `created_at`, for example), as do array items and query parameters.

## Upgrading

<!-- not compiled: before/after fragments -->
```rust
// Before
let created: i64 = completion.created;
let age = now_seconds - file.created_at;
let model = Model::new("gpt-x".into(), 1_686_935_002, Object::Model, "openai".into());

// After
let created: jiff::Timestamp = completion.created;
let seconds: i64 = completion.created.as_second();
let age: jiff::SignedDuration = jiff::Timestamp::now().duration_since(file.created_at);
let model = Model::new("gpt-x".into(), jiff::Timestamp::from_second(1_686_935_002)?, Object::Model, "openai".into());
```

A timestamp prints as RFC 3339 (`2023-06-16T17:03:22Z`), orders and compares, and converts to a zoned time with
jiff's time zone features (`created.to_zoned(jiff::tz::TimeZone::UTC)`); enable those in your own crate.

## The fields

- `AdminApiKey`: `created_at`, `expires_at`, `last_used_at`
- `AdminApiKeyCreateResponse`: `created_at`, `expires_at`, `last_used_at`
- `AdminApiKeyOwner`: `created_at`
- `AssignedRoleDetails`: `created_at`, `updated_at`
- `AssistantMessageItem`: `created_at`
- `AssistantObject`: `created_at`
- `AuditLog`: `effective_at`
- `Batch`: `cancelled_at`, `cancelling_at`, `completed_at`, `created_at`, `expired_at`, `expires_at`, `failed_at`, `finalizing_at`, `in_progress_at`
- `BetaCompactResource`: `created_at`
- `BetaResponse`: `completed_at`, `created_at`
- `Certificate`: `created_at`
- `CertificateCertificateDetails`: `expires_at`, `valid_at`
- `ChatCompletionResponseMessageAudio`: `expires_at`
- `ChatCompletionStreamResponseDeltaAudio`: `expires_at`
- `ChatSessionResource`: `expires_at`
- `ClientToolCallItem`: `created_at`
- `CompactResource`: `created_at`
- `ContainerFileResource`: `created_at`
- `ContainerResource`: `created_at`, `last_active_at`
- `ConversationResource`: `created_at`
- `CreateChatCompletionResponse`: `created`
- `CreateChatCompletionStreamResponse`: `created`
- `CreateCompletionResponse`: `created`
- `Eval`: `created_at`
- `EvalRun`: `created_at`
- `EvalRunOutputItem`: `created_at`
- `ExternalStorageResponse`: `created_at`
- `FineTuningCheckpointPermission`: `created_at`
- `FineTuningJob`: `created_at`, `estimated_finish`, `finished_at`
- `FineTuningJobCheckpoint`: `created_at`
- `FineTuningJobEvent`: `created_at`
- `Group`: `created_at`
- `GroupResourceWithSuccess`: `created_at`
- `GroupResponse`: `created_at`
- `ImageEditCompletedEvent`: `created_at`
- `ImageEditPartialImageEvent`: `created_at`
- `ImageGenCompletedEvent`: `created_at`
- `ImageGenPartialImageEvent`: `created_at`
- `ImagesResponse`: `created`
- `Invite`: `accepted_at`, `created_at`, `expires_at`
- `MessageObject`: `completed_at`, `created_at`, `incomplete_at`
- `Model`: `created`, `shutdown_date`
- `OpenAiFile`: `created_at`, `expires_at`
- `OrganizationCertificate`: `created_at`
- `OrganizationCertificateCertificateDetails`: `expires_at`, `valid_at`
- `OrganizationProjectCertificate`: `created_at`
- `OrganizationProjectCertificateCertificateDetails`: `expires_at`, `valid_at`
- `Project`: `archived_at`, `created_at`
- `ProjectApiKey`: `created_at`, `expires_at`, `last_used_at`
- `ProjectApiKeyOwnerServiceAccount`: `created_at`
- `ProjectApiKeyOwnerUser`: `created_at`
- `ProjectGroup`: `created_at`
- `ProjectServiceAccount`: `created_at`
- `ProjectServiceAccountApiKey`: `created_at`, `expires_at`
- `ProjectServiceAccountCreateResponse`: `created_at`
- `ProjectUser`: `added_at`
- `ProvenanceResource`: `created_at`
- `RealtimeCreateClientSecretResponse`: `expires_at`
- `RealtimeSession`: `expires_at`
- `RealtimeSessionCreateRequestClientSecret`: `expires_at`
- `RealtimeSessionCreateResponse`: `expires_at`
- `RealtimeSessionCreateResponseGa`: `expires_at`
- `RealtimeTranscriptionSessionCreateResponseClientSecret`: `expires_at`
- `RealtimeTranscriptionSessionCreateResponseGa`: `expires_at`
- `RealtimeTranslationClientSecretCreateResponse`: `expires_at`
- `RealtimeTranslationSession`: `expires_at`
- `Response`: `completed_at`, `created_at`
- `RunObject`: `cancelled_at`, `completed_at`, `created_at`, `expires_at`, `failed_at`, `started_at`
- `RunStepObject`: `cancelled_at`, `completed_at`, `created_at`, `expired_at`, `failed_at`
- `SafetyAlertResource`: `created_at`
- `SafetyCaseResource`: `created_at`
- `ServiceAccountApiKeyBody`: `created_at`, `expires_at`
- `SkillResource`: `created_at`
- `SkillVersionResource`: `created_at`
- `TaskGroupItem`: `created_at`
- `TaskItem`: `created_at`
- `ThreadObject`: `created_at`
- `ThreadResource`: `created_at`
- `Upload`: `created_at`, `expires_at`
- `UploadPart`: `created_at`
- `User`: `added_at`, `api_key_last_used_at`, `created`
- `UserMessageItem`: `created_at`
- `UserUser`: `banned_at`
- `VectorStoreFileBatchObject`: `created_at`
- `VectorStoreFileObject`: `created_at`
- `VectorStoreObject`: `created_at`, `expires_at`, `last_active_at`
- `VideoCharacterResource`: `created_at`
- `VideoResource`: `completed_at`, `created_at`, `expires_at`
- `VoiceConsentResource`: `created_at`
- `VoiceResource`: `created_at`
- `WebhookAgentEnvironmentExpired`: `created_at`
- `WebhookAgentEnvironmentFailed`: `created_at`
- `WebhookAgentEnvironmentReady`: `created_at`
- `WebhookAgentEnvironmentSuspended`: `created_at`
- `WebhookAgentSessionActionRequired`: `created_at`
- `WebhookAgentSessionCreated`: `created_at`
- `WebhookAgentSessionEnvelope`: `created_at`
- `WebhookAgentSessionFailed`: `created_at`
- `WebhookAgentSessionIdle`: `created_at`
- `WebhookAgentSessionInProgress`: `created_at`
- `WebhookBatchCancelled`: `created_at`
- `WebhookBatchCompleted`: `created_at`
- `WebhookBatchExpired`: `created_at`
- `WebhookBatchFailed`: `created_at`
- `WebhookEndpointBody`: `created_at`, `updated_at`
- `WebhookEndpointWithSecretResource`: `created_at`, `updated_at`
- `WebhookEvalRunCanceled`: `created_at`
- `WebhookEvalRunFailed`: `created_at`
- `WebhookEvalRunSucceeded`: `created_at`
- `WebhookFineTuningJobCancelled`: `created_at`
- `WebhookFineTuningJobFailed`: `created_at`
- `WebhookFineTuningJobSucceeded`: `created_at`
- `WebhookLiveCallIncoming`: `created_at`
- `WebhookLiveTransportConnected`: `created_at`
- `WebhookLiveTransportIncoming`: `created_at`
- `WebhookRealtimeCallIncoming`: `created_at`
- `WebhookResponseCancelled`: `created_at`
- `WebhookResponseCompleted`: `created_at`
- `WebhookResponseFailed`: `created_at`
- `WebhookResponseIncomplete`: `created_at`
- `WebhookSafetyAlertCreated`: `created_at`
- `WebhookSafetyDeactivationIssued`: `created_at`
- `WebhookSafetyOrgAlertCreated`: `created_at`
- `WebhookSafetyWarningIssued`: `created_at`
- `WidgetMessageItem`: `created_at`
