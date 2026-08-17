#!/bin/bash
# Quick reference for version management.
#
# Everything is derived from package.json — this file used to hardcode the
# version and drifted a release behind, listing files that no longer exist.

cd "$(dirname "$0")/.." || exit 1

VERSION=$(node -p "require('./package.json').version" 2>/dev/null)
: "${VERSION:=unknown}"

cat << EOF
╔══════════════════════════════════════════════════════════════╗
║           FXSound Linux - Version Management                 ║
╚══════════════════════════════════════════════════════════════╝

Current Version: ${VERSION}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📋 Version Locations (all must match package.json):

  1. package.json               → "version": "${VERSION}"
  2. src-tauri/Cargo.toml       → version = "${VERSION}"
  3. src-tauri/Cargo.lock       → fxsound-linux ${VERSION}
  4. src-tauri/tauri.conf.json  → "version": "${VERSION}"
  5. snap/snapcraft.yaml        → version: '${VERSION}'
  6. com.fxsound.linux.metainfo.xml → newest <release version="${VERSION}">
  7. landing/index.html         → download filenames
  8. GUIDE.md                   → download filenames
  9. CHANGELOG.md               → newest ## [${VERSION}] entry

  The status-bar version in the UI is injected from package.json by Vite at
  build time, so there is nothing to edit in src/.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔧 Quick Commands:

  Verify versions:
    ./scripts/verify-version.sh

  Test audio engine:
    cd src-tauri && cargo test --lib

  Lint + build frontend:
    npm run lint && npm run build

  Run app:
    npm run tauri:dev

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📦 Release Assets (npm run tauri:build):

    • fxsound-linux_${VERSION}_amd64.AppImage
    • fxsound-linux_${VERSION}_amd64.deb
    • fxsound-linux-${VERSION}-1.x86_64.rpm

  landing/index.html hardcodes these filenames because GitHub's
  releases/latest/download/<file> path needs the exact asset name. If they
  lag behind, every download button 404s.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🚀 Release Process:

  1. Bump:   npm version <new> --no-git-tag-version
             then update Cargo.toml, tauri.conf.json, snapcraft.yaml,
             metainfo.xml, landing/index.html, GUIDE.md, CHANGELOG.md
  2. Verify: ./scripts/verify-version.sh
  3. Test:   cargo test --lib && npm run lint && npm run build
  4. Tag:    git tag v${VERSION} && git push origin v${VERSION}

  The tag push triggers .github/workflows/release.yml, which builds the
  AppImage/.deb/.rpm, publishes the GitHub Release, pushes to the Snap Store
  stable channel, and deploys landing/ to Cloudflare Pages.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠️  Manual steps after each release:

  Until the pending release.yml changes land (they need a workflow-scoped
  token to push), do these by hand:

  • Production landing page — the CI deploy runs without --branch, and a tag
    checks out detached-HEAD, so wrangler publishes to a preview alias and
    production silently stays stale:

      npx wrangler pages deploy landing --project-name=fxsound-linux --branch=main

    Run it AFTER the release assets publish, then verify:
      curl -s https://fxsound-linux.pages.dev/ | grep AppImage

  • Release notes — replace the auto-generated notes with the CHANGELOG entry:

      gh release edit v${VERSION} --notes-file <changelog-section>

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📚 Documentation:

  • CHANGELOG.md  - Complete changelog
  • GUIDE.md      - Install and usage guide
  • DEPLOYMENT.md - Release and deployment notes

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

EOF
