#!/bin/bash
set -e

VERSION="0.1.0"
PACKAGE_NAME="zk"

# Создаём директорию для релизов
mkdir -p releases

# Linux (x86_64)
echo "Building for Linux (x86_64)..."
cargo build --release --target x86_64-unknown-linux-gnu
tar czf "releases/${PACKAGE_NAME}-v${VERSION}-linux-x86_64.tar.gz" -C target/x86_64-unknown-linux-gnu/release zk
sha256sum "releases/${PACKAGE_NAME}-v${VERSION}-linux-x86_64.tar.gz" > "releases/${PACKAGE_NAME}-v${VERSION}-linux-x86_64.tar.gz.sha256"

# Windows (x86_64)
echo "Building for Windows (x86_64)..."
cargo build --release --target x86_64-pc-windows-gnu
zip -j "releases/${PACKAGE_NAME}-v${VERSION}-windows-x86_64.zip" target/x86_64-pc-windows-gnu/release/zk.exe
sha256sum "releases/${PACKAGE_NAME}-v${VERSION}-windows-x86_64.zip" > "releases/${PACKAGE_NAME}-v${VERSION}-windows-x86_64.zip.sha256"

echo "Done! Release files are in the releases directory."