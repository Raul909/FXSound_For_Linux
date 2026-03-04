# Deployment Steps for FXSound Linux

## Prerequisites

1. **Create a GitHub Release**
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
   Then create a release on GitHub with the tag.

2. **Build the app locally first** to ensure everything works:
   ```bash
   npm install
   npm run tauri:build
   ```

---

## 1. Flathub Deployment

### Requirements
- GitHub account
- Flathub account (sign in with GitHub at flathub.org)

### Steps

1. **Generate Flatpak sources** (for Node.js dependencies):
   ```bash
   # Install flatpak-node-generator
   pip3 install aiohttp toml
   wget https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/node/flatpak-node-generator.py
   
   # Generate sources
   python3 flatpak-node-generator.py npm package-lock.json -o generated-sources.json
   ```

2. **Generate Cargo sources** (for Rust dependencies):
   ```bash
   # Install flatpak-cargo-generator
   wget https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
   
   # Generate sources
   python3 flatpak-cargo-generator.py src-tauri/Cargo.lock -o cargo-sources.json
   ```

3. **Update manifest** - Add `cargo-sources.json` to the sources in `com.fxsound.linux.yml`

4. **Test locally**:
   ```bash
   flatpak-builder --user --install --force-clean build-dir com.fxsound.linux.yml
   flatpak run com.fxsound.linux
   ```

5. **Submit to Flathub**:
   - Fork https://github.com/flathub/flathub
   - Create a new repository named `com.fxsound.linux`
   - Add your manifest files
   - Submit PR to flathub/flathub with link to your repo
   - Wait for review (usually 1-2 weeks)

**Documentation**: https://docs.flathub.org/docs/for-app-authors/submission

---

## 2. Snap Store Deployment

### Requirements
- Ubuntu One account
- Snapcraft account (snapcraft.io)

### Steps

1. **Install snapcraft**:
   ```bash
   sudo snap install snapcraft --classic
   ```

2. **Login**:
   ```bash
   snapcraft login
   ```

3. **Register the snap name**:
   ```bash
   snapcraft register fxsound-linux
   ```

4. **Build the snap**:
   ```bash
   snapcraft
   ```

5. **Test locally**:
   ```bash
   sudo snap install fxsound-linux_1.0.0_amd64.snap --dangerous
   fxsound-linux
   ```

6. **Upload to Snap Store**:
   ```bash
   snapcraft upload fxsound-linux_1.0.0_amd64.snap --release=stable
   ```

**Documentation**: https://snapcraft.io/docs/releasing-your-app

---

## 3. AUR Deployment

### Requirements
- Arch Linux system (or Arch-based distro)
- AUR account (aur.archlinux.org)
- SSH key added to AUR account

### Steps

1. **Update PKGBUILD checksums**:
   ```bash
   # Download the source tarball
   wget https://github.com/Raul909/FXSound_For_Linux/archive/refs/tags/v1.0.0.tar.gz
   
   # Generate SHA256
   sha256sum v1.0.0.tar.gz
   
   # Update sha256sums in PKGBUILD
   ```

2. **Test the package**:
   ```bash
   makepkg -si
   ```

3. **Generate .SRCINFO**:
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```

4. **Clone AUR repository**:
   ```bash
   git clone ssh://aur@aur.archlinux.org/fxsound-linux.git aur-fxsound-linux
   cd aur-fxsound-linux
   ```

5. **Add files and push**:
   ```bash
   cp ../PKGBUILD ../.SRCINFO .
   git add PKGBUILD .SRCINFO
   git commit -m "Initial commit: fxsound-linux 1.0.0"
   git push
   ```

**Documentation**: https://wiki.archlinux.org/title/AUR_submission_guidelines

---

## Post-Deployment

### Update README.md
Replace placeholders:
- `YOUR_USERNAME` → `Raul909`
- `YOUR_DEPLOY_URL_HERE` → Your actual deployment URL

### Monitor
- Flathub: Check build status at https://buildbot.flathub.org
- Snap Store: Check dashboard at https://snapcraft.io/fxsound-linux
- AUR: Monitor comments at https://aur.archlinux.org/packages/fxsound-linux

---

## Troubleshooting

**Flatpak build fails?**
- Check generated-sources.json is up to date
- Verify all dependencies are listed
- Test with `flatpak-builder` locally first

**Snap build fails?**
- Check snapcraft.yaml syntax
- Verify all plugs are correct
- Test with `snapcraft --debug`

**AUR package fails?**
- Verify SHA256 checksum matches
- Test with `makepkg -si` locally
- Check PKGBUILD syntax with `namcap`

---

## Notes

- All three platforms require manual submission
- Flathub has the longest review process (1-2 weeks)
- Snap Store is fastest (usually same day)
- AUR is instant once pushed
- You'll need to update all three when releasing new versions
