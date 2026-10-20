#!/usr/bin/env bash
# Check a pinned Git-source consumer without changing either repository. Usage:
#   scripts/check-downstream.sh [candidate-directory] [base-main-commit]
# Results survive checkout cleanup in DOWNSTREAM_RESULTS_DIR.
set -euo pipefail

candidate=$(cd "${1:-$(dirname "$0")/..}" && pwd -P)
baseline_sha=$(git -C "$candidate" rev-parse "${2:-origin/main}^{commit}")
candidate_sha=$(git -C "$candidate" rev-parse HEAD)
consumer_repository=${DOWNSTREAM_REPOSITORY:-https://github.com/genai-rs/openai-ergonomic.git}
# OA08 integration: wrapper 0.7 pins base 0.16 at an immutable Git revision.
consumer_sha=${DOWNSTREAM_COMMIT:-8ac6a6fe007eeb92754f64ecff40d63b505d6c50}
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
consumer_base_rev=UNKNOWN
candidate_diff=$(git -C "$candidate" diff --binary HEAD | shasum -a 256 | cut -d ' ' -f 1)

# Hash the complete checkout state, not just Git's status text. Porcelain does
# not change when an already-dirty file is edited again. Include every file,
# symlink, and directory outside .git (including ignored and untracked files),
# plus the exact index entries and Git status. Nothing in the source tree is
# silently excluded from this guard.
source_fingerprint() {
    python3 - "$1" <<'PYFINGERPRINT'
import hashlib
import os
from pathlib import Path
import subprocess
import sys

root = Path(sys.argv[1]).resolve()
digest = hashlib.sha256()
def add(data):
    digest.update(len(data).to_bytes(8, 'big'))
    digest.update(data)

status = subprocess.check_output(
    ['git', '-C', str(root), 'status', '--porcelain=v2', '-z',
     '--untracked-files=all', '--ignored=matching'])
index = subprocess.check_output(['git', '-C', str(root), 'ls-files', '--stage', '-z'])
add(status)
add(index)
for directory, names, files in os.walk(root, topdown=True, followlinks=False):
    names[:] = sorted(name for name in names if name != '.git')
    files.sort()
    parent = Path(directory)
    for name in names:
        path = parent / name
        if path.is_symlink():
            add(b'L' + os.fsencode(path.relative_to(root)))
            add(os.fsencode(os.readlink(path)))
    for name in files:
        path = parent / name
        relative = os.fsencode(path.relative_to(root))
        if path.is_symlink():
            add(b'L' + relative)
            add(os.fsencode(os.readlink(path)))
        elif path.is_file():
            add(b'F' + relative)
            add(path.stat().st_mode.to_bytes(4, 'big'))
            with path.open('rb') as source:
                while chunk := source.read(1024 * 1024):
                    add(chunk)
        else:
            add(b'O' + relative)
print(digest.hexdigest())
PYFINGERPRINT
}
source_fingerprint_before=$(source_fingerprint "$candidate")

finish() {
    local status=$?
    trap - EXIT
    local source_fingerprint_after=''
    local source_guard=PASS
    if ! source_fingerprint_after=$(source_fingerprint "$candidate"); then
        echo '::error::Could not fingerprint the candidate checkout after the downstream check.' >&2
        status=1
        candidate_result=FAIL
        source_guard=ERROR
    elif [[ "$source_fingerprint_after" != "$source_fingerprint_before" ]]; then
        echo '::error::Downstream check modified the candidate checkout.' >&2
        status=1
        candidate_result=FAIL
        source_guard=FAIL
    fi
    if ! rm -rf "$scratch"; then
        echo '::error::Could not remove the temporary downstream-check directory.' >&2
        status=1
    fi
    {
        echo '## Downstream compatibility'
        echo
        echo "- Consumer commit: \`$consumer_sha\`"
        echo "- Consumer base Git revision: \`$consumer_base_rev\`"
        echo "- Candidate source commit: \`$candidate_sha\` (working tree, including generated changes)"
        echo "- Candidate tracked diff SHA-256: \`$candidate_diff\`"
        echo "- Base-main commit: \`$baseline_sha\`"
        echo
        echo '| Consumer against | Result |'
        echo '| --- | --- |'
        echo "| Candidate | $candidate_result |"
        echo "| Base main (diagnostic only) | $baseline_result |"
        echo
        echo "Candidate failure blocks. Base-main failure is diagnostic. Candidate source-state guard: $source_guard. Exit status: $status."
        echo 'Full command output is in candidate.log and baseline.log.'
    } > "$results/summary.md"
    cat "$results/summary.md"
    if [[ -n "${GITHUB_STEP_SUMMARY:-}" ]]; then
        cat "$results/summary.md" >> "$GITHUB_STEP_SUMMARY"
    fi
    exit "$status"
}
trap finish EXIT

# Never enable live consumer tests, even when invoked from a credentialed shell.
unset OPENAI_API_KEY OPENAI_TEST_API_KEY AZURE_OPENAI_API_KEY
unset LANGFUSE_PUBLIC_KEY LANGFUSE_SECRET_KEY
export CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-2}
# Keep Cargo outputs and all patched consumer files outside the candidate.
export CARGO_TARGET_DIR="$scratch/target"

