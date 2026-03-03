# Maintainer: Your Name <your.email@example.com>
pkgname=fxsound-linux
pkgver=1.0.0
pkgrel=1
pkgdesc="Audio enhancer for Linux with 10-band EQ and effects"
arch=('x86_64')
url="https://github.com/YOUR_USERNAME/fxsound-linux"
license=('MIT')
depends=('webkit2gtk' 'libpulse')
makedepends=('rust' 'npm' 'cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$pkgname-$pkgver"
  npm install
  npm run tauri:build
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm755 "src-tauri/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
