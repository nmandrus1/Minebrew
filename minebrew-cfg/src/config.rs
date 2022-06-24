use anyhow::Result;
use serde::Deserialize;
use etcetera::base_strategy::{choose_base_strategy, BaseStrategy};

#[derive(Deserialize)]
pub struct Config {
    pub target: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { target: String::from("1.18.2") } 
    }
}

/// Function that finds and tries to load config file
/// ex location: "~/.config/mbrew/config.toml"
pub fn load_config() -> Result<Config> {
    let strat = choose_base_strategy() 
        .expect("Unable to locate config directory..."); // locate config dir

    let mut config_dir = strat.config_dir();         // append "mbrew" to path so it
    config_dir.push("mbrew");                        // now holds the full path to the dir 


    if !config_dir.exists() {                      // if the config directory does not exist
        std::fs::create_dir_all(&config_dir).ok(); // create it at default configuration directory
    }

    // attempt to read config.toml to string handling 
    // specific errors by loading a default configuration
    match std::fs::read_to_string(config_dir.join("config.toml")) { 
        Ok(cfg) => {  
            // if the file was read successfully try to parse it
            Ok(toml::from_str(&cfg)
                .unwrap_or_else(|e| { // if can't be parsed print err msg and exit
                    eprintln!("Error in config file: {}", e);
                    std::process::exit(1);
                }))
        },
        // Cool syntax I didn't know about 
        // if the file isnt found then use default config otherwise return an error
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
        Err(err) =>  Err(anyhow::Error::from(err)),
    }
}
