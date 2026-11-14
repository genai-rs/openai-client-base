#!/usr/bin/env bash
# Builds every Cargo feature on its own, with no other API feature: each API group with rustls, each `-types` feature
# alone, `types`, and no feature at all. The other CI jobs build only the default `full` set, where a missing
# #[cfg(feature = ...)] cannot show. Fails on any error or warning.
set -euo pipefail
cd "$(dirname "$0")/.."

features=$(python3 -I -c '
import tomllib
skip = {"default", "full", "_api", "rustls", "native-tls", "retry", "tracing"}
features = tomllib.load(open("Cargo.toml", "rb"))["features"]
print(" ".join(sorted(f for f in features if f not in skip)))
')

failed=()
check() {
    local label=$1
    shift
    local output
    if ! output=$(cargo check --no-default-features "$@" 2>&1) || grep -qE '^(warning|error)' <<<"$output"; then
        echo "::error::feature set '$label' does not build cleanly"
        echo "$output"
        failed+=("$label")
    else
        echo "ok: $label"
    fi
}

check "(none)"
for feature in $features; do
    case $feature in
        *-types | types) check "$feature" --features "$feature" ;;
        *) check "rustls,$feature" --features "rustls,$feature" ;;
    esac
done

if [ ${#failed[@]} -gt 0 ]; then
    echo "failing feature sets: ${failed[*]}"
    exit 1
fi
