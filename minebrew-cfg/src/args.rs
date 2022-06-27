use clap::{Args, Parser, Subcommand};
use super::{valid_target_string, ConfigFile};

// TODO: Think of a way to have one big configuration struct so that we can 
// "merge" the command line arguments and the config file and just have one 
// location for configuration

/// Struct to hold the arguments passed through the command line
#[derive(Parser)]
#[clap(name = "Minebrew")]
#[clap(author = "Niels Mandrus, Sam King, and Johnny Wilkes")]
#[clap(version = "0.1")]
#[clap(about = "A fast and hassle-free mod package manager for minecraft")]
pub struct Options {
    // The Subcommand enum which holds the struct 
    // with the arguments passed through
    #[clap(subcommand)]
    pub command: Subcommands,
}

// Our subcommands
#[derive(Subcommand)]
pub enum Subcommands {
    Install(Install),
    Search,
    Remove,
    Config,
}

impl Options {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

// Custom parsing function for target string
fn parse_target(s: &str) -> Result<String, String> {
    match valid_target_string(s) {
        Ok(_) => Ok(s.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

/// The install struct, holds data and options passed 
/// through the install subcommand
#[derive(Args)]
pub struct Install {
    // Vector of strings representing the queries to make
    #[clap(required = true, value_parser)]
    pub queries: Vec<String>,

    // Optional string to specify target version of Minecraft
    #[clap(short, long, value_parser = parse_target)]
    pub target: Option<String>,
}

impl Install {
    // Takes ownership of ConfigFile as we are 
    // merging the two and using the Install struct
    pub fn merge_configs(&mut self, cfg_file: ConfigFile) {
        self.target = match self.target.or(cfg_file.target) {
            Some(t)  => t,
            Err(e) => { eprintln!("{e}"); std::process::exit(1) }
        },
    }
}
