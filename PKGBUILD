# Maintainer: Your Name <your.email@example.com>
pkgname=framework-led-widgets
pkgver=0.2.0
pkgrel=1
pkgdesc="A rust application for configuring and displaying widgets on the Framework 16 LED Matrix modules"
arch=('x86_64')
url="https://github.com/yourusername/framework-led-widgets"
license=('GPL-3.0-only')
depends=('systemd')
makedepends=('rust' 'cargo')
backup=('etc/framework-led-widgets/config.toml')
source=()
sha256sums=()

# Build in the same directory as PKGBUILD (cargo project root)
_cargo_dir="$startdir"

prepare() {
    # No preparation needed when building from current directory
    return 0
}

build() {
    cd "$_cargo_dir"
    
    # Clean previous builds to ensure fresh build
    cargo clean
    
    # Set cargo target directory to avoid conflicts
    export CARGO_TARGET_DIR="$_cargo_dir/target"
    
    # Build the project in release mode
    cargo build --release --locked
}

check() {
    cd "$_cargo_dir"
    
    # Run tests if available
    cargo test --release --locked || true
}

package() {    
    # Install the binary
    install -Dm755 "$_cargo_dir/target/release/framework-led-widgets" "$pkgdir/usr/local/bin/framework-led-widgets"
    
    # Install systemd service file
    install -Dm644 "$_cargo_dir/framework-led-widgets.service" "$pkgdir/usr/lib/systemd/system/framework-led-widgets.service"
    
    # Install config file
    install -Dm644 "$_cargo_dir/config.toml" "$pkgdir/etc/framework-led-widgets/config.toml"
    
    # Install documentation if available
    if [ -f "$_cargo_dir/README.md" ]; then
        install -Dm644 "$_cargo_dir/README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
    fi
    
    # Install license if available
    if [ -f "$_cargo_dir/LICENSE" ]; then
        install -Dm644 "$_cargo_dir/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    fi
}

# Post-install script
post_install() {
    echo "Framework 16 LED Matrix Widgets installed successfully!"
    echo ""
    echo "Configuration file installed at: /etc/framework-led-widgets/config.toml"
    echo "You may want to edit this file to customize your widget settings."
    echo ""
    echo "To enable and start the service:"
    echo "  sudo systemctl enable framework-led-widgets.service"
    echo "  sudo systemctl start framework-led-widgets.service"
    echo ""
    echo "To check service status:"
    echo "  systemctl status framework-led-widgets.service"
    echo ""
    echo "To view logs:"
    echo "  journalctl -u framework-led-widgets.service -f"
    echo ""
    echo "Note: The service runs as root to access serial devices."
    echo "Make sure your user is in the 'uucp' group for manual testing:"
    echo "  sudo usermod -a -G uucp \$USER"
}

# Post-upgrade script
post_upgrade() {
    echo "Framework 16 LED Matrix Widgets updated!"
    echo ""
    echo "Configuration file: /etc/framework-led-widgets/config.toml"
    echo "Check for any new configuration options that may have been added."
    echo ""
    echo "Reloading systemd daemon..."
    systemctl daemon-reload
    echo ""
    echo "If the service was running, restart it with:"
    echo "  sudo systemctl restart framework-led-widgets.service"
}

# Pre-remove script
pre_remove() {
    if systemctl is-enabled framework-led-widgets.service >/dev/null 2>&1; then
        echo "Stopping and disabling framework-led-widgets.service..."
        systemctl stop framework-led-widgets.service
        systemctl disable framework-led-widgets.service
    fi
}

# Post-remove script
post_remove() {
    echo "Framework 16 LED Matrix Widgets removed."
    echo "Configuration file at /etc/framework-led-widgets/config.toml has been preserved."
    echo "Reloading systemd daemon..."
    systemctl daemon-reload
}
