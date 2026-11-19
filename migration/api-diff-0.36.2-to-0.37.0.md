# openai-client-base 0.36.2 → 0.37.0: a workspace of smaller crates

This release is generated as a Cargo workspace (timvw-rust `splitCrates`), so that the crate builds, and its docs
build, in a fraction of the memory: as one crate, rustc and rustdoc needed 11–12 GB, more than docs.rs's 6.4 GB, and
every crate depending on openai-client-base inherited the problem on docs.rs. The census is
`api-diff-0.36.2-to-0.37.0.json`, computed over the merged sources of the workspace (`scripts/merge_crate_sources.py`).

The version is a proposal: the release also takes the generator's 0.41.0 change below, which breaks webhook senders.

## What changes

`openai-client-base` is now a facade with no code of its own. Its `models` and `apis` modules re-export eleven
internal crates (`openai-client-base-common`, `-responses`, `-administration`, `-agents`, `-assistants`, `-chat`,
`-finetuning`, `-platform`, `-realtime`, `-tools`, `-untagged`), under the same paths and behind the same Cargo
features. A type is defined once, in one crate; the facade requires every crate at exactly its own version.

| | single crate (0.36.2) | workspace, largest crate |
| --- | --- | --- |
| `cargo check --all-features`, largest rustc process | 11.4 GB | 2.5 GB (`-responses`) |
| rustdoc, largest crate | 10.6 GB | 2.9 GB (`-responses`); the facade 3.2 GB |
| docs.rs-like build of the facade, all processes | 10.6 GB | 3.2 GB at `-j1`, 5.8 GB at `-j4` |

The census lists, besides an artefact of merging the sources (`apis/default_api.rs` `call`, whose split module the
merge concatenates in another order):

- the helpers the crates call across crate boundaries, now `pub` and `#[doc(hidden)]`: `models::union_dispatch`
  (`Tag`, `missing_tag`, `not_an_object`, `unknown_tag`, ...), `ParseEnumError::new`, `apis::ContentType`,
  `apis::multipart_helper` and `apis::api_response::header_value`. They are not part of the API: do not use them.

The generator release this is generated with includes timvw-rust 0.41.0, whose breaking change applies here too: a
webhook function (`apis::open_ai_client_base_webhooks_api::*`, re-exported by `apis::default_api`) posts to
`base_path` itself, the whole URL a receiver registered, instead of `{base_path}/{webhook name}`.

## Upgrading

Nothing to change for code that uses `openai_client_base::models` and `openai_client_base::apis`, with any features.
What differs:

- `std::any::type_name::<T>()` names the crate that defines the type
  (`openai_client_base_responses::models::response::Response`), as do the docs' source links and compiler notes.
- `Cargo.lock` and `cargo tree` list the twelve crates.
- Do not depend on the `openai-client-base-*` crates directly: which crate holds a model follows from the
  specification and changes with it.
- A webhook sender built on the generated webhook functions sets `base_path` to the receiver's full URL.
