pkgname="upspot"
pkgver=0.1.0
pkgrel=1
pkgdesc="Upload files to your PC, especially via hotspot network "
arch=('x86_64')
url="https://github.com/slow-cat/upload"
license=('MIT' 'Apache')
depends=()
makedepends=('rust' 'cargo' 'git')
source=("git+https://github.com/slow-cat/upload.git")
sha256sums=('SKIP')
build() {
    echo '$(pwd)'
    cd "$srcdir/upload"
    echo '$(pwd)'
    cargo build --release  --locked
}

package() {
    install -Dm755 "$srcdir/upload/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}

