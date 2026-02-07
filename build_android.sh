#!/bin/bash
# Build script for kokoro-g2p Android library

set -e

# Change to the script's directory
cd "$(dirname "$0")"

export ANDROID_NDK_HOME="${ANDROID_NDK_HOME:-$HOME/android-sdk/ndk/27.0.12077973}"
export PATH="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"
export CC_aarch64_linux_android=aarch64-linux-android24-clang
export CXX_aarch64_linux_android=aarch64-linux-android24-clang++
export AR_aarch64_linux_android=llvm-ar
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=aarch64-linux-android24-clang

echo "Building kokoro-g2p for Android arm64..."
cargo build --release --target aarch64-linux-android --features "jni,chinese,japanese"

# Copy to jniLibs
mkdir -p jniLibs/arm64-v8a
cp target/aarch64-linux-android/release/libkokoro_g2p.so jniLibs/arm64-v8a/

echo "Done! Library at jniLibs/arm64-v8a/libkokoro_g2p.so"
ls -la jniLibs/arm64-v8a/libkokoro_g2p.so
