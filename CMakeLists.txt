cmake_minimum_required(VERSION 3.23)
project(storm LANGUAGES CXX OBJC OBJCXX)

set(CMAKE_CXX_STANDARD 20)

set(SOURCES 
    src/main.cpp
    src/platform/windows/window.cpp
    src/platform/macos/window.mm
    src/platform/linux/window.cpp
)

add_executable(storm ${SOURCES})

if(APPLE)
    target_link_libraries(storm PRIVATE "-framework Cocoa")
elseif(UNIX)
    target_link_libraries(storm PRIVATE X11)
endif()
