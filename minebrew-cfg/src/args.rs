use clap::{Args, Parser, Subcommand};
use super::valid_target_string;

use std::path::PathBuf;

// TODO: Think of a way to have one big configuration struct so that we can 
// "merge" the command line arguments and the config file and just have one 
// location for configuration

// Custom parsing function for target string
fn parse_target(s: &str) -> Result<String, String> {
    match valid_target_string(s) {
        Ok(_) => Ok(s.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

/// Struct to hold the arguments passed through the command line
///
/// fields:
/// - command: Subcommands
/// - target: Option<String>
/// - mc_dir: Option<PathBuf>
#[derive(Parser)]
#[clap(name = "Minebrew")]
#[clap(author = "The Brogrammers")]
#[clap(version = "0.1")]
#[clap(about = "A fast and hassle-free mod package manager for minecraft")]
#[clap(long_about = None)]
pub struct Options {
    // The Subcommand enum which holds the struct 
    // with the arguments passed through
    #[clap(subcommand)]
    pub command: Subcommands,

    #[clap(short, long)]
    #[clap(help = "override the default Minecraft version")]
    #[clap(value_parser = parse_target)] 
    pub target: Option<String>,

    #[clap(short, long, help = "path to \".minecraft\"", value_parser)] 
    pub mc_dir: Option<PathBuf>,
}

impl Options {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

// Our subcommands
#[derive(Subcommand)]
pub enum Subcommands {
    #[clap(arg_required_else_help = true, about = "Subcommand used to install mods")]
    Install(InstallOpts),
    #[clap(arg_required_else_help = true, about = "Coming soon")]
    Search,
    #[clap(arg_required_else_help = true, about = "Coming soon")]
    Remove,
    #[clap(arg_required_else_help = true, about = "Coming soon")]
    Update,
    #[clap(arg_required_else_help = true, about = "Coming soon")]
    Config, 
}

impl Subcommands {
    /// "unwrap" the InstallOpts struct from enum
    pub fn install_opts(&mut self) -> Option<InstallOpts> {
        match self {
            Subcommands::Install(i) => Some(std::mem::take(i)),
            _ => None,
        }
    }
}

// The install struct, holds data and options passed 
// through the install subcommand
#[derive(Args, Default)]
pub struct InstallOpts {
    // Vector of strings representing the queries to make
    #[clap(required = true)]
    #[clap(help = "One or more mods to download, entries are separated by spaces")]
    #[clap(value_parser)]
    pub queries: Vec<String>,
}
