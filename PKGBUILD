# Maintainer: Gaurav Atreya <allmanpride@gmail.com>
pkgname=unit-calc
pkgver=0.1.0
pkgrel=1
pkgdesc="Simple GTK UI for gnu-units based calculation"
arch=('x86_64')
license=('GPL3')
depends=('gcc-libs' 'units')
makedepends=('rust' 'cargo')
OPTIONS=(strip !debug)

build() {
	cargo build --release
}

package() {
    cd "$srcdir"
    mkdir -p "$pkgdir/usr/bin"
    mkdir -p "$pkgdir/usr/share/applications"
    mkdir -p "$pkgdir/usr/share/unit-calc"
    cp "../target/release/${pkgname}" "$pkgdir/usr/bin/${pkgname}"
    cp "../icon.svg" "$pkgdir/usr/share/unit-calc/icon.svg"
    cp "../${pkgname}.desktop" "$pkgdir/usr/share/applications/${pkgname}.desktop"
}
