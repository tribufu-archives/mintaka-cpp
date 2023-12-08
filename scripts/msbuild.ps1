#!/usr/bin/env pwsh

echo "Building for windows-x86_64"
cargo build -p mintaka-native

New-Item "bin/windows/x86_64" -ItemType Directory -Force
Remove-Item -Path "bin/windows/x86_64/*" -Force

Copy-Item -Path "target/debug/mintaka_native.dll" -Destination "bin/windows/x86_64" -Force
Copy-Item -Path "target/debug/mintaka_native.dll.exp" -Destination "bin/windows/x86_64" -Force
Copy-Item -Path "target/debug/mintaka_native.dll.lib" -Destination "bin/windows/x86_64" -Force
Copy-Item -Path "target/debug/mintaka_native.lib" -Destination "bin/windows/x86_64" -Force
Copy-Item -Path "target/debug/mintaka_native.pdb" -Destination "bin/windows/x86_64" -Force

msbuild /p:Configuration="debug" /p:Platform="windows-x86_64"
