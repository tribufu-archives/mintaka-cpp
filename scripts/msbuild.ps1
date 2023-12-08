#!/usr/bin/env pwsh

echo "Building for windows-x86_64"
cargo build -p mintaka-native

msbuild /p:Configuration="debug" /p:Platform="x86_64-pc-windows-msvc"
