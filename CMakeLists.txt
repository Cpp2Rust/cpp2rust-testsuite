cmake_minimum_required(VERSION 3.20)
project(Cpp2Rust-TestSuite)

set(CPP2RUST_PROGRAMS woff2 brunsli)
set(CPP2RUST_MODELS   unsafe refcount)
set(CPP2RUST_SRCDIR   ${CMAKE_CURRENT_SOURCE_DIR}/../cpp2rust)
set(CPP2RUST_BINARY   ${CPP2RUST_SRCDIR}/build/cpp2rust/cpp2rust)

# -----------------------------------------------------------------------------

include(${CPP2RUST_SRCDIR}/cmake/rust-toolchain.cmake)

set(FILTER_SCRIPT ${CMAKE_CURRENT_SOURCE_DIR}/scripts/filter_compile_commands.py)

foreach(_prog IN LISTS CPP2RUST_PROGRAMS)
  include(${CMAKE_CURRENT_SOURCE_DIR}/cmake/${_prog}.cmake)

  set(_model_regen_targets "")
  set(_model_build_targets "")
  foreach(_model IN LISTS CPP2RUST_MODELS)
    ## FIXME: remove -Wstatic_mut_refs below when the generated code gets fixed
    add_custom_target("build-${_prog}-${_model}"
      COMMAND ${CMAKE_COMMAND} -E env "RUSTFLAGS=-Awarnings -Astatic_mut_refs" cargo +${RUST_STABLE_VERSION} build --release
      WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/${_prog}/out/${_model}"
    )

    list(APPEND _model_regen_targets "regen-${_prog}-${_model}")
    list(APPEND _model_build_targets "build-${_prog}-${_model}")
  endforeach()

  add_custom_target("build-${_prog}"
    DEPENDS "build-${_prog}-original" ${_model_build_targets}
  )

  add_custom_target("regen-${_prog}"
    DEPENDS "build-${_prog}-original" ${_model_regen_targets}
  )

  add_custom_target("check-${_prog}"
    COMMAND ${CMAKE_COMMAND} --build ${CMAKE_BINARY_DIR} --target "build-${_prog}" > /dev/null 2>&1
    COMMAND "${CMAKE_CURRENT_SOURCE_DIR}/scripts/${_prog}-test.sh"
            ${CMAKE_BINARY_DIR}
            ${CMAKE_CURRENT_SOURCE_DIR}
            ${CPP2RUST_MODELS}
    USES_TERMINAL
  )

  add_custom_target("benchmark-${_prog}"
    COMMAND ${CMAKE_COMMAND} --build ${CMAKE_BINARY_DIR} --target "build-${_prog}" > /dev/null 2>&1
    COMMAND python3
            "${CMAKE_CURRENT_SOURCE_DIR}/scripts/benchmark.py"
            --programs ${_prog}
            --csv "${CMAKE_BINARY_DIR}/bench_${_prog}.csv"
    USES_TERMINAL
  )
endforeach()

list(TRANSFORM CPP2RUST_PROGRAMS PREPEND "build-"     OUTPUT_VARIABLE _build_targets)
list(TRANSFORM CPP2RUST_PROGRAMS PREPEND "check-"     OUTPUT_VARIABLE _check_targets)
list(TRANSFORM CPP2RUST_PROGRAMS PREPEND "regen-"     OUTPUT_VARIABLE _regen_targets)

add_custom_target("build"     DEPENDS ${_build_targets})
add_custom_target("check"     DEPENDS ${_check_targets})
add_custom_target("regen"     DEPENDS ${_regen_targets})

add_custom_target("benchmark"
  COMMAND ${CMAKE_COMMAND} --build ${CMAKE_BINARY_DIR} --target build > /dev/null 2>&1
  COMMAND python3
          "${CMAKE_CURRENT_SOURCE_DIR}/scripts/benchmark.py"
          --csv "${CMAKE_BINARY_DIR}/bench.csv"
  USES_TERMINAL
)
