#!/bin/bash
# Setup script for the second MacBook
# Run this on your Personal MacBook

set -e

echo "=========================================="
echo "Display-Switch Setup for Second MacBook"
echo "=========================================="
echo ""

# Get user info
read -p "Enter your username on THIS Mac: " USERNAME
read -p "Enter the LG TV IP address [192.168.50.144]: " TV_IP
TV_IP=${TV_IP:-192.168.50.144}

read -p "Which HDMI input is THIS Mac connected to? [2]: " THIS_HDMI
THIS_HDMI=${THIS_HDMI:-2}

read -p "Which HDMI input is the OTHER Mac connected to? [4]: " OTHER_HDMI
OTHER_HDMI=${OTHER_HDMI:-4}

echo ""
echo "Configuration:"
echo "  This Mac's HDMI: $THIS_HDMI"
echo "  Other Mac's HDMI: $OTHER_HDMI"
echo "  TV IP: $TV_IP"
echo ""
read -p "Is this correct? (yes/no): " CONFIRM

if [[ "$CONFIRM" != "yes" ]]; then
    echo "Aborted."
    exit 1
fi

echo ""
echo "Step 1: Building display-switch..."
if [ ! -f "target/release/display_switch" ]; then
    cargo build --release
    echo "✓ Built"
else
    echo "✓ Already built"
fi

echo ""
echo "Step 2: Setting up Python environment..."
./setup_lg_network.sh

echo ""
echo "Step 3: Pairing with LG TV..."
echo "You'll see a pairing prompt on your TV - accept it!"
echo ""
read -p "Press Enter to continue..."
./lg_switch $TV_IP list

echo ""
echo "Step 4: Creating configuration file..."

CONFIG_FILE="$HOME/Library/Preferences/display-switch.ini"
PROJECT_DIR=$(pwd)

cat > "$CONFIG_FILE" <<EOF
# Display-Switch Configuration for Personal MacBook
# Monitors USB-C hub connection to switch TV inputs automatically

# Monitor the AX88179A ethernet adapter in your USB-C hub
usb_device = "0b95:1790"

# When hub connects to THIS Mac, switch TV to HDMI $THIS_HDMI
on_usb_connect_execute = "$PROJECT_DIR/lg_switch $TV_IP switch com.webos.app.hdmi$THIS_HDMI"

# When hub disconnects from THIS Mac, switch TV to HDMI $OTHER_HDMI (other Mac)
on_usb_disconnect_execute = "$PROJECT_DIR/lg_switch $TV_IP switch com.webos.app.hdmi$OTHER_HDMI"
EOF

echo "✓ Configuration created at: $CONFIG_FILE"

echo ""
echo "Step 5: Testing..."
echo "Testing switch to HDMI $THIS_HDMI..."
./lg_switch $TV_IP switch com.webos.app.hdmi$THIS_HDMI

echo ""
echo "=========================================="
echo "✓ Setup Complete!"
echo "=========================================="
echo ""
echo "Next steps:"
echo ""
echo "1. Run display-switch in debug mode to test:"
echo "   ./target/release/display_switch --debug"
echo ""
echo "2. Test by moving your USB-C cable between Macs"
echo "   The TV should automatically switch inputs!"
echo ""
echo "3. To run on startup, add to Login Items:"
echo "   System Settings → General → Login Items"
echo "   Add: $PROJECT_DIR/target/release/display_switch"
echo ""
echo "Happy switching! 🎉"
