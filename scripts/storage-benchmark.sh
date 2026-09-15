#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
require_volume

size_gib="${1:-4}"
[[ "$size_gib" =~ '^[0-9]+$' ]] || die "size must be an integer GiB"
test_file="$AI_ROOT/tmp/storage-benchmark-${size_gib}g.bin"
mkdir -p "$AI_ROOT/tmp" "$AI_ROOT/benchmarks"

print -- "Writing disposable ${size_gib} GiB benchmark file..."
write_start=$EPOCHREALTIME
dd if=/dev/zero of="$test_file" bs=8m count=$((size_gib * 128)) conv=fsync
write_end=$EPOCHREALTIME

print -- "Reading benchmark file..."
read_start=$EPOCHREALTIME
dd if="$test_file" of=/dev/null bs=8m
read_end=$EPOCHREALTIME

result="$AI_ROOT/benchmarks/storage-$(date -u +%Y%m%dT%H%M%SZ).json"
jq -n \
  --arg volume "$AI_VOLUME" \
  --arg device "$(df "$AI_VOLUME" | tail -1 | awk '{print $1}')" \
  --argjson bytes "$((size_gib * 1024 * 1024 * 1024))" \
  --argjson write_seconds "$((write_end - write_start))" \
  --argjson read_seconds "$((read_end - read_start))" \
  '{volume:$volume,device:$device,bytes:$bytes,write_seconds:$write_seconds,read_seconds:$read_seconds,write_MBps:($bytes/1000000/$write_seconds),read_MBps:($bytes/1000000/$read_seconds)}' | tee "$result"

rm -f "$test_file"
print -- "Saved $result and removed the disposable test file."
