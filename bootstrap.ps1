#!/usr/bin/env pwsh

if ($IsWindows)
{
    & "./vendor/premake/windows/premake5.exe" "vs2022"
}
elseif ($IsMacOS)
{
    & "./vendor/premake/mac/premake5" "xcode4"
}
elseif ($IsLinux)
{
    & "./vendor/premake/linux/premake5" "gmake2"
}
