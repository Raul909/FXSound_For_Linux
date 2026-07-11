#!/bin/bash
# Quick reference for version management

cat << 'EOF'
╔══════════════════════════════════════════════════════════════╗
║           FXSound Linux - Version Management                 ║
╚══════════════════════════════════════════════════════════════╝

Current Version: 1.1.1

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📋 Version Locations:

  1. package.json              → "version": "1.1.1"
  2. src-tauri/Cargo.toml      → version = "1.1.1"
  3. src-tauri/tauri.conf.json → "version": "1.1.1"
  4. src/App.jsx               → v1.1.1 (UI display)
  5. PKGBUILD                  → pkgver=1.1.1
  6. com.fxsound.linux.yml     → tag: v1.1.1

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔧 Quick Commands:

  Verify versions:
    ./verify-version.sh

  Build release:
    ./build-release.sh

  Test app:
    npm run tauri:dev

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📦 Build Output:

  When you run: npm run tauri:build

  You'll get:
    • fxsound-linux_1.1.1_amd64.AppImage
    • fxsound-linux_1.1.1_amd64.deb
    • fxsound-linux-1.1.1-1.x86_64.rpm

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🚀 Release Process:

  1. Verify: ./verify-version.sh
  2. Build:  ./build-release.sh
  3. Test:   Test the AppImage
  4. Tag:    git tag v1.1.1 && git push origin v1.1.1
  5. Upload: Create GitHub release and upload files

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📝 To Update Version (e.g., to 1.1.1):

  1. Edit package.json → "version": "1.1.1"
  2. Edit src-tauri/Cargo.toml → version = "1.1.1"
  3. Edit src-tauri/tauri.conf.json → "version": "1.1.1"
  4. Edit src/App.jsx → v1.1.1
  5. Edit PKGBUILD → pkgver=1.1.1
  6. Edit com.fxsound.linux.yml → tag: v1.1.1
  7. Update CHANGELOG.md
  8. Run: ./verify-version.sh

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📚 Documentation:

  • VERSION.md          - Version info and roadmap
  • CHANGELOG.md        - Complete changelog
  • VERSION_UPDATE.md   - Update summary

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Status: All versions synchronized to 1.1.1

EOF
