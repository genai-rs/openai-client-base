# Generation regression checks

Run the Python patch tests with:

```sh
uv run --with pyyaml python -m unittest discover -s scripts/tests -v
```

`generate.sh` downloads OpenAI's current published specification and validates
the raw and patched documents before running the generator. Missing `$ref`
targets stop generation and must be corrected upstream. It runs these
checks before applying the shared API response-error patch. Generated Rust is
limited to `src/apis` and `src/models` during cleanup; integration tests under
`tests` must survive regeneration. Run `cargo test --all-features` and
`cargo clippy --all-targets --all-features -- -D warnings` after generation.

## Response-error storage

`box_response_errors.py` boxes the shared `Error<T>::ResponseError` payload and
all generated API constructor expressions. The rule applies to every endpoint,
regardless of schema names or sizes. It leaves response schemas, JSON decoding,
status codes, raw bodies, and other error variants unchanged. Its transforms are
idempotent and reject unsupported generator shapes before writing files.

This fixes `clippy::result_large_err` when an endpoint's typed error response
makes the shared error enum large. The regression test uses a 4 KiB payload to
verify that error size stays constant as response schemas grow.

This changes the public variant from `ResponseError(ResponseContent<T>)` to
`ResponseError(Box<ResponseContent<T>>)`. Code constructing the variant must use
`Error::ResponseError(Box::new(response))` or `response.into()`. Field access in
match arms that bind the payload continues to work through dereferencing. Nested
struct patterns such as `Error::ResponseError(ResponseContent { status, .. })`
must first bind the box and then destructure `*response`. Moving the entire
response out also requires `*response`. Endpoint signatures and the `ResponseContent<T>`
fields stay the same. Each HTTP error response now allocates one box.

The Generate Client workflow defaults manual runs to validation only. Run it on
a review branch without `publish` to exercise downloading, generation, build,
lint, tests, and formatting without updating the bot branch.
Publishing is restricted to `main`, either on schedule or with `publish: true`.

## Default implementation scope

The manual enum `Default` check in `fix_generated_code.py` inspects variants
within the enum that owns the implementation. Generated modules can contain
multiple enums: a non-defaultable payload in a neighboring enum must not remove
a valid default from an unrelated unit enum. Fixtures cover different enum
names, file creation orders, valid defaults, invalid defaults, and idempotence.

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
