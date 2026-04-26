#!/usr/bin/env bash
set -e
echo "=== Gemma4 Chat Build Script (Ubuntu) ==="
# 1. Build chat-server
echo ""
echo "[1/4] Building chat-server..."
cargo build --release -p chat-server
# 2. Copy sidecar binary
echo ""
echo "[2/4] Copying sidecar binary..."
mkdir -p src-tauri/binaries
# Detect architecture
ARCH=$(rustc -Vv | grep host | cut -f2 -d' ')
echo "  Target: $ARCH"
cp target/release/chat-server "src-tauri/binaries/chat-server-${ARCH}"
chmod +x "src-tauri/binaries/chat-server-${ARCH}"
# Also copy for dev mode (no triplet)
mkdir -p target/debug
cp target/release/chat-server target/debug/chat-server
chmod +x target/debug/chat-server
echo "  Dev binary copied to target/debug/chat-server"
# 3. Install npm deps
echo ""
echo "[3/4] Installing npm dependencies..."
npm install
# 4. Build Tauri app
echo ""
echo "[4/4] Building Tauri app..."
npm run tauri build
echo ""
echo "=== Build complete! ==="
echo "Output: src-tauri/target/release/bundle/"
