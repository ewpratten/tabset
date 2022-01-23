use colored::Colorize;

use crate::config::{get_config_dir, get_device_name, get_device_peripherals, get_device_profiles};

pub fn list_tablets() {
    // Get the config directory
    let config_dir = get_config_dir().unwrap();

    // Get all known tablets in the directory
    let tablets = std::fs::read_dir(config_dir.join("devices"))
        .unwrap()
        .map(|entry| entry.unwrap())
        .collect::<Vec<_>>();

    // Iterate through the devices, grabbing additional info as needed and printing
    for tablet in &tablets {
        let tablet_dir_name = tablet.file_name();
        let tablet_dir_name = tablet_dir_name.to_str().unwrap();

        // Get the tablet's name from its `name` file
        let name = get_device_name(tablet_dir_name).unwrap();
        println!("{}: {}", tablet_dir_name, name.green());

        // Get the tablet's peripheral profile from its `peripherals` file
        let peripherals = get_device_peripherals(tablet_dir_name).unwrap();
        println!(
            "  - Peripherals:\n{}",
            peripherals.iter().map(|p| format!("    - {}", p.to_string().cyan())).collect::<Vec<_>>().join("\n")
        );

        // Get the tablet's profiles from its `profiles` directory
        let profiles = get_device_profiles(tablet_dir_name).unwrap();
        println!(
            "  - Profiles:\n{}",
            profiles.iter().map(|p| format!("    - {}", p.to_string().cyan())).collect::<Vec<_>>().join("\n")
        );
    }
}
