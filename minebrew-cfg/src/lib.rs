mod args;
mod config;

use config::{Config, load_config};
use args::Args;

pub fn load_args_and_config() -> (Args, Config) {
    let args = Args::parse_args(); // let clap do all the work of parsing
    
    // if this fn returns OK then we shouldnt have to return a Result,
    // if it Errors out then that means a config was found but can't 
    // be loaded and we should let the user know something is wrong
    match load_config() {
        Ok(cfg) => (args, cfg),
        Err(e)  => {
            eprintln!("Error loading config.toml: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {}
