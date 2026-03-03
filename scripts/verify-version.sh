#!/bin/bash
# Verify version consistency across the project

echo "🔍 FXSound Linux - Version Verification"
echo "========================================"
echo ""

EXPECTED_VERSION="1.0.0"
ERRORS=0

# Check package.json
echo "📦 Checking package.json..."
PKG_VERSION=$(grep '"version"' package.json | head -1 | sed 's/.*"version": "\(.*\)".*/\1/')
if [ "$PKG_VERSION" = "$EXPECTED_VERSION" ]; then
    echo "   ✅ package.json: $PKG_VERSION"
else
    echo "   ❌ package.json: $PKG_VERSION (expected $EXPECTED_VERSION)"
    ERRORS=$((ERRORS + 1))
fi

# Check Cargo.toml
echo "🦀 Checking Cargo.toml..."
CARGO_VERSION=$(grep '^version' src-tauri/Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
if [ "$CARGO_VERSION" = "$EXPECTED_VERSION" ]; then
    echo "   ✅ Cargo.toml: $CARGO_VERSION"
else
    echo "   ❌ Cargo.toml: $CARGO_VERSION (expected $EXPECTED_VERSION)"
    ERRORS=$((ERRORS + 1))
fi

# Check tauri.conf.json
echo "⚙️  Checking tauri.conf.json..."
TAURI_VERSION=$(grep '"version"' src-tauri/tauri.conf.json | sed 's/.*"version": "\(.*\)".*/\1/')
if [ "$TAURI_VERSION" = "$EXPECTED_VERSION" ]; then
    echo "   ✅ tauri.conf.json: $TAURI_VERSION"
else
    echo "   ❌ tauri.conf.json: $TAURI_VERSION (expected $EXPECTED_VERSION)"
    ERRORS=$((ERRORS + 1))
fi

# Check App.jsx
echo "⚛️  Checking App.jsx..."
UI_VERSION=$(grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+' src/App.jsx | head -1 | sed 's/v//')
if [ "$UI_VERSION" = "$EXPECTED_VERSION" ]; then
    echo "   ✅ App.jsx: v$UI_VERSION"
else
    echo "   ❌ App.jsx: v$UI_VERSION (expected v$EXPECTED_VERSION)"
    ERRORS=$((ERRORS + 1))
fi

# Check PKGBUILD
echo "📦 Checking PKGBUILD..."
if [ -f PKGBUILD ]; then
    PKG_BUILD_VERSION=$(grep '^pkgver=' PKGBUILD | sed 's/pkgver=//')
    if [ "$PKG_BUILD_VERSION" = "$EXPECTED_VERSION" ]; then
        echo "   ✅ PKGBUILD: $PKG_BUILD_VERSION"
    else
        echo "   ❌ PKGBUILD: $PKG_BUILD_VERSION (expected $EXPECTED_VERSION)"
        ERRORS=$((ERRORS + 1))
    fi
fi

# Check Flatpak manifest
echo "📦 Checking Flatpak manifest..."
if [ -f com.fxsound.linux.yml ]; then
    FLATPAK_VERSION=$(grep 'tag: v' com.fxsound.linux.yml | sed 's/.*tag: v\(.*\)/\1/')
    if [ "$FLATPAK_VERSION" = "$EXPECTED_VERSION" ]; then
        echo "   ✅ Flatpak manifest: v$FLATPAK_VERSION"
    else
        echo "   ❌ Flatpak manifest: v$FLATPAK_VERSION (expected v$EXPECTED_VERSION)"
        ERRORS=$((ERRORS + 1))
    fi
fi

echo ""
echo "========================================"
if [ $ERRORS -eq 0 ]; then
    echo "✅ All versions are consistent: $EXPECTED_VERSION"
    echo ""
    echo "Ready to build and release!"
    exit 0
else
    echo "❌ Found $ERRORS version mismatch(es)"
    echo ""
    echo "Please update all versions to $EXPECTED_VERSION"
    exit 1
fi
