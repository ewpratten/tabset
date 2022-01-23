use std::path::PathBuf;

use colored::Colorize;
use directories::ProjectDirs;

/// Gets the configuration directory for the app
pub fn get_config_dir() -> Result<PathBuf, std::io::Error> {
    // Construct the path to the config directory for this app
    let project_dir = ProjectDirs::from("com", "va3zza", "tabset");

    // Only pass back the directory if it exists
    match project_dir {
        Some(dir) => {
            let config_dir = dir.config_dir();
            if config_dir.exists() {
                Ok(config_dir.to_path_buf())
            } else {
                eprintln!(
                    "{}\nPlease create: {}\nConfiguration info can be found at: {}",
                    "tabset not configured".red(),
                    config_dir.display().to_string().cyan(),
                    "https://github.com/Ewpratten/tabset#configuration".cyan()
                );
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Config directory not found",
                ))
            }
        }
        None => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Config directory not found",
        )),
    }
}

/// Gets the "friendly name" of a tablet from its config name
pub fn get_device_name(config_name: &str) -> Result<String, std::io::Error> {
    let config_dir = get_config_dir()?;
    return std::fs::read_to_string(config_dir.join("devices").join(config_name).join("name"));
}

/// Gets a list of peripheral names for a tablet from its config name.
///
/// These would be what XWacom will look for if we run a `check` command
pub fn get_device_peripherals(config_name: &str) -> Result<Vec<String>, std::io::Error> {
    let config_dir = get_config_dir()?;
    return std::fs::read_to_string(
        config_dir
            .join("devices")
            .join(config_name)
            .join("peripherals"),
    )
    .map(|peripherals| {
        peripherals
            .split('\n')
            .map(|p| p.to_owned())
            .collect::<Vec<_>>()
    });
}

/// Gets a list of all profiles for a tablet from its config name.
pub fn get_device_profiles(config_name: &str) -> Result<Vec<String>, std::io::Error> {
    let config_dir = get_config_dir()?;
    return std::fs::read_dir(
        config_dir
            .join("devices")
            .join(config_name)
            .join("profiles"),
    )
    .map(|profiles| {
        profiles
            .map(|profile| {
                profile
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_owned()
                    .strip_suffix(".sh")
                    .unwrap()
                    .to_owned()
            })
            .collect::<Vec<_>>()
    });
}

/// Gets the path to the script for a profile for a tablet from its config name and profile name
pub fn get_profile_script(
    config_name: &str,
    profile_name: &str,
) -> Result<PathBuf, std::io::Error> {
    let config_dir = get_config_dir()?;
    return Ok(config_dir
        .join("devices")
        .join(config_name)
        .join("profiles")
        .join(format!("{}.sh", profile_name)));
}
