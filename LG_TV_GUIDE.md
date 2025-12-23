# LG TV / OLED Support via Network API

## The Problem with LG TVs and DDC/CI

Modern LG OLED and LCD TVs are often detected via DDC/CI but **do not respond properly to input switching commands**. Common issues include:
- DDC/CI reads fail with "invalid DDC/CI length" errors
- Input switch commands are sent but the TV doesn't actually change inputs
- Pairing is required but doesn't persist

**Solution**: Use LG's webOS **network API** instead, which is more reliable and actually supported by LG.

---

## Quick Setup Guide

### Prerequisites

- LG webOS TV (most models from 2014+)
- TV and Mac on the same WiFi network
- Python 3 (comes with macOS)

### Step 1: Install Dependencies

Run the setup script:

```bash
./setup_lg_network.sh
```

This creates a Python virtual environment and installs the `bscpylgtv` library.

### Step 2: Enable Network Control on Your TV

On your LG TV:
1. Press **Settings** button on remote
2. Navigate to: **Connection → LG Connect Apps**
3. **Enable** it
4. Also enable: **Connection → HDMI Settings → SIMPLINK (HDMI-CEC)**

### Step 3: Find Your TV's IP Address

On your LG TV:
1. Press **Settings**
2. Go to **Network → Network Status** (or **Connection → WiFi Connection**)
3. Note the IP address (e.g., `192.168.1.100`)

### Step 4: Initial Pairing

