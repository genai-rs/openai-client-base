#!/usr/bin/env bash
# Check a pinned consumer without changing either repository. Usage:
#   scripts/check-downstream.sh [candidate-directory] [base-main-commit]
# DOWNSTREAM_REPOSITORY / DOWNSTREAM_COMMIT also support local Git fixtures.
# Results survive temporary checkout cleanup in DOWNSTREAM_RESULTS_DIR.
set -euo pipefail

candidate=$(cd "${1:-$(dirname "$0")/..}" && pwd -P)
baseline_sha=$(git -C "$candidate" rev-parse "${2:-origin/main}^{commit}")
candidate_sha=$(git -C "$candidate" rev-parse HEAD)
consumer_repository=${DOWNSTREAM_REPOSITORY:-https://github.com/genai-rs/openai-ergonomic.git}
# Reviewed offline suite: live tests are ignored or require credentials.
consumer_sha=${DOWNSTREAM_COMMIT:-8717a622373d565d3d9511ba7150dca245b4fcfa}
if [[ ! "$consumer_sha" =~ ^[0-9a-f]{40}$ ]]; then
    echo "DOWNSTREAM_COMMIT must be a full commit SHA" >&2
    exit 1
fi
results=${DOWNSTREAM_RESULTS_DIR:-${RUNNER_TEMP:-${TMPDIR:-/tmp}}/downstream-results}
mkdir -p "$results"
results=$(cd "$results" && pwd -P)
scratch=$(mktemp -d "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/downstream.XXXXXX")
candidate_result=NOT_RUN
baseline_result=NOT_RUN

finish() {
    status=$?
    {
        echo '## Downstream compatibility'
        echo
        echo "- Consumer commit: \`$consumer_sha\`"
        echo "- Candidate source commit: \`$candidate_sha\` (working tree, including generated changes)"
        echo "- Candidate tracked diff SHA-256: \`$candidate_diff\`"
        echo "- Base-main commit: \`$baseline_sha\`"
        echo
        echo '| Consumer against | Result |'
        echo '| --- | --- |'
        echo "| Candidate | $candidate_result |"
        echo "| Base main (diagnostic only) | $baseline_result |"
        echo
        echo "Exit status: $status. Candidate failure always blocks; base-main failure is diagnostic."
        echo 'Full command output is in candidate.log and baseline.log.'
    } > "$results/summary.md"
    cat "$results/summary.md"
    if [[ -n "${GITHUB_STEP_SUMMARY:-}" ]]; then
        cat "$results/summary.md" >> "$GITHUB_STEP_SUMMARY"
    fi
    rm -rf "$scratch"
}
candidate_diff=$(git -C "$candidate" diff --binary HEAD | shasum -a 256 | cut -d ' ' -f 1)
trap finish EXIT

# Never enable live consumer tests, even when invoked from a credentialed shell.
unset OPENAI_API_KEY OPENAI_TEST_API_KEY AZURE_OPENAI_API_KEY
unset LANGFUSE_PUBLIC_KEY LANGFUSE_SECRET_KEY
export CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-2}
# Keep all Cargo outputs out of the candidate, including path dependency builds.
export CARGO_TARGET_DIR="$scratch/target"

mkdir -p "$scratch/base-main" "$scratch/candidate-consumer" "$scratch/baseline-consumer"
git -C "$candidate" archive "$baseline_sha" | tar -x -C "$scratch/base-main"
git init -q "$scratch/consumer"
git -C "$scratch/consumer" fetch --depth=1 "$consumer_repository" "$consumer_sha"
git -C "$scratch/consumer" checkout -q --detach FETCH_HEAD
test "$(git -C "$scratch/consumer" rev-parse HEAD)" = "$consumer_sha"
for variant in candidate baseline; do
    git -C "$scratch/consumer" archive HEAD | tar -x -C "$scratch/$variant-consumer"
done

check_consumer() (
    label=$1
    base=$2
    cd "$scratch/$label-consumer" || return
    # Absolute path in a disposable Cargo config; no consumer file is committed.
    python3 - "$base" > "$scratch/$label.toml" <<'PYCONFIG' || return
import json
import sys
print('[patch.crates-io]')
print('openai-client-base = { path = ' + json.dumps(sys.argv[1]) + ' }')
PYCONFIG
    config="$scratch/$label.toml"
    version=$(python3 - "$base/Cargo.toml" <<'PYVERSION'
import sys
import tomllib
with open(sys.argv[1], 'rb') as manifest:
    print(tomllib.load(manifest)['package']['version'])
PYVERSION
    ) || return
    # Refresh only the patched package; retain the consumer's other locked deps.
    cargo --config "$config" update -p openai-client-base --precise "$version" || return
    cargo --config "$config" metadata --format-version 1 --all-features > "$scratch/$label.json" || return
    # Cargo can silently ignore an incompatible patch. Never accept a registry
    # fallback as evidence that the candidate compiles.
    python3 - "$scratch/$label.json" "$base" <<'PYMETADATA' || return
import json
from pathlib import Path
import sys
metadata = json.loads(Path(sys.argv[1]).read_text())
packages = [p for p in metadata['packages'] if p['name'] == 'openai-client-base']
expected = (Path(sys.argv[2]) / 'Cargo.toml').resolve()
if len(packages) != 1 or Path(packages[0]['manifest_path']).resolve() != expected:
    sys.exit('ERROR: consumer did not resolve exclusively to the requested base path')
print('Verified consumer patch:', expected)
PYMETADATA
    echo '+ cargo check --all-targets --all-features'
    cargo --config "$config" check --all-targets --all-features || return
    echo '+ cargo test --offline --all-features'
    # --offline prevents Cargo downloads; ignored/gated live tests stay disabled.
    cargo --config "$config" test --offline --all-features || return
)

if check_consumer candidate "$candidate" > "$results/candidate.log" 2>&1; then
    candidate_result=PASS
else
    candidate_result=FAIL
fi
cat "$results/candidate.log"
# Always run main too, including after candidate failure, for attribution.
if check_consumer baseline "$scratch/base-main" > "$results/baseline.log" 2>&1; then
    baseline_result=PASS
else
    baseline_result=FAIL
fi
cat "$results/baseline.log"
if [[ "$candidate_result" != PASS ]]; then
    echo '::error::Downstream candidate compatibility failed; inspect candidate.log and baseline.log.' >&2
    exit 1
fi
if [[ "$baseline_result" != PASS ]]; then
    echo '::warning::Base-main consumer check failed; candidate passed. See baseline.log.' >&2
fi
