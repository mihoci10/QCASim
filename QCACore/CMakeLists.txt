collectSources(PROJECT_SRC src/QCACore)
collectSources(PROJECT_SRC_SIMULATION src/QCACore/Simulation)
collectSources(PROJECT_SRC_DATA src/QCACore/Data)
collectSources(PROJECT_SRC_UTILITIES src/QCACore/Utilities)
collectSources(PROJECT_SRC_UTILITIES_JSON src/QCACore/Utilities/Json)

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
${PROJECT_SRC_SIMULATION}
${PROJECT_SRC_DATA}
${PROJECT_SRC_UTILITIES}
${PROJECT_SRC_UTILITIES_JSON})

target_include_directories(QCACore PUBLIC src)

target_precompile_headers(QCACore PUBLIC src/qcaspch.h)

target_link_libraries(QCACore PUBLIC 
nlohmann_json)

source_group("QCACore" FILES ${PROJECT_SRC})
source_group("QCACore/Simulation" FILES ${PROJECT_SRC_SIMULATION})
source_group("QCACore/Data" FILES ${PROJECT_SRC_DATA})
source_group("QCACore/Utilities" FILES ${PROJECT_SRC_UTILITIES})
source_group("QCACore/Utilities/Json" FILES ${PROJECT_SRC_UTILITIES_JSON})