List available inputs (replace with your TV's IP):

```bash
./lg_switch 192.168.1.100 list
```

**A pairing prompt will appear on your TV** - press **Yes** to accept it.

The script will show all available inputs:
```
Available inputs on your LG TV:
==================================================
Use these input names with the 'switch' command:

  • HDMI 1               → HDMI_1
  • HDMI 2               → HDMI_2
  • HDMI 3               → HDMI_3
  • HDMI 4               → HDMI_4
==================================================
```

Credentials are saved to `~/.lg_tv_keys.json` - future commands won't require pairing!

### Step 5: Test Input Switching

Try switching between inputs:

```bash
./lg_switch 192.168.1.100 switch HDMI_1
./lg_switch 192.168.1.100 switch HDMI_2
```

Your TV should immediately switch inputs!

### Step 6: Find Your USB Device ID

Find the USB device you want to monitor (your USB switch):

```bash
# List current USB devices
system_profiler SPUSBDataType | grep "Product ID" > /tmp/before.txt

# NOW: Unplug or press your USB switch button

# List again
system_profiler SPUSBDataType | grep "Product ID" > /tmp/after.txt

# See what changed
diff /tmp/before.txt /tmp/after.txt
```

Look for lines with `Vendor ID` and `Product ID` like:
```
Vendor ID: 0x046d
Product ID: 0xc52b
```

Your USB device ID is: `046d:c52b` (without the `0x` prefix)

### Step 7: Configure Automatic Switching

Edit `~/Library/Preferences/display-switch.ini`:

```ini
# Your USB device to monitor
usb_device = "046d:c52b"  # Replace with YOUR device ID

# When USB device connects, switch to HDMI 1
on_usb_connect_execute = "/Users/YOUR_USERNAME/code/display-switch/lg_switch 192.168.1.100 switch HDMI_1"

# When USB device disconnects, switch to HDMI 2
on_usb_disconnect_execute = "/Users/YOUR_USERNAME/code/display-switch/lg_switch 192.168.1.100 switch HDMI_2"
```

**Important**: Replace:
- `YOUR_USERNAME` with your actual username
- `192.168.1.100` with your TV's IP address
- `046d:c52b` with your USB device ID
- `HDMI_1` and `HDMI_2` with the correct inputs for your setup

### Step 8: Run display-switch

Build if you haven't already:
```bash
cargo build --release
```

Run the monitor:
```bash
./target/release/display_switch --debug
```

**Done!** Now when you press your USB switch button, your LG TV will automatically switch inputs! 🎉

---

## Configuration Examples

### Example 1: Two MacBooks on Same TV

Work MacBook on HDMI 4, Personal MacBook on HDMI 2:

```ini
usb_device = "046d:c52b"
on_usb_connect_execute = "/Users/john/code/display-switch/lg_switch 192.168.1.100 switch HDMI_4"
on_usb_disconnect_execute = "/Users/john/code/display-switch/lg_switch 192.168.1.100 switch HDMI_2"
```

### Example 2: With Additional Commands

Run extra commands when switching (e.g., wake displays):

```ini
usb_device = "046d:c52b"
on_usb_connect_execute = "/Users/john/code/display-switch/lg_switch 192.168.1.100 switch HDMI_1 && caffeinate -u -t 1"
on_usb_disconnect_execute = "/Users/john/code/display-switch/lg_switch 192.168.1.100 switch HDMI_2"
```

### Example 3: Multiple TVs

If you have multiple LG TVs:

```ini
usb_device = "046d:c52b"
on_usb_connect_execute = "/Users/john/code/display-switch/lg_switch 192.168.1.100 switch HDMI_1 && /Users/john/code/display-switch/lg_switch 192.168.1.101 switch HDMI_1"
```

---

## Running on Startup (macOS)

To have display-switch run automatically on login:

### Option 1: LaunchAgent (Recommended)

Create `~/Library/LaunchAgents/com.user.display-switch.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.user.display-switch</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Users/YOUR_USERNAME/code/display-switch/target/release/display_switch</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/display-switch.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/display-switch.error.log</string>
</dict>
</plist>
```

Load it:
```bash
launchctl load ~/Library/LaunchAgents/com.user.display-switch.plist
```

### Option 2: Login Items

1. Open **System Settings → General → Login Items**
2. Click **+** under "Open at Login"
3. Navigate to and select: `/Users/YOUR_USERNAME/code/display-switch/target/release/display_switch`

---

## Troubleshooting

### TV Not Found / Connection Refused

**Check:**
1. TV and Mac are on the same WiFi network
2. Try pinging the TV: `ping 192.168.1.100`
3. "LG Connect Apps" is enabled on TV
4. TV is powered on (not in standby)

**Fix:**
- Disable and re-enable "LG Connect Apps" on TV
- Restart your TV
- Check TV's firewall settings (some models have this)

### Pairing Prompt Appears Every Time

**Problem:** Client key not being saved/loaded.

**Fix:**
1. Check if `~/.lg_tv_keys.json` exists and has content
2. Make sure the script has write permissions to your home directory
3. Re-run: `./lg_switch 192.168.1.100 list` and accept pairing

### "Module bscpylgtv not found"

**Problem:** Virtual environment not activated.

**Fix:**
```bash
# Re-run setup
./setup_lg_network.sh

# Or manually activate venv
source venv/bin/activate
pip install bscpylgtv
```

### Input Switches But TV Shows "No Signal"

**Problem:** You're switching to an input with no device connected.

**Fix:**
- Verify which physical HDMI port your devices are connected to
- Run `./lg_switch 192.168.1.100 list` and test each input
- Make sure your config uses the correct input IDs

### Switching is Slow

**Normal behavior:** LG TVs take 1-2 seconds to switch inputs via network API.

**If it's slower:**
- Check WiFi signal strength on TV
- Reduce network congestion
- Consider wired Ethernet connection for TV

---

## How It Works

1. **USB Detection**: The Rust `display_switch` binary monitors USB device connect/disconnect events
2. **Script Execution**: When your USB device connects/disconnects, it runs the `lg_switch` Python script
3. **Network API**: The Python script connects to your LG TV over WiFi using webOS protocol
4. **Input Switch**: TV receives the input change command and switches immediately

### Architecture

```
USB Switch Button Press
        ↓
[display_switch] (Rust binary)
    Detects USB event
        ↓
[lg_switch] (Python wrapper)
        ↓
[bscpylgtv library]
    Connects via WebSocket
        ↓
[LG TV webOS API]
    Port 3000/3001
        ↓
TV switches input!
```

---

## Advantages Over DDC/CI

✅ **Actually works** on modern LG TVs (DDC/CI often doesn't)
✅ More reliable and officially supported by LG
✅ Faster response time
✅ Works regardless of cable type (USB-C, HDMI, etc.)
✅ No adapter compatibility issues
✅ Can control other TV features (volume, power, etc.)
✅ Works over WiFi (no direct cable to TV needed)

---

## Advanced Usage

### Manual Control Script

You can use `lg_switch` standalone without `display_switch`:

```bash
# List inputs
./lg_switch 192.168.1.100 list

# Switch to specific input
./lg_switch 192.168.1.100 switch HDMI_1

# Create aliases for quick access
alias tv-hdmi1='./lg_switch 192.168.1.100 switch HDMI_1'
alias tv-hdmi2='./lg_switch 192.168.1.100 switch HDMI_2'
```

### Integration with Other Tools

The network API script can be integrated with:
- **Keyboard Maestro**: Trigger input switches with keyboard shortcuts
- **Alfred Workflows**: Quick input switching via Alfred
- **Home Assistant**: Integrate with smart home automation
- **AppleScript**: Control from other macOS apps
- **Shell Scripts**: Build custom automation

### Example: Keyboard Shortcut

Create a shell script `~/bin/tv-work.sh`:
```bash
#!/bin/bash
~/code/display-switch/lg_switch 192.168.1.100 switch HDMI_1
```

Bind it to a keyboard shortcut using System Settings or Keyboard Maestro.

---

## Technical Details

### Network Protocol

- **Protocol**: WebSocket over TCP
- **Port**: 3000 (ws://) or 3001 (wss://)
- **Authentication**: Client key-based (persists after pairing)
- **Discovery**: Can use SSDP for automatic TV discovery

### Input IDs

Common LG webOS input names for switching:
- HDMI 1: `HDMI_1`
- HDMI 2: `HDMI_2`
- HDMI 3: `HDMI_3`
- HDMI 4: `HDMI_4`

Note: The `get_inputs()` API returns full app IDs like `com.webos.app.hdmi1`, but the `set_input()` API requires the simpler format like `HDMI_1`. The `lg_switch` script handles this correctly.

### Python Library

This solution uses [bscpylgtv](https://github.com/chros73/bscpylgtv), an enhanced fork of aiopylgtv with:
- Faster connection handling
- Better error handling
- Additional calibration features
- Improved stability

---

## Alternative Solutions

If the network API doesn't work for your setup:

### 1. HDMI-CEC (SIMPLINK)
- Uses HDMI control signals
- Requires SIMPLINK enabled
- Limited to HDMI-connected devices

### 2. IR Blaster
- Uses infrared remote control signals
- Requires USB IR blaster hardware
- Works on any TV with IR remote

### 3. Physical KVM Switch
- Hardware solution
- No software needed
- More expensive

---

## Compatible TV Models

This solution works with most LG webOS TVs, including:

- **OLED Series**: CX, C1, C2, C3, G1, G2, G3, etc.
- **NanoCell Series**: Most 2018+ models
- **UHD Series**: Most 2016+ models
- **Older webOS**: Models from 2014+

**Check compatibility**: If your LG TV has the "LG Connect Apps" option in settings, it supports the network API!

---

## Credits

- **display-switch**: [haimgel/display-switch](https://github.com/haimgel/display-switch) - USB event monitoring
- **bscpylgtv**: [chros73/bscpylgtv](https://github.com/chros73/bscpylgtv) - LG TV network control library
- **LG webOS**: [LG Developer Portal](https://webostv.developer.lge.com/) - Official API documentation

---

## Support

For issues specific to:
- **USB detection**: See main [display-switch README](README.md)
- **LG network API**: Check [bscpylgtv documentation](https://github.com/chros73/bscpylgtv)
- **This setup**: Open an issue describing your TV model and problem

---

## License

This extension follows the same MIT license as the main display-switch project.
