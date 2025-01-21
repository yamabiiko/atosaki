cmake_minimum_required(VERSION 3.27)

project(kuukiyomu
    DESCRIPTION "kuukiyomu plugin for Hyprland"
    VERSION 0.1
)

set(CMAKE_CXX_STANDARD 23)
add_compile_definitions(WLR_USE_UNSTABLE)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -I/usr/include/pixman-1 -I/usr/include/libdrm -I/usr/include/hyprland/protocols")

file(GLOB_RECURSE SRC "*.cpp")
list(FILTER SOURCES EXCLUDE REGEX "CMakeFiles/.*")

add_library(kuukiyomu SHARED ${SRC})

find_package(PkgConfig REQUIRED)
find_package(Boost REQUIRED serialization)
pkg_check_modules(deps REQUIRED IMPORTED_TARGET
    hyprland
    libinput
    libudev
    libdrm
    pixman-1
    xkbcommon
)
target_link_libraries(kuukiyomu PRIVATE rt Boost::serialization)


install(TARGETS kuukiyomu)
