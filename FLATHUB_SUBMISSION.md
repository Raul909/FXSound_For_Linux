# Flathub Submission Guide

## ✅ Pre-submission Checklist Complete

All files are ready and linter passed with no errors!

## 📦 Files Ready for Submission

- `com.fxsound.linux.yml` - Flatpak manifest (linter passed ✓)
- `com.fxsound.linux.metainfo.xml` - AppStream metadata
- `com.fxsound.linux.desktop` - Desktop entry
- `cargo-sources.json` - Offline Cargo dependencies (6520 lines)

## 🚀 Submission Steps

### 1. Fork Flathub Repository

Go to: https://github.com/flathub/flathub

Click "Fork" and **UNCHECK** "Copy the master branch only"

### 2. Clone Your Fork

```bash
git clone --branch=new-pr git@github.com:YOUR_GITHUB_USERNAME/flathub.git
cd flathub
```

### 3. Create Submission Branch

```bash
git checkout -b fxsound-linux-submission new-pr
```

### 4. Create App Directory

```bash
mkdir com.fxsound.linux
cd com.fxsound.linux
```

### 5. Copy Files

Copy these files from your project to the `com.fxsound.linux/` directory:

```bash
# From /var/home/raul/Documents/fxsound-app/ copy:
- com.fxsound.linux.yml
- com.fxsound.linux.metainfo.xml
- com.fxsound.linux.desktop
- cargo-sources.json
```

### 6. Commit and Push

```bash
cd ..  # Back to flathub root
git add com.fxsound.linux/
git commit -m "Add com.fxsound.linux"
git push origin fxsound-linux-submission
```

### 7. Open Pull Request

1. Go to: https://github.com/YOUR_GITHUB_USERNAME/flathub
2. Click "Pull Request"
3. **IMPORTANT:** Set base branch to `new-pr` (NOT master!)
4. Title: "Add com.fxsound.linux"
5. Fill in the template
6. Submit

## 📝 PR Template

```markdown
## Submission

- [ ] I have read and agree to the Generative AI policy
- [ ] I have tested the build locally
- [ ] Linter passed with no errors
- [ ] I have 2FA enabled on GitHub

## Application Details

**Name:** FXSound for Linux
**App ID:** com.fxsound.linux
**License:** MIT
**Homepage:** https://github.com/Raul909/FXSound_For_Linux

## Description

A free, open-source audio enhancer for Linux with 10-band equalizer, effects, presets, and real-time audio visualization. Linux alternative to FXSound.

## Testing

Built and tested locally with:
- flatpak-builder-lint: ✓ Passed
- Local build: ✓ Success
- Runtime: org.freedesktop.Platform 24.08
```

## ⏳ After Submission

1. **Wait for review** (reviewers are volunteers)
2. **Respond to comments** - Don't close the PR!
3. **Request test build** - Comment: `bot, build`
4. **Wait for approval** and merge
5. **Accept GitHub invitation** within 1 week (requires 2FA)

## 🔧 If Build Fails

The reviewers will test build. If it fails, they'll comment. Common issues:

- Missing dependencies
- Icon path issues
- Desktop file validation

Just push fixes to the same branch - don't close the PR!

## 📚 Resources

- Flathub Docs: https://docs.flathub.org
- Linter Docs: https://docs.flathub.org/linter
- Submission Guide: https://docs.flathub.org/docs/for-app-authors/submission

## ✨ Quick Commands

```bash
# If you have GitHub CLI:
gh repo fork --clone flathub/flathub && cd flathub
git checkout --track origin/new-pr
git checkout -b fxsound-linux-submission
mkdir com.fxsound.linux
# Copy files, then:
git add com.fxsound.linux/
git commit -m "Add com.fxsound.linux"
git push origin fxsound-linux-submission
gh pr create --base new-pr --title "Add com.fxsound.linux"
```

---

**Note:** I've prepared everything, but you need to:
1. Have a GitHub account with 2FA enabled
2. Fork the flathub/flathub repo
3. Submit the PR yourself

All files are ready in your project directory!
