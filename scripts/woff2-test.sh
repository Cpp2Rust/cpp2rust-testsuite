#!/bin/bash

set -eo pipefail

# Usage: woff2-test.sh <build_dir> <source_dir> <model...>
BUILD_DIR="$1"
SRC_DIR="$2/woff2"
shift 2
MODELS=("$@")

FONTS_DIR="$SRC_DIR/tests"
if [ -d /dev/shm ] && [ -w /dev/shm ]; then
  TMP_DIR=$(mktemp -d /dev/shm/cpp2rust-woff2.XXXXXX)
else
  TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/cpp2rust-woff2.XXXXXX")
fi
trap 'rm -fr "$TMP_DIR"' EXIT

# Wait for all PIDs, failing if any exited non-zero.
wait_all() {
  local pid fail=0
  for pid in "$@"; do
    wait "$pid" || fail=1
  done
  return $fail
}

mkdir -p "$TMP_DIR/original"

for f in "$FONTS_DIR"/**/*.ttf; do
  cp "$f" "$TMP_DIR/original"
done

# Run original binaries
pids=()
for f in "$TMP_DIR"/original/*.ttf; do
  "$SRC_DIR/src/woff2_compress" "$f" &> /dev/null &
  pids+=($!)
done
wait_all "${pids[@]}" || { echo "FAIL: cpp woff2_compress"; exit 1; }

n_ttf=$(ls "$TMP_DIR"/original/*.ttf | wc -l)
n_woff2=$(ls "$TMP_DIR"/original/*.woff2 | wc -l)
[ "$n_ttf" -eq "$n_woff2" ] || { echo "FAIL: cpp woff2_compress produced $n_woff2 of $n_ttf"; exit 1; }

for f in "$TMP_DIR"/original/*.woff2; do
  "$SRC_DIR/src/woff2_info" "$f" | tail -n +2 > "$TMP_DIR/original/$(basename "$f" .woff2).info"
done

mkdir -p "$TMP_DIR/cc-decompressed"
cp "$TMP_DIR"/original/*.woff2 "$TMP_DIR/cc-decompressed/"
pids=()
for f in "$TMP_DIR/cc-decompressed"/*.woff2; do
  "$SRC_DIR/src/woff2_decompress" "$f" &
  pids+=($!)
done
wait_all "${pids[@]}" || { echo "FAIL: cpp woff2_decompress"; exit 1; }

# Run each model and compare against original

for model in "${MODELS[@]}"; do
  RUST_BIN="$SRC_DIR/out/$model/target/release"
  MODEL_DIR="$TMP_DIR/$model"

  rm -fr "$MODEL_DIR"
  mkdir -p "$MODEL_DIR"

  cp "$TMP_DIR"/original/*.ttf "$MODEL_DIR"

  pids=()
  for f in "$MODEL_DIR"/*.ttf; do
    "$RUST_BIN"/woff2_compress "$f" &> /dev/null &
    pids+=($!)
  done
  wait_all "${pids[@]}" || { echo "FAIL [$model]: woff2_compress"; exit 1; }

  # The compressed bytes may differ from the original's, e.g., brotli encoders
  # need not produce the same output, so check that decompressing restores the
  # same ttf as the original roundtrip.
  rm -f "$MODEL_DIR"/*.ttf
  pids=()
  for f in "$MODEL_DIR"/*.woff2; do
    "$RUST_BIN"/woff2_decompress "$f" &
    pids+=($!)
  done
  wait_all "${pids[@]}" || { echo "FAIL [$model]: woff2_decompress"; exit 1; }

  for f in "$TMP_DIR"/cc-decompressed/*.ttf; do
    diff "$MODEL_DIR/$(basename "$f")" "$f" \
      || { echo "FAIL [$model]: ttf mismatch on $f"; exit 1; }
  done

  # Compare woff2_info output on the original woff2 files
  for f in "$TMP_DIR"/original/*.info; do
    base=$(basename "$f" .info)
    "$RUST_BIN"/woff2_info "$TMP_DIR/original/$base.woff2" | tail -n +2 | diff - "$f" \
      || { echo "FAIL [$model]: info mismatch on $base"; exit 1; }
  done

  echo "WOFF2 $model tests passed!"
done
