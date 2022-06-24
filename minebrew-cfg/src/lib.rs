mod args;
mod config_file;

use config_file::load_config_file;
use args::Args;

/// The final Config Strcut that merges the config file and 
/// command line arguments passed through 
pub struct Config {
    // the target version of Minecraft
    pub target: String,

    // the queries made to the mod hosting service
    pub queries: Vec<String>,
}

impl Config {
    /// loads the Configuration Options
    // Parses command line arguments and then attempts to read the 
    // config file into a struct exiting the program if there is an 
    // error reading the file
    pub fn load() -> Self {
        let args = Args::parse_args(); // let clap do all the work of parsing
    
        // if this fn returns OK then we shouldnt have to return a Result,
        // if it Errors out then that means a config was found but can't 
        // be loaded and we should let the user know something is wrong
        let config_file = match load_config_file() {
            Ok(cfg) =>  cfg,
            Err(e)  => {
                eprintln!("Error loading config.toml: {}", e);
                std::process::exit(1);
            }
        };

        // go over all the fields and merge config options 
        // with a preference for the args passed 

        let target = match args.target {
            Some(t) => t,
            None  => config_file.target
        };

        // dont match for queries bc it should always be passed (for now)
        Self { target, queries: args.queries }
    }
}

#[cfg(test)]
mod tests {}
