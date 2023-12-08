#!/usr/bin/env pwsh

echo "Building for windows-x86_64"
cargo build -p mintaka-native --release
New-Item "lib/windows/x86_64" -ItemType Directory -Force
Remove-Item -Path "lib/windows/x86_64/*" -Force
Copy-Item -Path "target/release/mintaka_native.dll" -Destination "lib/windows/x86_64" -Force
Copy-Item -Path "target/release/mintaka_native.dll.lib" -Destination "lib/windows/x86_64" -Force
Copy-Item -Path "target/release/mintaka_native.lib" -Destination "lib/windows/x86_64" -Force

echo "Building for windows-i686"
cargo build -p mintaka-native --target i686-pc-windows-msvc --release
New-Item "lib/windows/i686" -ItemType Directory -Force
Remove-Item -Path "lib/windows/i686/*" -Force
Copy-Item -Path "target/i686-pc-windows-msvc/release/mintaka_native.dll" -Destination "lib/windows/i686" -Force
Copy-Item -Path "target/i686-pc-windows-msvc/release/mintaka_native.dll.lib" -Destination "lib/windows/i686" -Force
Copy-Item -Path "target/i686-pc-windows-msvc/release/mintaka_native.lib" -Destination "lib/windows/i686" -Force

echo "Building for windows-aarch64"
cargo build -p mintaka-native --target aarch64-pc-windows-msvc --release
New-Item "lib/windows/aarch64" -ItemType Directory -Force
Remove-Item -Path "lib/windows/aarch64/*" -Force
Copy-Item -Path "target/aarch64-pc-windows-msvc/release/mintaka_native.dll" -Destination "lib/windows/aarch64" -Force
Copy-Item -Path "target/aarch64-pc-windows-msvc/release/mintaka_native.dll.lib" -Destination "lib/windows/aarch64" -Force
Copy-Item -Path "target/aarch64-pc-windows-msvc/release/mintaka_native.lib" -Destination "lib/windows/aarch64" -Force
