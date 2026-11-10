# openai-client-base 0.27.0 → 0.28.0: typed `bucket_width`, nullable workflow state variables

This release generates with `timvw-rust` 0.22.0. The OpenAI spec (`stainless.yaml`) and openapi-generator (7.27.0) are
unchanged. The machine-readable census is `api-diff-0.27.0-to-0.28.0.json` in this folder (`compare_public_api.py`
against 0.27.0). The generator's own guide, [migrating from 0.21 to 0.22](https://github.com/timvw/openapi-generator-timvw-rust/blob/main/docs/guides/migration.md#from-021-to-022), shows every change with
the code before and after.

`timvw-rust` 0.22.0 does not change the generator configuration, so `scripts/generator-config.json` is unchanged.

## What stays the same

- Every model, field and enum except the one field below. No type is added, removed or renamed.
- Every operation keeps its module, function name and parameters, except `bucket_width` below.
- The JSON bodies sent and received, and the query string `organization_decisionsusage` sends.

## What changes

**`bucket_width` of `organization_decisionsusage` is typed.** The spec declares the parameter as
`anyOf: [{oneOf: [$ref: DecisionsUsageBucketWidth]}, {type: null}]`. It was `Option<&str>`, sent as the caller's text; it is now
`Option<models::DecisionsUsageBucketWidth>`, sent as the enum's value (`bucket_width=1d`). `None` sends nothing, as
before. Only this operation in `default_api` changes; the other usage operations (`usage_api`) declare `bucket_width`
as an inline `type: string` enum and keep `Option<&str>`.

**`ChatkitWorkflow.state_variables` values can be null.** The spec's map values are
`anyOf: [string, integer, boolean, number, null]`, so the field is
`Option<HashMap<String, Option<models::ChatkitWorkflowStateVariablesValue>>>` instead of
`Option<HashMap<String, models::ChatkitWorkflowStateVariablesValue>>`, and `ChatkitWorkflow::new` takes that type. A
response with `{"state_variables": {"k": null}}` failed to decode before; it decodes to `Some(None)` for that key now.

## Migrating

- `organization_decisionsusage`: `.bucket_width(models::DecisionsUsageBucketWidth::Variant1d)` instead of
  `.bucket_width("1d")`. A string can be parsed with `"1d".parse::<models::DecisionsUsageBucketWidth>()`.
- Code that reads `state_variables` handles the `Option` of each value (`.values().flatten()` skips the nulls); code
  that builds a `ChatkitWorkflow` wraps each value in `Some(..)`.
