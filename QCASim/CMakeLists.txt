collectSources(PROJECT_SRC src/QCASim)
collectSources(PROJECT_SRC_UI src/QCASim/UI)
collectSources(PROJECT_SRC_UI_FRAMES src/QCASim/UI/Frames)
collectSources(PROJECT_SRC_INPUT src/QCASim/Input)
collectSources(PROJECT_SRC_DATA src/QCASim/Data)

FetchContent_Declare(
    Cherry
    GIT_REPOSITORY https://github.com/mihoci10/Cherry.git
    GIT_TAG        origin/master
)

FetchContent_MakeAvailable(Cherry)

set(PROJECT_RESOURCES
resources/Roboto-Bold.ttf
resources/Roboto-Italic.ttf
resources/Roboto-Regular.ttf
)


add_executable(QCASim
${PROJECT_SRC}
${PROJECT_SRC_UI}
${PROJECT_SRC_UI_FRAMES}
${PROJECT_SRC_INPUT}
${PROJECT_SRC_DATA})

target_include_directories(QCASim PUBLIC src)

target_link_libraries(QCASim PUBLIC QCACore CherryAPI)

target_precompile_headers(QCASim PUBLIC src/qcaspch.h)

add_custom_command(TARGET QCASim POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
    ${CMAKE_CURRENT_SOURCE_DIR}/resources
    ${PROJECT_BINARY_DIR}/resources
    COMMENT "Copying resources into binary dir"
)

source_group("QCASim" FILES ${PROJECT_SRC})
source_group("QCASim/UI" FILES ${PROJECT_SRC_UI})
source_group("QCASim/UI/Frames" FILES ${PROJECT_SRC_UI_FRAMES})
source_group("QCASim/Input" FILES ${PROJECT_SRC_INPUT})
source_group("QCASim/Data" FILES ${PROJECT_SRC_DATA})