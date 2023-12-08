#!/bin/bash

build_for_architecture() {
    ARCH=$1
    OUTPUT_DIR="bin/linux/$ARCH"

    echo "Building for linux-$ARCH"
    mkdir -p $OUTPUT_DIR
    rm -f $OUTPUT_DIR/*

    docker run --rm \
        -v $(pwd):/src/mintaka \
        -v ~/.cargo/registry:/usr/local/cargo/registry \
        -w /src/mintaka \
        reg.tribufu.com/cross:linux-$ARCH \
        cargo build --target $ARCH-unknown-linux-gnu --package mintaka-native

    cp target/$ARCH-unknown-linux-gnu/debug/libmintaka_native.so $OUTPUT_DIR
    cp target/$ARCH-unknown-linux-gnu/debug/libmintaka_native.a $OUTPUT_DIR
}

architectures=("aarch64" "i686" "x86_64")

for arch in "${architectures[@]}"; do
    build_for_architecture $arch
done
