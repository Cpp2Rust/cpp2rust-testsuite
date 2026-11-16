set(BRUNSLI_SRC_DIR "${CMAKE_CURRENT_SOURCE_DIR}/brunsli/src")
set(BRUNSLI_OUT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/brunsli/out")

set(BRUNSLI_CBRUNSLI_FILTER c/common/ c/enc/ cbrunsli.cc !dbrunsli.cc !/enc/encode.cc !/dec/decode.cc !gtest !_deps "~!TEST_DATA_PATH")
set(BRUNSLI_DBRUNSLI_FILTER c/common/ c/dec/ dbrunsli.cc !cbrunsli.cc !/enc/encode.cc !/dec/decode.cc !jpeg_huffman_decode.cc !gtest !_deps "~!TEST_DATA_PATH")

add_custom_target("build-brunsli-original"
  COMMAND ${CMAKE_COMMAND} -S ${BRUNSLI_SRC_DIR}
                           -B ${BRUNSLI_SRC_DIR}/build
                           -G Ninja
                           -DCMAKE_C_COMPILER=${BENCH_CC}
                           -DCMAKE_CXX_COMPILER=${BENCH_CXX}
                           -DBUILD_TESTING=OFF
                           -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
  # Only the Makefile build links against the bundled Brotli
  # Important so the compressed files match perfectly
  COMMAND make CC=${BENCH_CC} CXX=${BENCH_CXX}
  WORKING_DIRECTORY "${BRUNSLI_SRC_DIR}"
)

add_custom_target("config-brunsli"
  # Filter: split compile_commands.json per binary
  COMMAND ${CMAKE_COMMAND} -E make_directory cbrunsli dbrunsli
  COMMAND python3 ${FILTER_SCRIPT} compile_commands.json cbrunsli/compile_commands.json
          ${BRUNSLI_CBRUNSLI_FILTER}
  COMMAND python3 ${FILTER_SCRIPT} compile_commands.json dbrunsli/compile_commands.json
          ${BRUNSLI_DBRUNSLI_FILTER}
  WORKING_DIRECTORY ${BRUNSLI_SRC_DIR}/build
  DEPENDS "build-brunsli-original"
)

foreach(_model IN LISTS CPP2RUST_MODELS)
  add_custom_target("regen-brunsli-${_model}"
    COMMAND ${CPP2RUST_BINARY} -dir ${BRUNSLI_SRC_DIR}/build/cbrunsli -model ${_model}
            -o ${BRUNSLI_OUT_DIR}/${_model}/src/bin/cbrunsli.rs
    COMMAND ${CPP2RUST_BINARY} -dir ${BRUNSLI_SRC_DIR}/build/dbrunsli -model ${_model}
            -o ${BRUNSLI_OUT_DIR}/${_model}/src/bin/dbrunsli.rs
    DEPENDS "config-brunsli"
  )
endforeach()
