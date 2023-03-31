collectSources(PROJECT_SRC src)

add_executable(QCAExample ${LIB_TYPE} 
${PROJECT_SRC})

target_link_libraries(QCAExample PUBLIC QCASim)

target_include_directories(QCAExample PUBLIC src)

source_group("" FILES ${PROJECT_SRC})