# Maintainer: smtdfc <me.smtdfc@gmail.com>
pkgname=bakeryos-iso-builder
pkgver=1.0.0
pkgrel=1
pkgdesc="ISO Builder for BakeryOS"
arch=('x86_64')
url="https://github.com/bakeryos-project/bakeryos-iso-builder"
license=('GPL-3.0-or-later')
depends=('gcc-libs' 'glibc' 'pacman')
makedepends=('go' 'git')
source=()
sha256sums=()
options=(!debug !strip)

build(){
   cd $startdir
  go build -o ./dist/bakeryos-iso-builder  .
}

package() {
   install -Dm755 "$startdir/dist/bakeryos-iso-builder" "$pkgdir/usr/bin/bakeryos-iso-builder"
   install -Dm644 "$startdir/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}