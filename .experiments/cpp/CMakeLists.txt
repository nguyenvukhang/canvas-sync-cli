cmake_minimum_required(VERSION 3.10)

project(canvas_sync VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_EXPORT_COMPILE_COMMANDS 1) # to build compile_commands.json

# include the `./include` directory for static downloads
include_directories(${PROJECT_SOURCE_DIR}/include)

# get GoogleTest suite
include(FetchContent)
FetchContent_Declare(googletest
  URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
  DOWNLOAD_EXTRACT_TIMESTAMP TRUE)
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)

# get Async++ library
FetchContent_Declare(asyncplusplus
  URL https://github.com/Amanieu/asyncplusplus/archive/refs/tags/v1.1.zip
  DOWNLOAD_EXTRACT_TIMESTAMP TRUE)

FetchContent_MakeAvailable(googletest asyncplusplus)

# `canvas_sync` as main output binary
add_executable(canvas_sync
  types.cc
  errors.cc
  canvas_api.cc
  filetree.cc
  httpjson.cc
  main.cc)

# link the `./bulid` directory
target_include_directories(canvas_sync PRIVATE build)

# link openssl stuff so `https` requests are handled properly
set(OPENSSL_DIR /opt/homebrew/Cellar/openssl@3/3.0.7)
set(OPENSSL_LIB ${OPENSSL_DIR}/lib/libssl.a ${OPENSSL_DIR}/lib/libcrypto.a)
target_link_libraries(canvas_sync ${OPENSSL_LIB})
target_include_directories(canvas_sync PRIVATE ${OPENSSL_DIR}/include)

# target_link_libraries(canvas_sync Async++)

# configure compile-time substitutions
configure_file(main.h.in main.h)

# `cs_test` as testing binary
enable_testing()
add_executable(cs_test
  filetree.cc filetree_test.cc
  canvas_api.cc canvas_api_test.cc
  types.cc)
target_link_libraries(cs_test GTest::gtest_main GTest::gmock_main)
include(GoogleTest)
gtest_discover_tests(cs_test)
