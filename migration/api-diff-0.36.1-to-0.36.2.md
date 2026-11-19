# openai-client-base 0.36.1 → 0.36.2: docs that docs.rs can build

This release is generated with `timvw-rust` 0.40.0 (runtime 0.4, the same OpenAI spec as 0.36.1). The census is
`api-diff-0.36.1-to-0.36.2.json`. Calls do not change: `Model::builder()...build()` and
`operation().configuration(&configuration)...call().await` compile as before, and a missing required member is still a
compile error.

## What changes

bon's finish functions required `S: <state module>::IsComplete`, a trait with one blanket impl per builder. That
bound made every one of those traits public API, so rustdoc tried each blanket impl on each struct and enum of the
crate (about 3,200 builders times 8,700 types). The docs took more than 40 minutes and 24 GB, and docs.rs, which allows
15 minutes, never built them.

- bon's own finish function is private now (`build_internal`, `call_internal`), and each builder gets a public
  `build()` (models, 2,369) or `call()` (API functions, 840 plus their `_response` twins), bounded by the state of each
  required member (`S::Model: IsSet`) instead of `IsComplete`. The census lists them as 2,411 added functions.
- The census also lists 2,369 "derive changes": each model struct now carries a `#[builder(...)]` attribute between
  its `#[derive(...)]` and the struct, which the census does not read. The derives are the same.

## Upgrading

Nothing to change, unless generic code of yours calls `build()` or `call()` on a builder whose state it bounds by
`IsComplete`: bound it by the required members instead (`S::Model: chat_completion_request_builder::IsSet`), as the
generated `build()` does. `IsComplete` itself still exists.
