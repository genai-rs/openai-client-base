# openai-client-base 0.32.0 → 0.33.0: timvw-rust 0.29.0, runtime 0.4

This release regenerates the client from the same spec with `timvw-rust` 0.29.0 (openapi-generator 7.27.0) and moves
to `timvw-rust-runtime` 0.4. The census is `api-diff-0.32.0-to-0.33.0.json`: no models or operations are added or
removed; one model field changes type, the multipart unions with a file variant hold uploads, and `apis::Error::Io` is
gone.

## What changes

**The multipart unions with a file variant hold `timvw_rust_runtime::Upload` (breaking for code that builds the
variant from a `PathBuf`).** `CreateVideoMultipartBodyInputReference`, `CreateVideoEditMultipartBodyVideo`,
`CreateVideoExtendMultipartBodyVideo`, `CreateSkillBodyFiles` and `CreateSkillVersionBodyFiles` hold an `Upload` (a
`Vec<Upload>` for an array of files) instead of a `PathBuf`, so they take bytes and streams too, on WebAssembly as
well. They gain `file(..)` (and `files(..)`) constructors and `From` conversions from `Upload` and `PathBuf`, and the
operation builders (`create_video`, `create_video_edit`, `create_video_extend`, `create_skill`,
`create_skill_version`) take anything that converts into the union:

```rust
use timvw_rust_runtime::Upload;
// a path, as before, without naming the variant
videos_api::create_video_edit().configuration(&configuration).video(PathBuf::from("clip.mp4")).prompt("...").call().await?;
// several files from memory
skills_api::create_skill()
    .configuration(&configuration)
    .files(models::CreateSkillBodyFiles::files([
        Upload::bytes(skill_md).file_name("SKILL.md"),
        Upload::bytes(script).file_name("run.py"),
    ]))
    .call()
    .await?;
```

The file variants are `#[serde(skip)]`: an upload has no JSON form. `Display` of a file variant prints the file name.

**`CreateContainerFileBody.file` is `Option<timvw_rust_runtime::Upload>`** (it was `Option<PathBuf>`). It is left out
of the model's JSON; the builder takes anything that converts into an `Upload`. The JSON operation
`create_container_file` never sent the file; `create_container_file_multipart` uploads it.

**`apis::Error::Io` and its `From<std::io::Error>` are removed.** Nothing produced them since 0.32: a file that cannot
be read is `Error::Http(e)` with `e.is_io()`.

**`RequestOptions::base_url`** (runtime 0.4) sends one call to another base URL: the generated functions declare
`Configuration.base_path` to the request, and `.options(&RequestOptions::new().base_url("https://...")?)` replaces it
for that call, keeping the operation's path and query. Per-call options now win over a `ProviderLayer`: it no longer
replaces a header or query parameter the options set, nor moves a call that has a base URL of its own.

**Cargo features.** `container-types`, `skill-types` and `video-types` now depend on `timvw-rust-runtime` (their models
hold uploads). `reqwest-middleware` is no longer part of the generator; this crate already sent through the runtime.

## Upgrading

- `Model::Text(path)` for the unions above: use `Model::file(path)` (or pass the `PathBuf` to the operation's builder);
  `ArrayOfStrings(vec![..])` becomes `Model::files([..])`. A `match` on the variant gets an `Upload`
  (`upload.name()` for its file name).
- Code that named `PathBuf` as `CreateContainerFileBody.file`'s type uses `Upload` (assigning `Some(path.into())`).
- `match` arms on `Error::Io` are removed; `Error::Http(e) if e.is_io()` matches a file that cannot be read.
- Depend on `timvw-rust-runtime` 0.4 if you name its types (`Upload`, `RequestOptions`, `Client`).
