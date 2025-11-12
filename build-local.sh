#!/bin/bash

echo "Local Build Script for Bongo Cat Core"
echo "====================================="

# 创建输出目录
mkdir -p dist

# 检测当前平台
CURRENT_OS=$(uname -s)
CURRENT_ARCH=$(uname -m)

echo "Current platform: $CURRENT_OS $CURRENT_ARCH"
echo ""

# 构建当前平台
echo "Building for current platform ($CURRENT_OS $CURRENT_ARCH)..."
cargo build --release

# 复制到 dist 目录
if [ "$CURRENT_OS" = "Darwin" ]; then
    # macOS
    if [ "$CURRENT_ARCH" = "x86_64" ]; then
        cp target/release/bongo-cat-core dist/bongo-cat-core-macos-x86_64
        echo "✓ Built: dist/bongo-cat-core-macos-x86_64"
    elif [ "$CURRENT_ARCH" = "arm64" ]; then
        cp target/release/bongo-cat-core dist/bongo-cat-core-macos-arm64
        echo "✓ Built: dist/bongo-cat-core-macos-arm64"
    fi
elif [ "$CURRENT_OS" = "Linux" ]; then
    # Linux
    if [ "$CURRENT_ARCH" = "x86_64" ]; then
        cp target/release/bongo-cat-core dist/bongo-cat-core-linux-x86_64
        echo "✓ Built: dist/bongo-cat-core-linux-x86_64"
    elif [ "$CURRENT_ARCH" = "aarch64" ]; then
        cp target/release/bongo-cat-core dist/bongo-cat-core-linux-arm64
        echo "✓ Built: dist/bongo-cat-core-linux-arm64"
    fi
elif [[ "$CURRENT_OS" == MINGW* ]] || [[ "$CURRENT_OS" == MSYS* ]] || [[ "$CURRENT_OS" == CYGWIN* ]] || [[ "$CURRENT_OS" == *"NT"* ]]; then
    # Windows (Git Bash, MSYS2, etc.)
    cp target/release/bongo-cat-core.exe dist/bongo-cat-core-windows-x86_64.exe
    echo "✓ Built: dist/bongo-cat-core-windows-x86_64.exe"
fi

echo ""
echo "Build completed! Files in dist/:"
ls -la dist/

echo ""
echo "To build for other platforms:"
echo "- Run this script on the target platform"
echo "- Or use Docker: docker run --rm -v \$(pwd):/app -w /app rust:latest cargo build --release"