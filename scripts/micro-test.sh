#!/bin/bash

set -e

# Usage: micro-test.sh <build_dir> <source_dir> <model...>
BUILD_DIR="$1"
SRC_DIR="$2/micro"
shift 2
MODELS=("$@")

TMP_DIR="$BUILD_DIR/tmp-micro"
CPP_BIN_DIR="$SRC_DIR/src/build"

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

# Collect and run C++ reference binaries
pids=()
binaries=()

# Find all executable files in the C++ build directory
for bin_path in "$CPP_BIN_DIR"/*; do
  if [ -x "$bin_path" ] && [ -f "$bin_path" ]; then
    bin_name=$(basename "$bin_path")
    binaries+=("$bin_name")
    "$bin_path" &> "$TMP_DIR/cpp/$bin_name.out" &
    pids+=($!)
  fi
done

if [ ${#binaries[@]} -eq 0 ]; then
  echo "Error: No binaries found in $CPP_BIN_DIR"
  exit 1
fi

wait_all "${pids[@]}" || { echo "FAIL: C++ execution failed"; exit 1; }

# Run Model binaries and diff against C++ output
for MODEL in "${MODELS[@]}"; do
  mkdir -p "$TMP_DIR/$MODEL"
  pids=()

  # Run all matching binaries for this model in parallel
  for bin_name in "${binaries[@]}"; do
    model_bin="$SRC_DIR/out/$MODEL/target/release/$bin_name"
    "$model_bin" &> "$TMP_DIR/$MODEL/$bin_name.out" &
    pids+=($!)
  done

  wait_all "${pids[@]}" || { echo "FAIL: $MODEL execution failed"; exit 1; }

  for bin_name in "${binaries[@]}"; do
    cpp_out="$TMP_DIR/cpp/$bin_name.out"
    model_out="$TMP_DIR/$MODEL/$bin_name.out"
    diff "$cpp_out" "$model_out" \
      || { echo "FAIL [$MODEL]: output mismatch on $bin_name"; exit 1; }
  done

  echo "Micro $MODEL tests passed!"
done

rm -fr "$TMP_DIR"
