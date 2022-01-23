use std::io::BufRead;

use colored::Colorize;

use crate::config::get_device_peripherals;

pub fn check_tablet(tablet_name: &str) -> i32 {
    // Get the peripheral list for the tablet
    let peripherals = get_device_peripherals(tablet_name).unwrap();

    // Use `xsetwacom` to check if the tablet is connected
    let xsetwacom = std::process::Command::new("xsetwacom")
        .arg("--list")
        .output()
        .unwrap();
    let xsetwacom_output = xsetwacom.stdout;

    // Check if the tablet is connected
    let mut is_connected = false;
    for line in xsetwacom_output.lines() {
        let line = line.unwrap();
        for peripheral in &peripherals {
            if line.contains(peripheral) {
                is_connected = true;
                break;
            }
        }
    }

    // Print the message and return the status code
    if is_connected {
        println!("{} is {}", tablet_name, "connected".green());
        0
    } else {
        println!("{} is {}", tablet_name, "not connected".red());
        1
    }
}
