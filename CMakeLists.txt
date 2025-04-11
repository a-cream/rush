cmake_minimum_required(VERSION 3.10)

set(PROJECT_NAME rush)

project(${PROJECT_NAME} C)

set(SRC_DIR ${CMAKE_CURRENT_SOURCE_DIR}/src)
set(INCLUDE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/include)

file(GLOB_RECURSE C_SOURCES "${SRC_DIR}/*.c")

add_executable(${PROJECT_NAME} ${C_SOURCES})

target_include_directories(${PROJECT_NAME} PRIVATE ${INCLUDE_DIR})

set(C_STANDARD_VERSION 11)
set_target_properties(${PROJECT_NAME} PROPERTIES
    C_STANDARD ${C_STANDARD_VERSION}
    C_STANDARD_REQUIRED ON
    C_EXTENSIONS OFF
)

target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra -Wpedantic)
