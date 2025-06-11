#!/bin/bash

set -e

# Configuration
SERVICE_NAME="framework-led-widgets"
BINARY_NAME="framework-led-widgets"
SERVICE_FILE="${SERVICE_NAME}.service"
INSTALL_DIR="/usr/local/bin"
SYSTEMD_DIR="/etc/systemd/system"
CONFIG_DIR="/etc/framework-led-widgets"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running as root
if [[ $EUID -ne 0 ]]; then
    print_error "This script must be run as root (use sudo)"
    exit 1
fi

# Check if binary exists
if [[ ! -f "target/release/${BINARY_NAME}" ]]; then
    print_error "Binary not found at target/release/${BINARY_NAME}"
    print_error "Please build your project first with: cargo build --release"
    exit 1
fi

# Check if config file exists
if [[ ! -f "config.toml" ]]; then
    print_error "Config file config.toml not found in current directory"
    exit 1
fi

# Check if service file exists
if [[ ! -f "${SERVICE_FILE}" ]]; then
    print_error "Service file ${SERVICE_FILE} not found in current directory"
    exit 1
fi

print_status "Installing ${SERVICE_NAME} service..."

# Stop the service if it's running
if systemctl is-active --quiet "${SERVICE_NAME}"; then
    print_status "Stopping existing ${SERVICE_NAME} service..."
    systemctl stop "${SERVICE_NAME}"
fi

# Copy binary to install directory
print_status "Copying binary to ${INSTALL_DIR}/${BINARY_NAME}..."
cp "target/release/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

# Create config directory and copy config file
print_status "Installing config file to ${CONFIG_DIR}/config.toml..."
mkdir -p "${CONFIG_DIR}"
cp "config.toml" "${CONFIG_DIR}/config.toml"
chmod 644 "${CONFIG_DIR}/config.toml"

# Copy service file
print_status "Installing service file to ${SYSTEMD_DIR}/${SERVICE_FILE}..."
cp "${SERVICE_FILE}" "${SYSTEMD_DIR}/${SERVICE_FILE}"

# Reload systemd daemon
print_status "Reloading systemd daemon..."
systemctl daemon-reload

# Enable the service
print_status "Enabling ${SERVICE_NAME} service..."
systemctl enable "${SERVICE_NAME}"

# Start the service
print_status "Starting ${SERVICE_NAME} service..."
systemctl start "${SERVICE_NAME}"

# Check service status
if systemctl is-active --quiet "${SERVICE_NAME}"; then
    print_status "✓ ${SERVICE_NAME} service is running successfully!"
else
    print_error "✗ Failed to start ${SERVICE_NAME} service"
    print_status "Check service status with: systemctl status ${SERVICE_NAME}"
    print_status "Check logs with: journalctl -u ${SERVICE_NAME} -f"
    exit 1
fi

print_status "Installation complete!"
print_status ""
print_status "Configuration file: ${CONFIG_DIR}/config.toml"
print_status "You may want to edit this file to customize your widget settings."
print_status ""
print_status "Useful commands:"
print_status "  Check status: systemctl status ${SERVICE_NAME}"
print_status "  View logs:    journalctl -u ${SERVICE_NAME} -f"
print_status "  Stop service: systemctl stop ${SERVICE_NAME}"
print_status "  Start service: systemctl start ${SERVICE_NAME}"
print_status "  Restart service: systemctl restart ${SERVICE_NAME}"
print_status "  Disable service: systemctl disable ${SERVICE_NAME}"
print_status "  Edit config: sudo nano ${CONFIG_DIR}/config.toml"
