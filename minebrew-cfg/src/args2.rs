use clap::{Arg, Command, ArgMatches};
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

fn arg_target<'a>() -> Arg<'a> {
    Arg::new("target")
        .short('t')
        .long("target")
        .help("Minecraft version the mod(s) should be compatible with")
        .takes_value(true)
        .value_parser(parse_target)
}

fn arg_mc_dir<'a>() -> Arg<'a> {
    Arg::new("directory")
        .short('d')
        .long("directory")
        .help("Path to .minecraft (minecraft for MacOS) folder")
        .takes_value(true)
        .value_parser(clap::value_parser!(PathBuf))
}

/// Struct to hold the arguments passed through the command line
///
/// fields:
/// - command: Subcommands
pub struct Options {
    // The Subcommand enum which holds the struct 
    // with the arguments passed through
    pub command: Subcommands,
}

impl Options {
    pub fn parse() -> Self {
        use super::ConfigFile;

        // load config file for Options Defaults
        let cfg_file = ConfigFile::load();
        // unwrap them here so we dont have to later
        let target = cfg_file.target.unwrap();
        let mc_dir = cfg_file.mc_dir.unwrap();

        // App Structure
        let app = Command::new("mbrew")
        .name("Minebrew")
        .author("The Brogrammers")
        .version("0.0.2")
        .about("A fast and hassle-free mod package manager for minecraft")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // install subcommand
        .subcommand(
            Command::new("install")
                .about("Searches for and installs a mod")
                .arg(
                    // install specific argument defined here
                    Arg::new("queries")
                        .help("the mod(s) to installs separated by spaces")
                        .takes_value(true)
                        .multiple_values(true)
                        .required(true)
                ) // Set default values for target and mc_dir 
                  // arg_target and arg_mc_dir return args bc
                  // they will be used outside of install
                  // we add default values here for lifetime reasons
                .arg( // --target option
                        arg_target()
                            .default_value(&target)
                            .hide_default_value(true)
                )
                .arg( // --mc_dir options
                        arg_mc_dir()
                            .default_value_os(mc_dir.as_os_str())
                            .hide_default_value(true)
                )
        );

        let mut matches = app.get_matches();

        let (cmd, sub_matches) = matches.remove_subcommand().unwrap();

        match (cmd.as_str(), sub_matches) {
            ("install", install_matches) => Self {
                command: Subcommands::Install(InstallOpts::from(install_matches))
            },

            _ => unreachable!()
        }
    }
}

// Our subcommands
pub enum Subcommands {
    Install(InstallOpts),
    Search,
    Remove,
    Update,
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
#[derive(Default)]
pub struct InstallOpts {
    // Vector of strings representing the queries to make
    pub queries: Vec<String>,
    pub target: String,
    pub mc_dir: PathBuf,
}

impl From<ArgMatches> for InstallOpts {
    fn from(mut matches: ArgMatches) -> Self {
        Self {
            queries: matches.remove_many("queries").unwrap().collect(),
            target: matches.remove_one::<String>("target").unwrap(),
            mc_dir: matches.remove_one::<PathBuf>("directory").unwrap()
        }
    }
}
