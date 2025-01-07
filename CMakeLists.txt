cmake_minimum_required(VERSION 3.23)
project(storm LANGUAGES CXX OBJC OBJCXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Common sources
set(COMMON_SOURCES
    src/main.cpp
)

# Platform-specific sources
if(APPLE)
    set(PLATFORM_SOURCES
        src/platform/macos/window.mm
    )
    set(PLATFORM_LIBS "-framework Cocoa")
elseif(WIN32)
    set(PLATFORM_SOURCES
        src/platform/windows/window.cpp
    )
    set(PLATFORM_LIBS "")
elseif(UNIX)
    set(PLATFORM_SOURCES
        src/platform/linux/window.cpp
    )
    set(PLATFORM_LIBS X11)
endif()

# Combine sources
set(SOURCES
    ${COMMON_SOURCES}
    ${PLATFORM_SOURCES}
)

add_executable(storm ${SOURCES})

# Link platform-specific libraries
target_link_libraries(storm PRIVATE ${PLATFORM_LIBS})
