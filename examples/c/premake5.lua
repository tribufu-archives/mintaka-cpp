--- @diagnostic disable: undefined-global

project "example_c"
    location "."
    kind "ConsoleApp"
    language "C++"

    cppdialect "C++20"

    targetdir("../../bin/%{cfg.platform:gsub('-', '/')}")
    objdir("../../target/%{cfg.buildcfg}/%{cfg.platform}/%{prj.name}")

    files
    {
        "src/**.h",
        "src/**.c",
    }

    includedirs
    {
        "../../include",
        "../../vendor/*/include",
    }

    libdirs
    {
        "../../bin/%{cfg.platform:gsub('-', '/')}",
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

    filter { "platforms:windows-*" }
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
            "mintaka.dll.lib",
        }

        prelinkcommands
        {
        }

    filter { "platforms:mac-*" }
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
            "mintaka",
        }

        prelinkcommands
        {
        }

    filter { "platforms:linux-*" }
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
            "mintaka",
        }

        prelinkcommands
        {
        }

    filter { "platforms:android-*" }
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
            "mintaka",
        }

        prelinkcommands
        {
        }

    filter { "platforms:ios-*" }
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
            "mintaka",
        }

    -- Architecture

    filter { "platforms:*-i686" }
        architecture "x32"

        defines
        {
            "MINTAKA_32BITS",
            "MINTAKA_I686",
        }

    filter { "platforms:*-x86_64" }
        architecture "x64"

        defines
        {
            "MINTAKA_64BITS",
            "MINTAKA_X8664",
        }

    filter { "platforms:*-aarch64" }
        architecture "ARM64"

        defines
        {
            "MINTAKA_64BITS",
            "MINTAKA_AARCH64",
        }
