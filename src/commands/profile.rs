use colored::Colorize;

use crate::config::{get_device_profiles, get_profile_script};

pub fn set_profile(tablet_name: &str, profile_name: &str) {
    // Get all valid profiles for the tablet
    let tablet_profiles = get_device_profiles(tablet_name).unwrap();

    // Check if the profile is valid
    if !tablet_profiles.contains(&profile_name.to_string()) {
        println!(
            "{} is not a valid profile for {}",
            profile_name, tablet_name
        );
        return;
    }

    // Set the profile
    println!(
        "Setting tablet {} to use profile {}",
        tablet_name, profile_name.green()
    );
    let profile_script = get_profile_script(tablet_name, profile_name).unwrap();
    let _output = std::process::Command::new("bash")
        .arg("-exc")
        .arg(profile_script)
        .spawn()
        .unwrap();
}
