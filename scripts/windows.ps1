#!/usr/bin/env pwsh

function Build-For-Architecture {
    param(
        [string]$Arch
    )

    $OutputDir = "lib/windows/$Arch"
    Write-Output "Building for $Arch"

    cargo build -p mintaka-native --target $Arch-pc-windows-msvc --release

    if (-not (Test-Path $OutputDir)) {
        New-Item -ItemType Directory -Force -Path $OutputDir
    }

    Remove-Item -Path "$OutputDir/*" -Force
    Copy-Item -Path "target/$Arch-pc-windows-msvc/release/mintaka_native.dll" -Destination $OutputDir -Force
    Copy-Item -Path "target/$Arch-pc-windows-msvc/release/mintaka_native.dll.lib" -Destination $OutputDir -Force
    Copy-Item -Path "target/$Arch-pc-windows-msvc/release/mintaka_native.lib" -Destination $OutputDir -Force
}

$architectures = @("aarch64", "i686", "x86_64")

foreach ($arch in $architectures) {
    Build-For-Architecture -Arch $arch
}
