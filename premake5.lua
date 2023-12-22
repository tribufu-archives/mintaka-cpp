--- @diagnostic disable: undefined-global

workspace "mintaka"
    location "."
    configurations { "debug", "release" }

if _ACTION == "vs2022" then
    platforms {
        "windows-aarch64",
        "windows-i686",
        "windows-x86_64",
    }
end

if _ACTION == "gmake2" then
    platforms {
        "linux-aarch64",
        "linux-i686",
        "linux-x86_64",
    }
end

if _ACTION == "xcode4" then
    platforms {
        "mac-aarch64",
        "mac-x86_64",
    }
end

include "examples/c"
include "examples/cpp"
