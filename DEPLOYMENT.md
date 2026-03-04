# Deployment Guide

Step-by-step instructions for publishing FXSound for Linux on Flatpak (Flathub), Snap Store, and AUR.

---

## Prerequisites

Before deploying, build and test the app locally:

```bash
npm install
npm run tauri:build
```

Output files will be in:

- `src-tauri/target/release/bundle/appimage/*.AppImage`
- `src-tauri/target/release/bundle/deb/*.deb`
- `src-tauri/target/release/bundle/rpm/*.rpm`

Make sure the app runs correctly before submitting to any store.

---

## Flatpak (Flathub)

Flathub is a universal Linux app store. Flatpak apps work on all major distros.

### Required Files

The project already includes the manifest and metadata:

- `com.fxsound.linux.yml` — Flatpak build manifest
- `com.fxsound.linux.desktop` — Desktop entry
- `com.fxsound.linux.metainfo.xml` — AppStream metadata

### Generate Dependency Sources

Flatpak builds run offline, so all dependencies must be pre-downloaded:

```bash
# Node.js dependencies
pip3 install aiohttp toml
wget https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/node/flatpak-node-generator.py
python3 flatpak-node-generator.py npm package-lock.json -o generated-sources.json

# Rust dependencies (already included as cargo-sources.json)
wget https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
python3 flatpak-cargo-generator.py src-tauri/Cargo.lock -o cargo-sources.json
```

### Build and Test Locally

```bash
flatpak-builder --user --install --force-clean build-dir com.fxsound.linux.yml
flatpak run com.fxsound.linux
```

### Submit to Flathub

1. Fork <https://github.com/flathub/flathub>
2. Create a new repo named `com.fxsound.linux` in your fork
3. Add the manifest and metadata files
4. Open a pull request to flathub/flathub linking your repo
5. Wait for review (typically 1–2 weeks)

**Official guide:** <https://docs.flathub.org/docs/for-app-authors/submission>

---

## Snapcraft (Snap Store)

Snap packages are primarily used on Ubuntu but work on many distros.

### Required Files

The project includes `snapcraft.yaml` at the project root.

### Build the Snap

```bash
# Install snapcraft
sudo snap install snapcraft --classic

# Build
snapcraft
```

### Test Locally

```bash
sudo snap install fxsound-linux_1.0.0_amd64.snap --dangerous
fxsound-linux
```

### Publish to Snap Store

```bash
# Create an account at https://snapcraft.io if you don't have one

# Login
snapcraft login

# Register the snap name (one-time)
snapcraft register fxsound-linux

# Upload and release
snapcraft upload fxsound-linux_1.0.0_amd64.snap --release=stable
```

**Official guide:** <https://snapcraft.io/docs/releasing-your-app>

---

## AUR (Arch Linux)

The AUR lets Arch Linux users install packages via helpers like `yay` or `paru`.

### Required Files

The project includes `PKGBUILD` and `.SRCINFO` at the project root.

### Test the Package

On an Arch Linux system (or Arch-based distro):

```bash
makepkg -si
```

### Update Checksums

After creating a GitHub release, update the checksum:

```bash
wget https://github.com/Raul909/FXSound_For_Linux/archive/refs/tags/v1.0.0.tar.gz
sha256sum v1.0.0.tar.gz
# Replace the sha256sums value in PKGBUILD with the output
```

Regenerate `.SRCINFO`:

```bash
makepkg --printsrcinfo > .SRCINFO
```

### Submit to AUR

```bash
# Create an account at https://aur.archlinux.org
# Add your SSH key to your AUR account

# Clone the AUR repo (creates a new package)
git clone ssh://aur@aur.archlinux.org/fxsound-linux.git aur-fxsound-linux
cd aur-fxsound-linux

# Copy your files in
cp /path/to/PKGBUILD /path/to/.SRCINFO .

# Commit and push
git add PKGBUILD .SRCINFO
git commit -m "Initial upload: fxsound-linux 1.0.0"
git push
```

**Official guide:** <https://wiki.archlinux.org/title/AUR_submission_guidelines>

---

## Updating Releases

When releasing a new version, update all three:

| Platform | What to Update |
|----------|---------------|
| Flathub  | Update `source-tag` in manifest, regenerate dependency sources, push to flathub repo |
| Snap     | Update `version` in `snapcraft.yaml`, rebuild and upload |
| AUR      | Update `pkgver` in `PKGBUILD`, update checksums, regenerate `.SRCINFO`, push |

---

## Troubleshooting

**Flatpak build fails?**

- Regenerate `generated-sources.json` and `cargo-sources.json`
- Test locally with `flatpak-builder` before submitting

**Snap build fails?**

- Run `snapcraft --debug` for interactive debugging
- Check that all plugs in `snapcraft.yaml` are valid

**AUR package fails?**

- Verify checksums match with `sha256sum`
- Test with `makepkg -si` on a clean Arch install
- Validate with `namcap PKGBUILD`
