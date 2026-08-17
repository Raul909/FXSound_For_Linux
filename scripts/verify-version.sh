#!/bin/bash
# Verify version consistency across the project.
#
# The version is derived from package.json rather than hardcoded here — the
# previous hardcoded EXPECTED_VERSION was itself forgotten at release time, so
# the checker sat a release behind and reported the wrong answer for everything.

set -uo pipefail

cd "$(dirname "$0")/.." || exit 1

VERSION=$(node -p "require('./package.json').version" 2>/dev/null)
if [ -z "$VERSION" ]; then
    echo "❌ Could not read version from package.json"
    exit 1
fi

echo "🔍 FXSound Linux - Version Verification"
echo "========================================"
echo "Reference version (package.json): $VERSION"
echo ""

ERRORS=0

# check <label> <file> <actual-version>
# An empty actual value means the pattern did not match at all.
check() {
    local label="$1" file="$2" actual="$3"
    if [ ! -f "$file" ]; then
        echo "   ⏭️  $label: $file not present, skipping"
        return
    fi
    if [ -z "$actual" ]; then
        echo "   ❌ $label: no version found in $file"
        ERRORS=$((ERRORS + 1))
    elif [ "$actual" = "$VERSION" ]; then
        echo "   ✅ $label: $actual"
    else
        echo "   ❌ $label: $actual (expected $VERSION)"
        ERRORS=$((ERRORS + 1))
    fi
}

# count_check <label> <file> <pattern> <expected-count-source>
# Verifies a file contains no *stale* version strings.
check_no_stale() {
    local label="$1" file="$2"
    if [ ! -f "$file" ]; then
        echo "   ⏭️  $label: $file not present, skipping"
        return
    fi
    local stale
    stale=$(grep -oE '[0-9]+\.[0-9]+\.[0-9]+' "$file" | sort -u | grep -v "^${VERSION}$" | tr '\n' ' ')
    if [ -n "$stale" ]; then
        echo "   ❌ $label: found other version(s): $stale"
        ERRORS=$((ERRORS + 1))
    else
        echo "   ✅ $label: all references at $VERSION"
    fi
}

echo "🦀 Rust crate..."
check "Cargo.toml" src-tauri/Cargo.toml \
    "$(grep -m1 '^version' src-tauri/Cargo.toml | sed 's/version = "\(.*\)"/\1/')"
check "Cargo.lock" src-tauri/Cargo.lock \
    "$(grep -A1 'name = "fxsound-linux"' src-tauri/Cargo.lock | grep -m1 '^version' | sed 's/version = "\(.*\)"/\1/')"

echo "⚙️  Tauri..."
check "tauri.conf.json" src-tauri/tauri.conf.json \
    "$(node -p "require('./src-tauri/tauri.conf.json').version" 2>/dev/null)"

echo "📦 Packaging..."
# Snap Store rejects a re-upload of an already-published version, so a stale
# snapcraft.yaml fails the release job rather than shipping something wrong.
check "snapcraft.yaml" snap/snapcraft.yaml \
    "$(grep -m1 '^version:' snap/snapcraft.yaml | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')"
# AppStream: the newest <release> must be this version or software centres
# show the wrong changelog.
check "metainfo.xml (latest release)" com.fxsound.linux.metainfo.xml \
    "$(grep -m1 '<release version=' com.fxsound.linux.metainfo.xml | sed 's/.*version="\([^"]*\)".*/\1/')"
check "PKGBUILD" PKGBUILD \
    "$(grep -m1 '^pkgver=' PKGBUILD 2>/dev/null | sed 's/pkgver=//')"
check "Flatpak manifest" com.fxsound.linux.yml \
    "$(grep -m1 'tag: v' com.fxsound.linux.yml 2>/dev/null | sed 's/.*tag: v\(.*\)/\1/')"

echo "🌐 Landing page & docs..."
# These hardcode release asset filenames. If they lag, every download button
# 404s, which has shipped broken before — so require zero stale references.
check_no_stale "landing/index.html" landing/index.html
check_no_stale "GUIDE.md" GUIDE.md

echo "📝 Changelog..."
check "CHANGELOG.md (top entry)" CHANGELOG.md \
    "$(grep -m1 -oE '^## \[[0-9]+\.[0-9]+\.[0-9]+\]' CHANGELOG.md | tr -d '#[] ')"

echo ""
echo "Note: the UI version in the status bar is injected from package.json by"
echo "Vite at build time, so it needs no separate check."

echo ""
echo "========================================"
if [ $ERRORS -eq 0 ]; then
    echo "✅ All versions are consistent: $VERSION"
    echo ""
    echo "Ready to build and release!"
    exit 0
else
    echo "❌ Found $ERRORS version mismatch(es)"
    echo ""
    echo "Please update all versions to $VERSION"
    exit 1
fi
