collectSources(PROJECT_SRC src/QCACore)
collectSources(PROJECT_SRC_SIM src/QCACore/Sim)
collectSources(PROJECT_SRC_UTILS src/QCACore/Utils)
collectSources(PROJECT_SRC_UTILS_JSON src/QCACore/Utils/Json)

if(${QCAS_BUILD_SHARED})
    set(LIB_TYPE "SHARED")
else()
    set(LIB_TYPE "STATIC")
endif()

FetchContent_Declare(
    json
    GIT_REPOSITORY https://github.com/nlohmann/json.git
    GIT_TAG        bbe337c3a30d5f6eea418b4aee399525536de37a
)

FetchContent_MakeAvailable(json)

add_library(QCACore ${LIB_TYPE} 
${PROJECT_SRC}
${PROJECT_SRC_SIM}
${PROJECT_SRC_UTILS}
${PROJECT_SRC_UTILS_JSON})

target_include_directories(QCACore PUBLIC src)

target_precompile_headers(QCACore PUBLIC src/qcaspch.h)

target_link_libraries(QCACore PUBLIC 
nlohmann_json)

source_group("QCACore" FILES ${PROJECT_SRC})
source_group("QCACore/Sim" FILES ${PROJECT_SRC_SIM})
source_group("QCACore/Utils" FILES ${PROJECT_SRC_UTILS})
source_group("QCACore/Utils/Json" FILES ${PROJECT_SRC_UTILS_JSON})