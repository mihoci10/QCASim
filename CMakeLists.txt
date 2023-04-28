cmake_minimum_required(VERSION 3.4)

project(QCASim VERSION 0.1.0
             DESCRIPTION "A tool for simulating a quantum cellular automata."
             LANGUAGES CXX)

include(FetchContent)

set (CMAKE_CXX_STANDARD 17)
set (CMAKE_CXX_STANDARD_REQUIRED ON)

set_property(GLOBAL PROPERTY USE_FOLDERS ON)

set(QCAS_OUT_DIR ${CMAKE_CURRENT_BINARY_DIR}/out CACHE STRING "Set the output base directory")
set(QCAS_BUILD_SHARED False CACHE BOOL "Build QCASim as a dynamic library")

# set output directories
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${QCAS_OUT_DIR}/archive)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${QCAS_OUT_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${QCAS_OUT_DIR}/bin)

# auto-ignore build directory
if(NOT EXISTS ${PROJECT_BINARY_DIR}/.gitignore)
  file(WRITE ${PROJECT_BINARY_DIR}/.gitignore "*")
endif()

# detect platform
if(WIN32)
  set(QCAS_PLATFORM_WIN True BOOL)
endif()
if(UNIX)
  set(QCAS_PLATFORM_UNIX True BOOL)
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

defineBoolVariable(QCAS_PLATFORM_WIN)
defineBoolVariable(QCAS_BUILD_SHARED)

add_subdirectory(QCACore)
add_subdirectory(QCASim)
#add_subdirectory(QCAExample)

enable_testing()
add_subdirectory(QCATest)