# Generated API review evidence

`api_change_report.py` captures read-only snapshots of an OpenAPI 3 input and
`src/apis/**/*.rs` / `src/models/**/*.rs`, then compares two snapshots. It needs
Python 3.10+ and PyYAML, the same dependency used by the generation scripts.
It never runs the generator, rustdoc, Cargo, Git, or a network command.

The report flags removed method/path operations and component schemas, renamed
operation IDs, changed declared required-field sets and enum wire values
(including inline/nested schemas), and added/removed/changed generated Rust files.
It preserves enum wire types and treats operation IDs separately from endpoint
identity. Additions also need review: an added Rust enum variant or struct field
can break a consumer even if upstream considers it additive.

This is **not complete Rust semver analysis**. The script does not parse Rust
signatures, exports, traits or features, resolve schema composition into effective
types, detect every wire-contract change, or prove downstream compatibility.
Generated file changes are review pointers, not classified Rust API changes.
Required fields and enums are compared at schema locations; composition branches
are sorted by content to ignore ordering, so a changed branch can produce several
conservative location differences. Empty results are not a compatibility guarantee.

## Capture and compare

Capture the baseline **before generation**, while both the checked-in spec and
generated files still represent that baseline. Use the exact baseline commit SHA
and its generator version. Capture the candidate after generation succeeds. The
source URL and generator version are caller-supplied provenance, not independently
verified claims; use the version actually selected for generation, and record any
uncertainty about historical generation separately. Prefer an immutable source URL
when available. The raw input SHA-256 identifies the bytes even for a moving URL.

For example, run from the repository root (substitute the real SHAs and versions):

```sh
mkdir -p target/reports/api-change
uv run --with pyyaml python scripts/api_change_report.py snapshot \
  --spec stainless.yaml --generated-root . \
  --source-url https://raw.githubusercontent.com/openai/openai-openapi/main/openapi.yaml \
  --generator-version 7.15.0 --revision BASELINE_COMMIT_SHA \
  --output target/reports/api-change/before.json

# Run the existing generation pipeline here, without changing its upstream input.
# Snapshot the resulting files, using the version selected for this run.
uv run --with pyyaml python scripts/api_change_report.py snapshot \
  --spec stainless.yaml --generated-root . \
  --source-url https://raw.githubusercontent.com/openai/openai-openapi/main/openapi.yaml \
  --generator-version 7.15.0 --revision CANDIDATE_COMMIT_OR_RUN_ID \
  --output target/reports/api-change/after.json

uv run --with pyyaml python scripts/api_change_report.py compare \
  --before target/reports/api-change/before.json \
  --after target/reports/api-change/after.json \
  --json-output target/reports/api-change/report.json \
  --markdown-output target/reports/api-change/report.md
```

For an uncommitted generated candidate, use an unambiguous run ID plus checkout
SHA as its revision and record the eventual PR head in the review. Do not call
the checkout SHA the generated candidate's commit. Historical comparisons can
use separate Git checkouts without regenerating either one. To compare patched
generator inputs as well as upstream inputs, capture a separate pair from the
actual before/after patched files; never substitute a historical artifact for
the current upstream download.

Snapshots store input SHA-256, source URL, declared generator version, revision,
the covered schema inventory, and every generated Rust file's SHA-256. Reports
include the baseline revision and canonical-JSON snapshot hashes. No timestamps
or absolute local paths are embedded. Equivalent object, required/enum array,
and composition branch reorderings produce no covered changes, though raw byte
hashes correctly differ. Missing generated directories, unresolved references,
external references, duplicate YAML keys and unsupported path-reference shapes
fail with exit code 2. No absent schema is fabricated or backfilled.

The checked-in fixture specifications are synthetic test inputs only. Snapshots
are review evidence, not generation caches or approved upstream substitutes.

## PR and release review integration

The Generate Client workflow captures `before.json` before running
`generate.sh`, then captures `after.json` after compilation, lint, tests and
formatting pass. It publishes `report.json` and `report.md`, places the Markdown
report in the Actions run summary, and links the run from the generated PR. The
four files are uploaded as the `api-change-report` artifact. The existing OA02
downstream compatibility check remains after generation and its own artifact
upload remains independent.

The generated PR no longer receives the `automerge` label and no workflow step
enables GitHub auto-merge. Before updating an older generated PR, the workflow
disables any existing auto-merge request and removes that label, which is
consumed by the genai-ci-bot workflow. With both auto-merge paths removed, a
human must merge the generated PR. The PR body requires explicit maintainer
or designated senior reviewer approval for a flagged removal; the workflow
creates flagged-removal PRs as drafts, which must be explicitly marked ready
before they can be merged. It applies that draft state to existing generated
PRs as well as new ones. The PR body records the accepted head, affected surface,
migration and version decision. The release-plz
PR body directs release reviewers to the generated-client PR and its report.
No branch protection or repository setting is changed.

The workflow integration:

1. Capture baseline evidence before fetching/generation overwrites files, then
   capture the candidate only after successful generation. If capture fails,
   stop; do not present an incomplete or stale report as clean.
2. Preserve all four files (`before.json`, `after.json`, `report.json`,
   `report.md`) as downloadable CI artifacts and put `report.md` in the job
   summary. Link the exact run/artifact from the generated PR and release PR
   review evidence. The release comparison baseline is the previous release;
   the latest nightly comparison alone may omit earlier release changes.
3. Read `review_required` and `removals_flagged` from `report.json`. An exit code
   of zero means successful reporting, **not compatibility approval**. A removal
   cannot auto-merge and requires explicit maintainer/designated senior reviewer
   approval. Other flagged changes also need review. Record reviewer identity,
   accepted head, migration and version decision; do not infer approval from a
   green generation run.
4. Preserve report links or artifacts with the release evidence before CI
   artifact retention expires. Keep `semver_check = false` and the existing
   manual review requirement; this report does not replace it.

## Validation

```sh
python3 -m unittest discover -s scripts/tests
```

With PyYAML installed, the suite exercises unchanged/reordered, additive and
breaking fixtures; nested/inline schemas; operation IDs; generated output
changes; unresolved references; input hashes; and deterministic, read-only CLI
execution with Cargo/rustdoc/Git absent from `PATH`.
