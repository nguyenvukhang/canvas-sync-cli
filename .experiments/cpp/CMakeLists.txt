cmake_minimum_required(VERSION 3.10)

project(canvas_sync VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_EXPORT_COMPILE_COMMANDS 1) # to build compile_commands.json

# --- GoogleTest start
include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
  DOWNLOAD_EXTRACT_TIMESTAMP true
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

# link the tests
add_executable(
  canvas_sync_tests
  filetree_test.cc
)
target_link_libraries(
  canvas_sync_tests
  GTest::gtest_main
)
include(GoogleTest)
gtest_discover_tests(canvas_sync_tests)
# --- GoogleTest end

add_executable(canvas_sync
  main.cc
  errors.cc
  canvas_api.cc
  filetree.cc
  types.cc)

include_directories(${PROJECT_SOURCE_DIR}/include)

target_link_libraries(canvas_sync /opt/homebrew/Cellar/openssl@3/3.0.7/lib/libssl.a)
target_link_libraries(canvas_sync /opt/homebrew/Cellar/openssl@3/3.0.7/lib/libcrypto.a)
target_include_directories(canvas_sync PRIVATE /opt/homebrew/Cellar/openssl@3/3.0.7/include)

# message("hello from CMAKE")

# uncomment this to link rust
# target_link_libraries(canvas_sync ${CMAKE_SOURCE_DIR}/build/libembed.a)

target_include_directories(canvas_sync PUBLIC build)

configure_file(main.h.in main.h)
