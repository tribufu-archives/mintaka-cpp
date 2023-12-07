--- @diagnostic disable: undefined-global

project "example_cpp"
    location "."
    kind "ConsoleApp"
    language "C"

    targetdir("../../target/%{cfg.buildcfg}")
    objdir("../../target/%{cfg.platform}/%{cfg.buildcfg}/examples/%{prj.name}")

    files
    {
        "src/**.h",
        "src/**.cpp",
    }

    includedirs
    {
        "../../include",
        "../../vendor/*/include",
    }

    libdirs
    {
        "../../target/%{cfg.buildcfg}",
        "../../target/%{cfg.platform}/%{cfg.buildcfg}",
    }

    -- Profile

    filter { "configurations:debug" }
        runtime "Debug"
        symbols "On"

        defines
        {
            "MINTAKA_DEVEL",
            "DEBUG",
            "TRACE",
        }

    filter { "configurations:release" }
        runtime "Release"
        optimize "On"

        defines
        {
            "MINTAKA_RETAIL",
            "RELEASE",
        }

    -- Platform

    filter { "platforms:*-pc-windows-msvc" }
        system "windows"
        systemversion "latest"
        staticruntime "On"

        toolset "msc"

        defines
        {
            "MINTAKA_DESKTOP",
            "MINTAKA_WINDOWS",
        }

        links
        {
            "mintaka_native.dll.lib",
        }

        prelinkcommands
        {
        }

    filter { "platforms:*-apple-darwin" }
        system "macosx"
        systemversion "10.15"

        toolset "clang"

        defines
        {
            "MINTAKA_APPLE",
            "MINTAKA_DESKTOP",
            "MINTAKA_MAC",
        }

        links
        {
            "mintaka_native",
        }

        prelinkcommands
        {
        }

    filter { "platforms:*-unknown-linux-gnu" }
        system "linux"

        toolset "gcc"

        defines
        {
            "MINTAKA_DESKTOP",
            "MINTAKA_LINUX",
            "MINTAKA_UNIX",
        }

        links
        {
            "mintaka_native",
        }

        prelinkcommands
        {
        }

    filter { "platforms:*-linux-android*" }
        system "android"

        toolset "clang"

        defines
        {
            "MINTAKA_ANDROID",
            "MINTAKA_MOBILE",
            "MINTAKA_UNIX",
        }

        links
        {
            "mintaka_native",
        }

        prelinkcommands
        {
        }

    filter { "platforms:*-apple-ios" }
        system "ios"
        systemversion "13.0"

        toolset "clang"

        defines
        {
            "MINTAKA_APPLE",
            "MINTAKA_IOS",
            "MINTAKA_MOBILE",
        }

        links
        {
            "mintaka_native",
        }

    -- Architecture

    filter { "platforms:i686-*" }
        architecture "x32"

        defines
        {
            "MINTAKA_32BITS",
            "MINTAKA_I686",
        }

    filter { "platforms:x86_64-*" }
        architecture "x64"

        defines
        {
            "MINTAKA_64BITS",
            "MINTAKA_X8664",
        }

    filter { "platforms:aarch64-*" }
        architecture "ARM64"

        defines
        {
            "MINTAKA_64BITS",
            "MINTAKA_AARCH64",
        }
