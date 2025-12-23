#!/bin/bash
# Setup script for LG Network API control

set -e

echo "=== LG Network API Setup ==="
echo ""

# Create a virtual environment in the project
if [ ! -d "venv" ]; then
    echo "Creating Python virtual environment..."
    python3 -m venv venv
    echo "✓ Virtual environment created"
else
    echo "✓ Virtual environment already exists"
fi

# Activate and install
echo ""
echo "Installing bscpylgtv library..."
source venv/bin/activate
pip install --upgrade pip
pip install bscpylgtv

echo ""
echo "✓ Setup complete!"
echo ""
echo "=== Quick Test ==="
echo ""
echo "Run this to test (I've already set your TV IP):"
echo "  ./venv/bin/python3 lg_network_switch.py 192.168.50.144 list"
echo ""
echo "Or activate the virtual environment first:"
echo "  source venv/bin/activate"
echo "  ./lg_network_switch.py 192.168.50.144 list"
echo ""
echo "First time: Accept the pairing prompt on your TV!"
echo ""
