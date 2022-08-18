use std::path::PathBuf;

use argh::FromArgs;

use super::valid_target_string;

#[derive(FromArgs)]
/// A fast, hassle-free package manager for minecraft
pub struct Args {
    #[argh(subcommand)]
    pub command: Commands,
}

impl Args {
    pub fn parse() -> Self {
        argh::from_env::<Self>()
    }
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Commands {
    Install(InstallArgs)
}

#[derive(FromArgs)]
#[argh(subcommand, name = "install")]
/// Install one or more mods
pub struct InstallArgs {
    #[argh(option, short = 't', from_str_fn(parse_target))]
    /// the target version of Minecraft
    pub target: Option<String>, 
    
    #[argh(option, short = 'd')]
    /// path to your .minecraft directory
    pub directory: Option<PathBuf>, 
    
    #[argh(positional)]
    /// one or more mods to install
    pub queries: Vec<String>, 
}

fn parse_target(value: &str) -> Result<String, String> {
    valid_target_string(value).map(|_| value.to_string()).map_err(|e| e.to_string())
}
