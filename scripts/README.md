# Generation regression checks

Run the Python patch tests with:

```sh
python3 -m unittest discover -s scripts/tests -v
```

`generate.sh` downloads the current Stainless specification and runs these
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
lint, tests, documentation, and formatting without updating the bot branch.
Publishing is restricted to `main`, either on schedule or with `publish: true`.

## Default implementation scope

The manual enum `Default` check in `fix_generated_code.py` inspects variants
within the enum that owns the implementation. Generated modules can contain
multiple enums: a non-defaultable payload in a neighboring enum must not remove
a valid default from an unrelated unit enum. Fixtures cover different enum
names, file creation orders, valid defaults, invalid defaults, and idempotence.

## CI documentation memory

Rustdoc's synthetic trait collection for the generated client exceeded 18 GiB
in a local MSRV run. `prepare_ci_memory.sh` provisions enough swap on ephemeral
Linux runners to bring RAM plus swap to 24 GiB, prints resource information,
and fails if allocation fails. CI keeps documentation checks on all three
toolchains and records their peak resident memory with `/usr/bin/time -v`.
