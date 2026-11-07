# Generation regression checks

Run the Python patch tests with:

```sh
uv run --with pyyaml python -m unittest discover -s scripts/tests -v
```

`generate.sh` downloads OpenAI's current published specification and validates
it before running the generator. Missing `$ref` targets stop generation and
must be corrected upstream. Generated Rust is limited to `src/apis` and
`src/models` during cleanup; integration tests under `tests` must survive
regeneration. Run `cargo test --all-features` and
`cargo clippy --all-targets --all-features -- -D warnings` after generation.

## Response-error storage

The shared `Error<T>::ResponseError` payload is boxed by the generator
(`boxResponseError` in `generator-config.json`), in the enum and in every
generated constructor expression. It leaves response schemas, JSON decoding,
status codes, raw bodies, and other error variants unchanged.

This fixes `clippy::result_large_err` when an endpoint's typed error response
makes the shared error enum large; `tests/response_errors.rs` checks that
error size stays constant as response schemas grow.

The public variant is `ResponseError(Box<ResponseContent<T>>)`. Code
constructing the variant must use `Error::ResponseError(Box::new(response))` or
`response.into()`. Field access in match arms that bind the payload works
through dereferencing. Nested struct patterns such as
`Error::ResponseError(ResponseContent { status, .. })` must first bind the box
and then destructure `*response`. Moving the entire response out also requires
`*response`. Each HTTP error response allocates one box.

The Generate Client workflow defaults manual runs to validation only. Run it on
a review branch without `publish` to exercise downloading, generation, build,
lint, tests, and formatting without updating the bot branch.
Publishing is restricted to `main`, either on schedule or with `publish: true`.

## Model rewrites

None. Unions, `Default`, `Display` and imports are decided by the generator's
model-level rules (the options in `generator-config.json`). The transitional
`legacyModelRewrites` (Java ports of the scripts that used to run here) was
turned off in 0.26.0 and removed from timvw-rust 0.17.0.

## Documentation build

Run `cargo doc --no-deps --all-features` on a machine with sufficient memory
when checking the generated Rust documentation. Rustdoc's synthetic trait
collection for this large generated client exceeded 18 GiB locally. GitHub's
standard Linux runners repeatedly stopped during this step, even with swap,
before reaching tests, formatting, and clippy. CI therefore checks the client
with build, tests, formatting, and clippy; the generated Markdown documentation
remains part of the reproducible generation output.

## Release API compatibility

Release-plz also invokes rustdoc through `cargo-semver-checks` to generate JSON
for API comparison. The release workflow repeatedly stopped in that phase with
exit 143 and a runner shutdown signal, including [October 2](https://github.com/genai-rs/openai-client-base/actions/runs/37058999261)
and [October 3](https://github.com/genai-rs/openai-client-base/actions/runs/37108040310).
Those logs do not contain memory measurements or establish the exact shutdown
cause. A local reproduction with cargo-semver-checks 0.50.0 and one build job
exceeded 18 GiB RSS while rustdoc was generating the current client's JSON.
This exceeds the [16 GB RAM of standard public Linux runners](https://docs.github.com/en/actions/reference/runners/github-hosted-runners).

The package-specific `semver_check = false` in `release-plz.toml` keeps release
preparation from invoking this resource-intensive analysis. Version selection
still follows conventional commits, and publication still verifies the crate
build. Automatic API breaking-change detection is unavailable, so review API
changes and the proposed version before merging a release PR. Mark breaking
changes with `!` or a `BREAKING CHANGE:` footer, or adjust the release version
manually. The release PR body includes this review requirement.

On a machine with sufficient memory, install cargo-semver-checks and run
`CARGO_BUILD_JOBS=1 cargo semver-checks check-release --baseline-version 0.14.0`
(replace the baseline with the latest published version). Restore automated
checks when rustdoc's memory usage improves or a suitable runner is available.
To validate release preparation without publishing or updating a GitHub PR,
run `release-plz update` in a disposable clone; it updates only that clone's
version, lockfile, and changelog.
