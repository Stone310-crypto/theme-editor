#!/bin/bash
# Build-Script für die Gaming-WASM-Extension
set -e

echo "🦀 Compiling Gaming Extension → WASM..."

cd "$(dirname "$0")"

# Rustup cargo verwenden (nicht Homebrew — wegen WASM-Target)
export PATH="$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$HOME/.cargo/bin:$PATH"

# WASM-Target installieren
rustup target add wasm32-unknown-unknown 2>/dev/null || true

# Release-Build (mit rustup cargo für WASM-Support)
cargo build --release --target wasm32-unknown-unknown

# WASM-Datei finden und kopieren
WASM_FILE=$(find target/wasm32-unknown-unknown/release -name "*.wasm" -type f 2>/dev/null | head -1)
if [ -z "$WASM_FILE" ]; then
    echo "❌ Keine .wasm-Datei gefunden!"
    echo "   Dateien in target/wasm32-unknown-unknown/release/:"
    ls -la target/wasm32-unknown-unknown/release/ 2>/dev/null || echo "   (Ordner leer)"
    exit 1
fi

cp "$WASM_FILE" module.wasm

echo "✅ Fertig:"
ls -lh module.wasm
echo ""
echo "Upload: gh release create v1.0.0 module.wasm"
