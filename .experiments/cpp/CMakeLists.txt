cmake_minimum_required(VERSION 3.10)

project(canvas_sync VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_EXPORT_COMPILE_COMMANDS 1)

add_executable(canvas_sync main.cpp errors.cpp canvas_api.cpp)
include_directories(${PROJECT_SOURCE_DIR}/include)

target_link_libraries(canvas_sync /opt/homebrew/Cellar/openssl@3/3.0.7/lib/libssl.a)
target_link_libraries(canvas_sync /opt/homebrew/Cellar/openssl@3/3.0.7/lib/libcrypto.a)
target_include_directories(canvas_sync PRIVATE /opt/homebrew/Cellar/openssl@3/3.0.7/include)

# message("hello from CMAKE")

# uncomment this to link rust
# target_link_libraries(canvas_sync ${CMAKE_SOURCE_DIR}/build/libembed.a)

target_include_directories(canvas_sync PUBLIC build)

configure_file(main.h.in main.h)
