collectSources(PROJECT_SRC src/QCAS)

if(${QCAS_BUILD_SHARED})
    set(LIB_TYPE "SHARED")
else()
    set(LIB_TYPE "STATIC")
endif()

add_library(QCASim ${LIB_TYPE} 
${PROJECT_SRC})

target_include_directories(QCASim PUBLIC src)

target_precompile_headers(QCASim PUBLIC src/qcaspch.h)

source_group("QCAS" FILES ${PROJECT_SRC})