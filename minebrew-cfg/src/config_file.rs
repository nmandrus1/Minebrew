use anyhow::Result;
use serde::Deserialize;
use etcetera::base_strategy::{choose_base_strategy, BaseStrategy};

use std::path::PathBuf;
use home::home_dir;

const DEFAULT_MC_VERSION: &'static str = "1.19";

#[derive(Deserialize)]
pub struct ConfigFile {
    pub target: Option<String>,
    pub mc_dir: PathBuf,
}

// Conditional Compilation: 
// This default function is for unix systems only
#[cfg(target_family = "unix")]
impl Default for ConfigFile {
    fn default() -> Self {
        // attempt to locate home_directory
        let mut mc_dir = match home_dir() {
            Some(d) => d,
            None => {
                eprintln!("Impossible to locate home directory...");
                std::process::exit(1);
            }
        };

        mc_dir.push(".minecraft");

        Self { 
            target: Some(String::from(DEFAULT_MC_VERSION)),
            mc_dir,
        } 
    }
}

#[cfg(target_family = "windows")]
impl Default for ConfigFile {
    fn default() -> Self {
        // attempt to locate home_directory
        let mut mc_dir = match home_dir() {
            Some(d) => d,
            None => {
                eprintln!("Impossible to locate home directory...");
                std::process::exit(1);
            }
        };

        mc_dir.push("AppData");
        mc_dir.push(".minecraft");

        Self { 
            target: String::from(DEFAULT_MC_VERSION),
            mc_dir,
        } 
    }
}

/// Function that finds and tries to load config file
/// ex location: "~/.config/mbrew/config.toml"
pub fn load_config_file() -> Result<ConfigFile> {
    let strat = choose_base_strategy() 
        .expect("Unable to locate config directory..."); // locate config dir

    let mut config_dir = strat.config_dir();         // append "mbrew" to path so it
    config_dir.push("minebrew");                        // now holds the full path to the dir 


    if !config_dir.exists() {                         // if the config directory does not exist
        // std::fs::create_dir_all(&config_dir).ok(); // create it at default configuration directory
        return Ok(ConfigFile::default());             // Just use default Options
    }

    // attempt to read config.toml to string handling 
    // specific errors by loading a default configuration
    let config_file = match std::fs::read_to_string(config_dir.join("config.toml")) { 
        Ok(cfg) => {  
            // if the file was read successfully try to parse it
            toml::from_str(&cfg)
                .unwrap_or_else(|e| { // if can't be parsed print err msg and exit
                    eprintln!("Error in config file: {}", e);
                    std::process::exit(1);
                })
        },
        // Cool syntax I didn't know about 
        // if the file isnt found then use default config otherwise return an error
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => ConfigFile::default(),
        Err(err) =>  return Err(anyhow::Error::from(err)),
    };

    Ok(config_file)
    }
