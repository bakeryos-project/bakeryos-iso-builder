# Maintainer: smtdfc <me.smtdfc@gmail.com>
pkgname=bakeryos-iso-builder
pkgver=1.0.0
pkgrel=1
pkgdesc="ISO Builder for BakeryOS"
arch=('x86_64')
url="https://github.com/bakeryos-project/bakeryos-iso-builder"
license=('GPL-3.0-or-later')
depends=('gcc-libs' 'glibc' 'pacman')
makedepends=('cargo' 'git')
source=()
sha256sums=()
options=(!debug !strip)

build(){
 cd $startdir
 cargo build --release
}

package() {
   install -Dm755 "$startdir/target/release/bakeryos-iso-builder" "$pkgdir/usr/bin/bakeryos-iso-builder"
   install -Dm644 "$startdir/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
