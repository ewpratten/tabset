use std::path::PathBuf;

use colored::Colorize;
use directories::ProjectDirs;

pub fn get_config_dir() -> Result<PathBuf, ()> {
    // Construct the path to the config directory for this app
    let project_dir = ProjectDirs::from("com", "va3zza", "tabset");

    // Only pass back the directory if it exists
    match project_dir {
        Some(dir) => {
            let config_dir = dir.config_dir();
            if config_dir.exists() {
                Ok(config_dir.to_path_buf())
            } else {
                eprintln!("{}\nPlease create: {}\nConfiguration info can be found at: ", 
                    "tabset requires configuration files to be placed in its config directory.\nThis does not exist.".red(), 
                    config_dir.display()
                );
                Err(())
            }
        }
        None => Err(()),
    }
}
