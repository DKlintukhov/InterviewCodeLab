cmake_minimum_required(VERSION 3.25.1)

set(TARGET InterviewCodeLab)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_C_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_C_EXTENSIONS OFF)
set(CMAKE_COMPILE_WARNING_AS_ERROR ON)

enable_testing()

project(${TARGET} LANGUAGES C CXX)

find_package(Threads REQUIRED)
find_package(Boost COMPONENTS unit_test_framework REQUIRED)

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    if(MSVC)
        set(CMAKE_MSVC_RUNTIME_LIBRARY MultiThreadedDebug)
    endif()
elseif(CMAKE_BUILD_TYPE STREQUAL "Release")
    if(CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
        set(CMAKE_MSVC_RUNTIME_LIBRARY MultiThreaded)
    endif()
endif()

set(SRC ${CMAKE_SOURCE_DIR}/src/util.cpp)

add_executable(${TARGET} main.cpp ${SRC})
target_link_libraries(${TARGET} PRIVATE Threads::Threads ${Boost_LIBRARIES})
target_include_directories(${TARGET} PRIVATE ${CMAKE_SOURCE_DIR}/src ${CMAKE_SOURCE_DIR} ${Boost_INCLUDE_DIR})
target_precompile_headers(${TARGET} PRIVATE pch.h)

add_test(NAME ${TARGET} COMMAND $<TARGET_FILE:${TARGET}> --log_level=message)
