#!/usr/bin/env pwsh

echo "Building for x86_64-pc-windows-msvc"
cargo build -p mintaka-native

msbuild /p:Configuration="debug" /p:Platform="x86_64-pc-windows-msvc"
