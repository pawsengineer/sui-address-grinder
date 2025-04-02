#!/bin/bash

set -e 
set -o pipefail  

REPO_URL="https://github.com/pawsengineer/sui-address-grinder.git"
INSTALL_DIR="/usr/local/bin"
APP_NAME="sui-address-grinder"

# Build the project in release mode
echo "Building $APP_NAME..."
cargo build --release

# Install the binary
echo "Installing $APP_NAME to $INSTALL_DIR..."
sudo mv "target/release/$APP_NAME" "$INSTALL_DIR/$APP_NAME"

# Verify installation
echo "Installation completed. Verifying..."
"$INSTALL_DIR/$APP_NAME" --help || { echo "Installation failed!"; exit 1; }

echo "$APP_NAME installed successfully!"
