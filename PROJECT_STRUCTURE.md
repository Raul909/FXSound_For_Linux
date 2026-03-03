# Project Structure

```
fxsound-app/
├── src/                          # React frontend
│   ├── App.jsx                   # Main UI component
│   ├── main.jsx                  # React entry point
│   └── index.css                 # Global styles
│
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── lib.rs                # Tauri commands & app setup
│   │   └── audio.rs              # Audio engine & PulseAudio
│   ├── Cargo.toml                # Rust dependencies
│   ├── tauri.conf.json           # Tauri configuration
│   ├── build.rs                  # Build script
│   ├── capabilities/             # Tauri permissions
│   └── icons/                    # App icons
│
├── public/                       # Static assets
│   └── screenshots/              # App screenshots
│
├── docs/                         # Documentation
│   ├── README.md                 # Documentation index
│   ├── AUDIO_IMPLEMENTATION.md   # Audio processing details
│   ├── TESTING_AND_DISTRIBUTION.md  # Complete guide
│   ├── DISTRIBUTION_GUIDE.md     # Distribution strategies
│   ├── MIGRATION_PLAN.md         # Migration history
│   ├── MIGRATION_STATUS.md       # Migration status
│   ├── COMPLETE_SUMMARY.md       # Project summary
│   └── VERSION_UPDATE.md         # Version update notes
│
├── .github/                      # GitHub configuration
│   └── workflows/
│       └── release.yml           # Automated release workflow
│
├── scripts/                      # Helper scripts
│   ├── setup-deps.sh             # Install system dependencies
│   ├── test-locally.sh           # Test system readiness
│   ├── test-audio.sh             # Test audio system
│   ├── build-release.sh          # Build production version
│   ├── verify-version.sh         # Verify version consistency
│   ├── version-info.sh           # Display version info
│   └── show-guide.sh             # Display complete guide
│
├── README.md                     # Project overview
├── QUICK_START.md                # Quick start guide
├── VERSION.md                    # Version information
├── CHANGELOG.md                  # Version history
├── PKGBUILD                      # Arch Linux package
├── com.fxsound.linux.yml         # Flatpak manifest
├── package.json                  # Node.js dependencies
├── package-lock.json             # Locked dependencies
├── vite.config.js                # Vite configuration
├── eslint.config.js              # ESLint configuration
├── index.html                    # HTML entry point
└── .gitignore                    # Git ignore rules
```

## Key Files

### Frontend
- **src/App.jsx** - Main React component with UI and Tauri IPC
- **src/main.jsx** - React entry point
- **src/index.css** - Minimal global styles

### Backend
- **src-tauri/src/lib.rs** - Tauri command handlers
- **src-tauri/src/audio.rs** - Audio engine with PulseAudio integration
- **src-tauri/Cargo.toml** - Rust dependencies (libpulse, rustfft, etc.)
- **src-tauri/tauri.conf.json** - App configuration (window size, identifier, etc.)

### Configuration
- **package.json** - Node.js dependencies and scripts
- **vite.config.js** - Vite build configuration
- **tauri.conf.json** - Tauri app configuration

### Documentation
- **README.md** - Main project documentation
- **QUICK_START.md** - 5-minute quick start
- **docs/** - Detailed guides and references

### Scripts
- **scripts/** - All helper scripts for development and building

### Distribution
- **PKGBUILD** - Arch Linux package definition
- **com.fxsound.linux.yml** - Flatpak manifest
- **.github/workflows/release.yml** - Automated GitHub releases

## Build Output

When you run `npm run tauri:build`, output goes to:

```
src-tauri/target/release/bundle/
├── appimage/
│   └── fxsound-linux_1.0.0_amd64.AppImage
├── deb/
│   └── fxsound-linux_1.0.0_amd64.deb
└── rpm/
    └── fxsound-linux-1.0.0-1.x86_64.rpm
```

## Ignored Files

See `.gitignore` for files not tracked by Git:
- `node_modules/` - Node.js dependencies
- `dist/` - Vite build output
- `src-tauri/target/` - Rust build output
- Editor and OS files
