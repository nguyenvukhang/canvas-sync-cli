cmake_minimum_required(VERSION 3.10)

project(canvas_sync VERSION 0.1 LANGUAGES CXX)

add_executable(canvas_sync main.cpp)

configure_file(main.h.in main.h)
message("---------------------")
message(${PROJECT_SOURCE_DIR})
message("---------------------")
target_link_libraries(canvas_sync ${CMAKE_SOURCE_DIR}/build/libembed.a)

target_include_directories(canvas_sync PUBLIC build)
