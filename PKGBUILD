pkgname=drink-water-reminder
pkgver=0.0.4
pkgrel=1
pkgdesc="Hydration reminder"
arch=('x86_64')
url="https://github.com/cowbones/drink-water-reminder"
license=('MIT')
depends=('libnotify')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$pkgname-$pkgver"
    
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 "assets/icon.png" "$pkgdir/usr/share/icons/hicolor/128x128/apps/$pkgname.png"
    install -Dm644 "drink-water-reminder.service" "$pkgdir/usr/lib/systemd/user/drink-water-reminder.service"
}