mkdir -p "$scratch/base-main" "$scratch/candidate-consumer" "$scratch/baseline-consumer"
git -C "$candidate" archive "$baseline_sha" | tar -x -C "$scratch/base-main"
git init -q "$scratch/consumer"
git -C "$scratch/consumer" fetch --depth=1 "$consumer_repository" "$consumer_sha"
git -C "$scratch/consumer" checkout -q --detach FETCH_HEAD
test "$(git -C "$scratch/consumer" rev-parse HEAD)" = "$consumer_sha"
consumer_base_rev=$(python3 - "$scratch/consumer/Cargo.toml" <<'PYREV'
import re
import sys
import tomllib
with open(sys.argv[1], 'rb') as manifest_file:
    manifest = tomllib.load(manifest_file)
dependency = manifest.get('dependencies', {}).get('openai-client-base')
if not isinstance(dependency, dict):
    sys.exit('ERROR: pinned consumer must declare openai-client-base as a Git dependency')
source = dependency.get('git')
revision = dependency.get('rev')
if source != 'https://github.com/genai-rs/openai-client-base' or not isinstance(revision, str) or not re.fullmatch(r'[0-9a-f]{40}', revision):
    sys.exit('ERROR: consumer base dependency must use the canonical Git URL and a full immutable revision')
print(revision)
PYREV
)
for variant in candidate baseline; do
    git -C "$scratch/consumer" archive HEAD | tar -x -C "$scratch/$variant-consumer"
done

check_consumer() (
    label=$1
    base=$2
    consumer="$scratch/$label-consumer"
    config="$scratch/$label.toml"
    cd "$consumer" || return
    # Derive the exact Git source from the pinned consumer manifest. Cargo
    # resolves in this temporary copy; no consumer lockfile is committed.
    python3 - "$consumer/Cargo.toml" "$base/Cargo.toml" > "$config" <<'PYCONFIG' || return
import json
import sys
import tomllib
with open(sys.argv[1], 'rb') as manifest_file:
    consumer = tomllib.load(manifest_file)
with open(sys.argv[2], 'rb') as base_file:
    base = tomllib.load(base_file)
dependency = consumer.get('dependencies', {}).get('openai-client-base')
if not isinstance(dependency, dict) or dependency.get('git') != 'https://github.com/genai-rs/openai-client-base':
    sys.exit('ERROR: consumer does not declare the expected Git dependency')
if not isinstance(dependency.get('rev'), str) or not __import__('re').fullmatch(r'[0-9a-f]{40}', dependency['rev']):
    sys.exit('ERROR: consumer Git dependency must use a full immutable revision')
if base.get('package', {}).get('name') != 'openai-client-base':
    sys.exit('ERROR: candidate path is not openai-client-base')
if base.get('package', {}).get('version') != '0.16.0':
    sys.exit('ERROR: expected accepted openai-client-base 0.16.0 source')
print('[patch.' + json.dumps(dependency['git']) + ']')
from pathlib import Path
print('openai-client-base = { path = ' + json.dumps(str(Path(sys.argv[2]).resolve().parent)) + ' }')
PYCONFIG
    echo '+ cargo metadata --format-version 1 --all-features'
    cargo --config "$config" metadata --format-version 1 --all-features > "$scratch/$label.json" || return
    # Inspect Cargo's actual resolved graph. A source assertion or matching
    # package name alone cannot prove the wrapper uses the local candidate.
    python3 - "$scratch/$label.json" "$consumer/Cargo.toml" "$base/Cargo.toml" <<'PYMETADATA' || return
import json
from pathlib import Path
import sys
import tomllib
metadata = json.loads(Path(sys.argv[1]).read_text())
with open(sys.argv[2], 'rb') as f:
    consumer = tomllib.load(f)
with open(sys.argv[3], 'rb') as f:
    base = tomllib.load(f)
expected = Path(sys.argv[3]).resolve()
packages = [p for p in metadata['packages'] if p['name'] == 'openai-client-base']
if len(packages) != 1 or Path(packages[0]['manifest_path']).resolve() != expected:
    sys.exit('ERROR: Cargo did not resolve exclusively to the requested local base manifest')
root = metadata['resolve']['root']
root_package = next((p for p in metadata['packages'] if p['id'] == root), None)
if root_package is None or root_package['name'] != 'openai-ergonomic':
    sys.exit('ERROR: resolved workspace root is not the pinned openai-ergonomic package')
root_node = next((n for n in metadata['resolve']['nodes'] if n['id'] == root), None)
base_id = packages[0]['id']
if root_node is None or not any(d['name'] == 'openai_client_base' and d['pkg'] == base_id for d in root_node['deps']):
    sys.exit('ERROR: Cargo dependency graph does not link the wrapper directly to the candidate manifest')
if base['package']['version'] != consumer['dependencies']['openai-client-base']['version'].lstrip('='):
    sys.exit('ERROR: patched manifest version does not equal the consumer pin')
print('Verified Cargo edge: openai-ergonomic ->', expected)
PYMETADATA
    echo '+ cargo check --locked --all-targets --all-features'
    cargo --config "$config" check --locked --all-targets --all-features || return
    echo '+ cargo test --offline --locked --all-features'
    # Metadata fetched/resolved dependencies. Tests now run offline with the
    # same verified temporary lockfile and local Git-source patch.
    cargo --config "$config" test --offline --locked --all-features || return
)

if check_consumer candidate "$candidate" > "$results/candidate.log" 2>&1; then
    candidate_result=PASS
else
    candidate_result=FAIL
fi
cat "$results/candidate.log"
# Always run base main too, including after candidate failure, for attribution.
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
