--- @diagnostic disable: undefined-global

workspace "mintaka"
    location "."

    configurations { "debug", "release" }

if _ACTION == "vs2022" then
    platforms {
        "aarch64-pc-windows-msvc",
        "i686-pc-windows-msvc",
        "x86_64-pc-windows-msvc",
    }
end

if _ACTION == "gmake2" then
    platforms {
        "aarch64-unknown-linux-gnu",
        "i686-unknown-linux-gnu",
        "x86_64-unknown-linux-gnu",
    }
end

if _ACTION == "xcode4" then
    platforms {
        "aarch64-apple-darwin",
        "x86_64-apple-darwin",
    }
end

include "examples/c"
include "examples/cpp"
