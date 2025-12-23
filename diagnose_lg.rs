// Comprehensive DDC/CI diagnostic tool for LG OLED
// This will test various DDC/CI features to see what works

use ddc_hi::{Ddc, Display};

const INPUT_SELECT: u8 = 0x60;
const BRIGHTNESS: u8 = 0x10;
const CONTRAST: u8 = 0x12;
const POWER_MODE: u8 = 0xD6;

fn main() {
    println!("=== LG OLED DDC/CI Comprehensive Diagnostic ===\n");

    println!("Scanning for DDC-compatible displays...");
    let displays = Display::enumerate();

    if displays.is_empty() {
        eprintln!("ERROR: No DDC-compatible displays found!");
        std::process::exit(1);
    }

    println!("Found {} display(s)\n", displays.len());

    for (i, mut display) in displays.into_iter().enumerate() {
        let display_id = &display.info.id;
        println!("{}", "=".repeat(60));
        println!("Display #{}: '{}'", i + 1, display_id);
        println!("{}\n", "=".repeat(60));

        // Test 1: Try reading various VCP features
        println!("Test 1: Reading VCP Features");
        println!("{}", "-".repeat(40));

        let features = vec![
            (BRIGHTNESS, "Brightness (0x10)"),
            (CONTRAST, "Contrast (0x12)"),
            (INPUT_SELECT, "Input Select (0x60)"),
            (POWER_MODE, "Power Mode (0xD6)"),
        ];

        for (feature_code, feature_name) in features {
            print!("  {}: ", feature_name);
            match display.handle.get_vcp_feature(feature_code) {
                Ok(value) => {
                    println!("✓ Value: {} (0x{:02X})", value.value(), value.value());
                }
                Err(e) => {
                    println!("✗ Error: {:?}", e);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // Test 2: Try setting input to various values
        println!("\nTest 2: Testing Input Switching");
        println!("{}", "-".repeat(40));
        println!("Trying different input values...\n");

        let input_tests = vec![
            (0x0F, "DisplayPort 1"),
            (0x10, "DisplayPort 2"),
            (0x11, "HDMI 1"),
            (0x12, "HDMI 2"),
            (0x13, "HDMI 3"),
            (0x14, "HDMI 4"),
        ];

        for (value, name) in input_tests {
            println!("  Attempting to switch to {} (0x{:02X})...", name, value);

            match display.handle.set_vcp_feature(INPUT_SELECT, value) {
                Ok(_) => {
                    println!("    ✓ Command sent successfully");
                    println!("    ⏳ Waiting 3 seconds... (WATCH YOUR DISPLAY!)");
                    std::thread::sleep(std::time::Duration::from_secs(3));

                    // Try to verify
                    print!("    Verifying: ");
                    match display.handle.get_vcp_feature(INPUT_SELECT) {
                        Ok(new_val) => {
                            if new_val.value() == value {
                                println!("✓ Confirmed at {} (0x{:02X})", name, value);
                            } else {
                                println!("Value is now {} (0x{:02X}) - different from expected",
                                    new_val.value(), new_val.value());
                            }
                        }
                        Err(e) => {
                            println!("Cannot verify (read error: {:?})", e);
                        }
                    }
                }
                Err(e) => {
                    println!("    ✗ Failed to send command: {:?}", e);
                }
            }
            println!();
        }

        // Test 3: Try raw input values (some displays use non-standard values)
        println!("Test 3: Testing Raw Input Values");
        println!("{}", "-".repeat(40));
        println!("Some displays use non-standard input codes.\n");

        let raw_tests = vec![
            (0x01, "Raw 0x01"),
            (0x02, "Raw 0x02"),
            (0x03, "Raw 0x03"),
            (0x04, "Raw 0x04"),
        ];

        for (value, name) in raw_tests {
            println!("  Testing {} (0x{:02X})...", name, value);
            match display.handle.set_vcp_feature(INPUT_SELECT, value) {
                Ok(_) => {
                    println!("    ✓ Command accepted");
                    std::thread::sleep(std::time::Duration::from_secs(2));
                }
                Err(e) => {
                    println!("    ✗ Error: {:?}", e);
                }
            }
        }

        println!("\n{}\n", "=".repeat(60));
    }

    println!("\n=== Diagnostic Complete ===\n");
    println!("IMPORTANT QUESTIONS:");
    println!("1. Did you see your display switch inputs at any point?");
    println!("2. Did you see any 'on-screen display' messages from your TV?");
    println!("3. Check your LG OLED settings:");
    println!("   - Look for 'SIMPLINK (HDMI-CEC)' - try enabling/disabling");
    println!("   - Look for any 'External Control' or 'Network' settings");
    println!("   - Some LG OLEDs require enabling control via network/app settings");
}
