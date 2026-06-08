set(MICRO_SRC_DIR "${CMAKE_CURRENT_SOURCE_DIR}/micro/src")
set(MICRO_OUT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/micro/out")

file(GLOB MICRO_SRCS "${MICRO_SRC_DIR}/*.cpp")

set(micro_binaries)
foreach(_src_path IN LISTS MICRO_SRCS)
    get_filename_component(_bench_name "${_src_path}" NAME_WE)
    set(_bin_output "${MICRO_SRC_DIR}/build/${_bench_name}")
    list(APPEND micro_binaries "${_bin_output}")
    add_custom_command(
        OUTPUT "${_bin_output}"
        COMMAND ${CMAKE_COMMAND} -E make_directory "${MICRO_SRC_DIR}/build"
        COMMAND clang++ -O3 "${_src_path}" -o "${_bin_output}"
        DEPENDS "${_src_path}"
    )
endforeach()

add_custom_target("build-micro-original"
    DEPENDS ${micro_binaries}
)

foreach(_model IN LISTS CPP2RUST_MODELS)
    set(_translation_commands)
    foreach(_src_path IN LISTS MICRO_SRCS)
        get_filename_component(_bench_name "${_src_path}" NAME_WE)
        list(APPEND _translation_commands
            COMMAND ${CPP2RUST_BINARY}
                -file "${_src_path}"
                -model ${_model}
                -o "${MICRO_OUT_DIR}/${_model}/src/bin/${_bench_name}.rs"
        )
    endforeach()
    add_custom_target("regen-micro-${_model}"
        ${_translation_commands}
    )
endforeach()
