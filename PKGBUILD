# Maintainer: madratzz
pkgname=google-chat-desktop
pkgver=1.0.2
pkgrel=1
pkgdesc='A Tauri desktop wrapper for Google Chat'
arch=('x86_64')
url='https://github.com/madratzz/tauri-google-chat'
license=('MIT')
depends=('webkit2gtk-4.1' 'libayatana-appindicator')
makedepends=('rust' 'cargo' 'nodejs' 'npm' 'base-devel')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "tauri-google-chat-$pkgver"
    npm ci
    npm run tauri build
}

package() {
    cd "tauri-google-chat-$pkgver"

    install -Dm755 "src-tauri/target/release/google-chat-desktop" \
        "$pkgdir/usr/bin/google-chat-desktop"

    install -Dm644 "src-tauri/icons/icon.png" \
        "$pkgdir/usr/share/pixmaps/google-chat-desktop.png"

    mkdir -p "$pkgdir/usr/share/applications"
    cat > "$pkgdir/usr/share/applications/google-chat-desktop.desktop" << 'EOF'
[Desktop Entry]
Name=Google Chat Desktop
Exec=google-chat-desktop
Icon=google-chat-desktop
Type=Application
Categories=Network;InstantMessaging;
Comment=A Tauri desktop wrapper for Google Chat
EOF
}
