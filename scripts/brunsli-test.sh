#!/bin/bash

set -e

# Usage: brunsli-test.sh <build_dir> <source_dir> <model...>
BUILD_DIR="$1"
SRC_DIR="$2/brunsli"
shift 2
MODELS=("$@")

IMGS_DIR="$SRC_DIR/tests"
TMP_DIR="$BUILD_DIR/tmp-brunsli"

# Wait for all PIDs, failing if any exited non-zero.
wait_all() {
  local pid fail=0
  for pid in "$@"; do
    wait "$pid" || fail=1
  done
  return $fail
}

rm -fr "$TMP_DIR"
mkdir -p "$TMP_DIR/cpp"

# Run original binaries
pids=()
for f in "$IMGS_DIR"/*.jpg; do
  "$SRC_DIR/src/bin/cbrunsli" "$f" "$TMP_DIR/cpp/$(basename "$f" .jpg).brn" &
  pids+=($!)
done
wait_all "${pids[@]}" || { echo "FAIL: cpp cbrunsli"; exit 1; }

pids=()
for f in "$TMP_DIR/cpp"/*.brn; do
  "$SRC_DIR/src/bin/dbrunsli" "$f" "$TMP_DIR/cpp/$(basename "$f" .brn).jpg" &
  pids+=($!)
done
wait_all "${pids[@]}" || { echo "FAIL: cpp dbrunsli"; exit 1; }

# check roundtrip lossless
for f in "$IMGS_DIR"/*.jpg; do
  diff "$f" "$TMP_DIR/cpp/$(basename "$f")" \
    || { echo "FAIL: cpp roundtrip not lossless on $f"; exit 1; }
done

# Run each model and compare against original

for model in "${MODELS[@]}"; do
  RUST_BIN="$SRC_DIR/out/$model/target/release"
  MODEL_DIR="$TMP_DIR/$model"

  rm -fr "$MODEL_DIR"
  mkdir -p "$MODEL_DIR"

  pids=()
  for f in "$IMGS_DIR"/*.jpg; do
    "$RUST_BIN"/cbrunsli "$f" "$MODEL_DIR/$(basename "$f" .jpg).brn" &
    pids+=($!)
  done
  wait_all "${pids[@]}" || { echo "FAIL [$model]: cbrunsli"; exit 1; }

  # Compare brn files against original
  for f in "$IMGS_DIR"/*.jpg; do
    base=$(basename "$f" .jpg)
    diff "$MODEL_DIR/$base.brn" "$TMP_DIR/cpp/$base.brn" \
      || { echo "FAIL [$model]: brn mismatch on $base"; exit 1; }
  done

  # Decompress and compare roundtrip
  pids=()
  for f in "$MODEL_DIR"/*.brn; do
    "$RUST_BIN"/dbrunsli "$f" "$MODEL_DIR/$(basename "$f" .brn).jpg" &
    pids+=($!)
  done
  wait_all "${pids[@]}" || { echo "FAIL [$model]: dbrunsli"; exit 1; }

  for f in "$IMGS_DIR"/*.jpg; do
    diff "$MODEL_DIR/$(basename "$f")" "$f" \
      || { echo "FAIL [$model]: jpg mismatch on $f"; exit 1; }
  done

  echo "Brunsli $model tests passed!"
done

rm -fr "$TMP_DIR"
