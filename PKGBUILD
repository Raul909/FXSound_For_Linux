# Maintainer: Your Name <your.email@example.com>
pkgname=fxsound-linux
pkgver=1.0.0
pkgrel=1
pkgdesc="Audio enhancer with 10-band EQ and effects"
arch=('x86_64')
url="https://github.com/Raul909/FXSound_For_Linux"
license=('MIT')
depends=('webkit2gtk' 'gtk3' 'libpulse')
makedepends=('rust' 'cargo' 'npm' 'git')
source=("$pkgname-$pkgver.tar.gz::https://github.com/Raul909/FXSound_For_Linux/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
    cd "$srcdir/FXSound_For_Linux-$pkgver"
    npm install
}

build() {
    cd "$srcdir/FXSound_For_Linux-$pkgver"
    npm run build
    cd src-tauri
    cargo build --release --locked
}

package() {
    cd "$srcdir/FXSound_For_Linux-$pkgver"
    
    # Install binary
    install -Dm755 "src-tauri/target/release/fxsound-linux" "$pkgdir/usr/bin/fxsound-linux"
    
    # Install desktop file
    install -Dm644 "com.fxsound.linux.desktop" "$pkgdir/usr/share/applications/com.fxsound.linux.desktop"
    
    # Install icon
    install -Dm644 "src-tauri/icons/128x128.png" "$pkgdir/usr/share/icons/hicolor/128x128/apps/com.fxsound.linux.png"
    
    # Install license
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
