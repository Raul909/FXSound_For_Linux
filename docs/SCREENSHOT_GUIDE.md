# Screenshot Guide

## Quick Instructions

To update the screenshots with the new UI design:

### 1. Start the App
```bash
npm run tauri:dev
```

### 2. Take Screenshots

**For Equalizer Screenshot:**
1. Click on "EQUALIZER" tab
2. Select "Music" preset
3. Adjust a few sliders to show variety (e.g., boost bass at 32Hz, 64Hz)
4. Make sure power button is ON (glowing red)
5. Wait for visualizer to show activity
6. Take screenshot

**For Effects Screenshot:**
1. Click on "EFFECTS" tab
2. Select "Movies" preset (shows good effect values)
3. Make sure power button is ON
4. Visualizer should be active
5. Take screenshot

### 3. Screenshot Tools

**Option A: GNOME Screenshot (Ubuntu/Pop!_OS/Fedora)**
```bash
gnome-screenshot -w
# Then click on the FXSound window
```

**Option B: Spectacle (KDE/Kubuntu)**
```bash
spectacle -a
```

**Option C: Flameshot (Universal)**
```bash
flameshot gui
# Select the window area
```

**Option D: scrot (Command line)**
```bash
scrot -u 'screenshot-eq.png' -d 3
# Click on FXSound window within 3 seconds
```

### 4. Save Screenshots

Save the files as:
- `public/screenshots/screenshot-eq.png` (Equalizer tab)
- `public/screenshots/screenshot-effects.png` (Effects tab)

### 5. Optimize (Optional)

```bash
# Install optipng if not installed
sudo apt install optipng  # Ubuntu/Debian
sudo dnf install optipng  # Fedora

# Optimize PNG files
optipng -o7 public/screenshots/*.png
```

## Screenshot Specifications

- **Format:** PNG
- **Recommended width:** 800-1000px
- **Background:** Keep the dark background visible
- **Window state:** Normal (not maximized)
- **Quality:** High resolution, no compression artifacts

## Tips

- Use a clean desktop background (dark preferred)
- Make sure the window is fully visible
- Capture with some padding around the window
- Ensure good lighting/contrast
- Show the app in an active state (power ON, visualizer moving)
