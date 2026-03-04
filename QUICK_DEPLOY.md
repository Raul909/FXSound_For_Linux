# Quick Deployment Reference

## 🚀 Fastest Way to Distribute (5 minutes)

### 1. Push to GitHub
```bash
git init
git add .
git commit -m "Initial release"
git remote add origin https://github.com/YOUR_USERNAME/fxsound-linux.git
git push -u origin main
```

### 2. Create Release
```bash
# Tag your version
git tag v1.0.0
git push origin v1.0.0

# GitHub Actions will automatically:
# - Build AppImage, DEB, and RPM
# - Create a release
# - Upload all files
```

### 3. Share Download Link
```
https://github.com/YOUR_USERNAME/fxsound-linux/releases/latest
```

**Done!** Users can now download and install your app.

---

## 📦 Distribution Channels Comparison

| Channel | Setup Time | Reach | Auto-Updates | Cost |
|---------|-----------|-------|--------------|------|
| **GitHub Releases** | 5 min | High | No | Free |
| **Flathub** | 2-3 hours | Very High | Yes | Free |
| **Snap Store** | 1-2 hours | High | Yes | Free |
| **AUR** | 30 min | Medium | No | Free |

**Recommendation:** Start with GitHub Releases, add Flathub later for wider reach.

---

## 🎯 Step-by-Step: GitHub Releases

### Prerequisites
- GitHub account
- Git installed
- Code pushed to GitHub

### Steps

1. **Ensure workflow file exists:**
   - File: `.github/workflows/release.yml` ✓ (already created)

2. **Build locally to test:**
   ```bash
   npm run tauri:build
   ```

3. **Commit everything:**
   ```bash
   git add .
   git commit -m "Ready for v1.0.0 release"
   git push
   ```

4. **Create and push tag:**
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

5. **Wait for GitHub Actions:**
   - Go to: `https://github.com/YOUR_USERNAME/fxsound-linux/actions`
   - Watch the build (takes ~10-15 minutes)
   - Release will be created automatically

6. **Verify release:**
   - Go to: `https://github.com/YOUR_USERNAME/fxsound-linux/releases`
   - Should see v1.0.0 with AppImage, DEB, and RPM files

7. **Update README:**
   - Replace `YOUR_USERNAME` with your actual GitHub username
   - Commit and push

---

## 🌐 Optional: Add Download Website

### Using GitHub Pages (Free)

1. **Create `docs/index.html`** (already created in DEPLOYMENT_GUIDE.md)

2. **Enable GitHub Pages:**
   - Go to repo Settings → Pages
   - Source: `main` branch, `/docs` folder
   - Save

3. **Your website:**
   ```
   https://YOUR_USERNAME.github.io/fxsound-linux
   ```

---

## 📱 Optional: Submit to Flathub

### Why Flathub?
- Works on ALL Linux distros
- Automatic updates
- Sandboxed security
- Centralized app store

### Quick Steps

1. **Create manifest files** (templates in DEPLOYMENT_GUIDE.md):
   - `com.fxsound.linux.yml`
   - `com.fxsound.linux.desktop`
   - `com.fxsound.linux.metainfo.xml`

2. **Test locally:**
   ```bash
   flatpak-builder build-dir com.fxsound.linux.yml --force-clean --install --user
   flatpak run com.fxsound.linux
   ```

3. **Submit to Flathub:**
   - Fork: https://github.com/flathub/flathub
   - Add your manifest
   - Create pull request

4. **Wait for review** (1-2 weeks)

**Guide:** https://docs.flathub.org/docs/for-app-authors/submission

---

## 🔄 Updating Your App

### Release New Version

```bash
# 1. Update version in package.json and src-tauri/tauri.conf.json
# 2. Commit changes
git add .
git commit -m "Bump version to 1.1.0"
git push

# 3. Create new tag
git tag v1.1.0
git push origin v1.1.0

# 4. GitHub Actions builds and releases automatically
```

---

## 📊 Download Statistics

### GitHub Releases
- View at: `https://github.com/YOUR_USERNAME/fxsound-linux/releases`
- Shows download count per file

### Flathub
- View at: `https://flathub.org/apps/com.fxsound.linux`
- Shows installs and updates

---

## 🆘 Troubleshooting

### Build fails on GitHub Actions?

**Check logs:**
```
https://github.com/YOUR_USERNAME/fxsound-linux/actions
```

**Common issues:**
- Missing dependencies → Update workflow file
- Rust version → Update `rust-toolchain` in workflow
- Node version → Update `node-version` in workflow

### Users can't install?

**Check:**
- File permissions (AppImage needs `chmod +x`)
- Dependencies installed
- Architecture matches (x86_64 only)

---

## 📚 Full Documentation

- **[DEPLOYMENT_GUIDE.md](./DEPLOYMENT_GUIDE.md)** - Complete deployment guide
- **[INSTALL.md](./INSTALL.md)** - User installation guide
- **[BUILD.md](./BUILD.md)** - Build from source guide

---

## ✅ Deployment Checklist

- [ ] Code pushed to GitHub
- [ ] `.github/workflows/release.yml` exists
- [ ] Version updated in `package.json` and `tauri.conf.json`
- [ ] Tag created and pushed (`git tag v1.0.0 && git push origin v1.0.0`)
- [ ] GitHub Actions build successful
- [ ] Release created with files
- [ ] README updated with correct download links
- [ ] Tested AppImage on clean system
- [ ] (Optional) GitHub Pages enabled
- [ ] (Optional) Submitted to Flathub
- [ ] (Optional) Submitted to Snap Store
- [ ] (Optional) Submitted to AUR

---

## 🎉 You're Done!

Your app is now:
- ✅ Built automatically
- ✅ Released on GitHub
- ✅ Downloadable by users
- ✅ Easy to update

Share your release:
```
https://github.com/YOUR_USERNAME/fxsound-linux/releases/latest
```
