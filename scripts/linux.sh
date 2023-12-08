#!/bin/bash

echo "Building for linux-x86_64"
cargo build -p mintaka-native --release
mkdir -p lib/linux/x86_64
rm -f lib/linux/x86_64/*
cp target/release/libmintaka_native.so lib/linux/x86_64
cp target/release/libmintaka_native.a lib/linux/x86_64

echo "Building for linux-i686"
cargo build -p mintaka-native --target i686-unknown-linux-gnu --release
mkdir -p lib/linux/i686
rm -f lib/linux/i686/*
cp target/i686-unknown-linux-gnu/release/libmintaka_native.so lib/linux/i686
cp target/i686-unknown-linux-gnu/release/libmintaka_native.a lib/linux/i686

echo "Building for linux-aarch64"
cargo build -p mintaka-native --target aarch64-unknown-linux-gnu --release
mkdir -p lib/linux/aarch64
rm -f lib/linux/aarch64/*
cp target/aarch64-unknown-linux-gnu/release/libmintaka_native.so lib/linux/aarch64
cp target/aarch64-unknown-linux-gnu/release/libmintaka_native.a lib/linux/aarch64
