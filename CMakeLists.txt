cmake_minimum_required(VERSION 3.4)

project(QCASim VERSION 0.1.0
             DESCRIPTION "A tool for simulating a quantum cellular automata."
             LANGUAGES CXX)

include(FetchContent)

set (CMAKE_CXX_STANDARD 17)
set (CMAKE_CXX_STANDARD_REQUIRED ON)

set_property(GLOBAL PROPERTY USE_FOLDERS ON)

set(QCASIM_OUT_DIR ${CMAKE_CURRENT_BINARY_DIR}/out CACHE STRING "Set the output base directory")
set(QCASIM_BUILD_SHARED False CACHE BOOL "Build QCASim as a dynamic library")

# set output directories
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${QCASIM_OUT_DIR}/archive)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${QCASIM_OUT_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${QCASIM_OUT_DIR}/bin)

# auto-ignore build directory
if(NOT EXISTS ${PROJECT_BINARY_DIR}/.gitignore)
  file(WRITE ${PROJECT_BINARY_DIR}/.gitignore "*")
endif()

# detect platform
if(WIN32)
  set(QCASIM_PLATFORM_WIN True BOOL)
endif()
if(UNIX)
  set(QCASIM_PLATFORM_UNIX True BOOL)
endif()

function(defineBoolVariable varName)
  set(varValue "${${varName}}")
  if(varValue)
    add_compile_definitions(${varName})
  endif()
endfunction()

function(collectSources varName dir)
  file(GLOB SOURCES "${dir}/*.c" "${dir}/*.cpp" "${dir}/*.h" "${dir}/*.hpp")
  set("${varName}" "${SOURCES}" PARENT_SCOPE)
endfunction()

defineBoolVariable(QCASIM_PLATFORM_WIN)
defineBoolVariable(QCASIM_BUILD_SHARED)