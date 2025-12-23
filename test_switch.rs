// Simple test program to manually switch monitor inputs
// Usage: cargo run --bin test_switch <input_value>
// Example: cargo run --bin test_switch 17  (for HDMI1)

use ddc_hi::{Ddc, Display};

const INPUT_SELECT: u8 = 0x60;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input_value>", args[0]);
        eprintln!("\nCommon input values:");
        eprintln!("  17 (0x11) - HDMI 1");
        eprintln!("  18 (0x12) - HDMI 2");
        eprintln!("  19 (0x13) - HDMI 3");
        eprintln!("  20 (0x14) - HDMI 4");
        eprintln!("  15 (0x0F) - DisplayPort 1");
        eprintln!("  16 (0x10) - DisplayPort 2");
        std::process::exit(1);
    }

    let input_value: u16 = args[1].parse().expect("Invalid input value");

    println!("Scanning for DDC-compatible displays...");
    let displays = Display::enumerate();

    if displays.is_empty() {
        eprintln!("ERROR: No DDC-compatible displays found!");
        std::process::exit(1);
    }

    println!("Found {} display(s):", displays.len());

    for (i, mut display) in displays.into_iter().enumerate() {
        let display_id = &display.info.id;
        println!("\n[{}] Display: '{}'", i + 1, display_id);

        // Try to read current input
        print!("  Current input: ");
        match display.handle.get_vcp_feature(INPUT_SELECT) {
            Ok(current) => {
                println!("{} (0x{:02X})", current.value(), current.value());
            }
            Err(e) => {
                println!("ERROR reading: {:?}", e);
            }
        }

        // Try to switch input
        println!("  Switching to input {} (0x{:02X})...", input_value, input_value);
        match display.handle.set_vcp_feature(INPUT_SELECT, input_value) {
            Ok(_) => {
                println!("  ✓ Switch command sent successfully!");

                // Wait a bit and try to read the new value
                std::thread::sleep(std::time::Duration::from_secs(2));
                match display.handle.get_vcp_feature(INPUT_SELECT) {
                    Ok(new_value) => {
                        println!("  New input value: {} (0x{:02X})", new_value.value(), new_value.value());
                    }
                    Err(e) => {
                        println!("  Could not verify new input: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("  ✗ ERROR: Failed to switch input: {:?}", e);
            }
        }
    }

    println!("\n=== Testing Complete ===");
    println!("\nNote: Even if reading the current input fails, the switch command");
    println!("might still work. Watch your display to see if it actually switched.");
}
