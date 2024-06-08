pkgname=vpn_handler
pkgver=1.0.1
pkgrel=1
makedepends=('rust' 'cargo')
depends=('wireguard-tools' 'openresolv')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')

build() {
    return 0
}

package() {
    cargo install --root="$pkgdir" vpn_handler
}
