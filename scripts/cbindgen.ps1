#!/usr/bin/env sh

cbindgen --config ./config/cbindgen.toml --crate mintaka-native --output ./include/mintaka/native.h
