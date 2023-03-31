collectSources(PROJECT_SRC src)

FetchContent_Declare(
    googletest
    URL https://github.com/google/googletest/archive/refs/tags/v1.13.0.zip
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

add_executable(QCATest
${PROJECT_SRC})

target_link_libraries(QCATest PUBLIC 
QCACore 
GTest::gtest_main)

target_include_directories(QCATest PUBLIC src)

source_group("" FILES ${PROJECT_SRC})

include(GoogleTest)
gtest_discover_tests(QCATest)