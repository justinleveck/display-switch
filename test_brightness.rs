// Test if we can control brightness via DDC/CI
// This will tell us if DDC/CI works at all, or if input switching specifically is blocked

use ddc_hi::{Ddc, Display};

const BRIGHTNESS: u8 = 0x10;
const INPUT_SELECT: u8 = 0x60;

fn main() {
    println!("=== Testing Basic DDC/CI Functionality ===\n");

    let displays = Display::enumerate();
    if displays.is_empty() {
        eprintln!("No displays found!");
        return;
    }

    for mut display in displays {
        println!("Display: '{}'", display.info.id);
        println!("{}", "-".repeat(50));

        // Test 1: Can we read brightness?
        print!("\n1. Reading brightness: ");
        match display.handle.get_vcp_feature(BRIGHTNESS) {
            Ok(value) => {
                let current = value.value();
                println!("✓ Current brightness: {}", current);

                // Test 2: Can we write brightness?
                let test_brightness = if current > 50 { current - 10 } else { current + 10 };
                println!("\n2. Testing brightness control...");
                println!("   Attempting to change brightness to {}", test_brightness);
                println!("   WATCH YOUR SCREEN - brightness should change!");

                match display.handle.set_vcp_feature(BRIGHTNESS, test_brightness) {
                    Ok(_) => {
                        println!("   ✓ Brightness command sent!");
                        std::thread::sleep(std::time::Duration::from_secs(2));

                        // Verify
                        match display.handle.get_vcp_feature(BRIGHTNESS) {
                            Ok(new_val) => {
                                println!("   New brightness reading: {}", new_val.value());
                                if new_val.value() == test_brightness {
                                    println!("   ✓✓ DDC/CI WRITE WORKS!");
                                } else {
                                    println!("   ⚠ Brightness changed but not to expected value");
                                }
                            }
                            Err(e) => println!("   Cannot verify: {:?}", e),
                        }

                        // Restore original brightness
                        println!("\n   Restoring original brightness...");
                        let _ = display.handle.set_vcp_feature(BRIGHTNESS, current);
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                    Err(e) => {
                        println!("   ✗ Failed to set brightness: {:?}", e);
                        println!("   This means DDC/CI writes are not working at all!");
                    }
                }
            }
            Err(e) => {
                println!("✗ Cannot read brightness: {:?}", e);
                println!("   This means DDC/CI reads are not working!");
            }
        }

        // Test 3: What about input select?
        println!("\n3. Testing input select feature:");
        print!("   Reading current input: ");
        match display.handle.get_vcp_feature(INPUT_SELECT) {
            Ok(value) => {
                println!("✓ Current input value: {} (0x{:02X})", value.value(), value.value());

                // Try to write the SAME value back (shouldn't change anything)
                println!("   Writing same input value back...");
                match display.handle.set_vcp_feature(INPUT_SELECT, value.value()) {
                    Ok(_) => println!("   ✓ Input select WRITE command accepted!"),
                    Err(e) => println!("   ✗ Input select WRITE failed: {:?}", e),
                }
            }
            Err(e) => {
                println!("✗ {:?}", e);
                println!("   Cannot read current input (this is expected based on earlier tests)");

                println!("\n   Trying to write input value anyway...");
                println!("   Attempting HDMI 1 (0x11)...");
                match display.handle.set_vcp_feature(INPUT_SELECT, 0x11) {
                    Ok(_) => {
                        println!("   ✓ Command sent! (WATCH YOUR SCREEN!)");
                        std::thread::sleep(std::time::Duration::from_secs(3));
                    }
                    Err(e) => println!("   ✗ Write failed: {:?}", e),
                }
            }
        }

        println!("\n{}", "=".repeat(50));
    }

    println!("\n=== Analysis ===");
    println!("If brightness control worked:");
    println!("  → DDC/CI is functional");
    println!("  → But input switching might be specifically disabled/unsupported");
    println!("\nIf brightness control did NOT work:");
    println!("  → DDC/CI writes are blocked entirely");
    println!("  → Your LG OLED may not support DDC/CI control");
    println!("  → You'll need to use LG's network API instead");
}
