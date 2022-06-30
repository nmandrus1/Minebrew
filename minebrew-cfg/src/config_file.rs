use super::{valid_target_string, get_mc_dir}; 

use serde::Deserialize;

use std::path::PathBuf;

const DEFAULT_MC_VERSION: &str = "1.19";

fn default_target() -> Option<String> { Some(DEFAULT_MC_VERSION.to_string()) }
fn default_mc_dir() -> Option<PathBuf> { Some(get_mc_dir()) }

#[derive(Deserialize)]
pub struct ConfigFile {
    #[serde(default = "default_target")]
    pub target: Option<String>,
    #[serde(default = "default_mc_dir")]
    pub mc_dir: Option<PathBuf>,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self { 
            target: default_target(),
            mc_dir: default_mc_dir(),
        } 
    }
}

impl ConfigFile {
    /// Function that finds and tries to load config file
    /// ex location: "~/.config/mbrew/config.toml"
    ///
    ///  * If the config directory doesn't exist then it 
    /// loads `ConfigFile::default()`
    ///
    /// * If the "minebrew" config directory exists but the 
    /// "config.toml" file is not there then it loads `ConfigFile::default()`
    ///
    /// * If there was a different reason the config file couldn't 
    /// be read then it returns an error
    ///
    /// * If parsing the config file fails return an error
    ///
    /// * Checks if target field exists and if its valid, 
    /// if it isnt then return an error
    pub fn load() -> ConfigFile {
        let mut config_dir = dirs::config_dir()              
            .expect("Unable to locate config directory...");  // locate config dir
       
        config_dir.push("minebrew");                          // append "mbrew" to path so it

        if !config_dir.exists() {                             // if the config directory does not exist
            // std::fs::create_dir_all(&config_dir).ok();     // create it at default configuration directory
            return ConfigFile::default();                 // Just use default Options for now
        }

        // attempt to read config.toml to string handling 
        // specific errors by loading a default configuration

        let config_file: ConfigFile = match std::fs::read_to_string(config_dir.join("config.toml")) { 
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
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return ConfigFile::default(),
            Err(err) => {
                eprintln!("Error reading \"config.toml\": {err}");
                std::process::exit(1);
            }
        };

        // Takes &Option<String> and makes it an Option<&String> then we unwrap
        match valid_target_string(config_file.target.as_ref().unwrap()) {
            Ok(_) => config_file,
            Err(e) => { // invalid target string, exit program
                eprintln!("Error with field: {e}");
                std::process::exit(1);
            }, 
        } 
    }
}
