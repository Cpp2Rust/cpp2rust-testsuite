# Cpp2Rust Test Suite

This repository contains open-source C/C++ programs that are known to be
automatically translatable by Cpp2Rust to Rust.

The CI bots of Cpp2Rust run these tests and update the expected output if
needed.

## Current set of included programs
 - [WOFF2](https://github.com/google/woff2): TTF font compression/decompression


## To build and run the benchmarks
```
mkdir build
cd build
cmake -GNinja ..
```

By default, cpp2rust is expected to be built in `../cpp2rust/build`. Use
`-DCPP2RUST_BUILD_DIR=<path>` to point to a different build directory.
The original C/C++ programs are compiled with the same clang used to build
cpp2rust.

Available commands:
 - `ninja build` (or just `ninja`) - Compile the original and Rust programs.
 - `ninja check` - Run the original and Rust binaries and compare the output.
 - `ninja benchmark` - Compare the performance of the original and the Rust
   binaries.
 - `ninja regen` - Regenerate the Rust code using Cpp2Rust.

Note: All commands can be appended with `-<prog>` to run for a specific program
only (e.g., `ninja build-woff2`).
