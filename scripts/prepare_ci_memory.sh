#!/usr/bin/env bash
set -euo pipefail

# Rustdoc's synthetic trait collection for this generated client has exceeded
# 18 GiB locally. Give standard Linux runners 24 GiB of RAM + swap so the
# documentation check can finish without requiring a larger paid runner.
free -h
df -h "${RUNNER_TEMP:?RUNNER_TEMP must identify the ephemeral CI directory}"

available_kib=$(awk '/^(MemTotal|SwapTotal):/ { total += $2 } END { print total }' /proc/meminfo)
target_kib=$((24 * 1024 * 1024))
if (( available_kib < target_kib )); then
    needed_mib=$(((target_kib - available_kib + 1023) / 1024))
    swap_file="$RUNNER_TEMP/openai-client.swap"
    # Do not replace an existing file or silently continue without headroom.
    test ! -e "$swap_file"
    echo "Adding ${needed_mib} MiB of swap for generated documentation"
    sudo fallocate -l "${needed_mib}M" "$swap_file"
    sudo chmod 600 "$swap_file"
    sudo mkswap "$swap_file"
    sudo swapon "$swap_file"
fi

free -h
