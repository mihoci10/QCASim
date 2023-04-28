collectSources(PROJECT_SRC src/QCASim)

FetchContent_Declare(
    Cherry
    GIT_REPOSITORY https://github.com/mihoci10/Cherry.git
    GIT_TAG        origin/master
)

FetchContent_MakeAvailable(Cherry)

add_executable(QCASim 
${PROJECT_SRC})

target_include_directories(QCASim PUBLIC src)

target_link_libraries(QCASim PUBLIC QCACore CherryAPI)

target_precompile_headers(QCASim PUBLIC src/qcaspch.h)

source_group("QCASim" FILES ${PROJECT_SRC})