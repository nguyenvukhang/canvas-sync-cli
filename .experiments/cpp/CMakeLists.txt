cmake_minimum_required(VERSION 3.10)

project(canvas_sync VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_EXPORT_COMPILE_COMMANDS 1)

add_executable(canvas_sync main.cpp errors.cpp)

configure_file(main.h.in main.h)

# message("hello from CMAKE")

# uncomment this to link rust
# target_link_libraries(canvas_sync ${CMAKE_SOURCE_DIR}/build/libembed.a)

target_include_directories(canvas_sync PUBLIC build)
