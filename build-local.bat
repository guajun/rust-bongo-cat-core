@echo off
echo Local Build Script for Bongo Cat Core (Windows)
echo ===============================================

REM 创建输出目录
if not exist dist mkdir dist

echo Building for Windows...
cargo build --release

REM 检查构建是否成功
if %ERRORLEVEL% NEQ 0 (
    echo Build failed!
    exit /b 1
)

REM 复制到 dist 目录
if exist target\release\bongo-cat-core.exe (
    copy target\release\bongo-cat-core.exe dist\bongo-cat-core-windows-x86_64.exe
    echo ✓ Built: dist\bongo-cat-core-windows-x86_64.exe
) else (
    echo Error: bongo-cat-core.exe not found in target\release\
    exit /b 1
)

echo.
echo Build completed! Files in dist\:
dir /b dist\

echo.
echo To build for other platforms:
echo - Use the provided build-local.sh script on Linux/macOS
echo - Or use Docker: docker run --rm -v %cd%:/app -w /app rust:latest cargo build --release