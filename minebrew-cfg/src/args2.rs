use clap::{Command, Arg, ArgM};
use super::valid_target_string;

use std::path::PathBuf;

// TODO: Think of a way to have one big configuration struct so that we can 
// "merge" the command line arguments and the config file and just have one 
// location for configuration

// This way if we have to update default target version its in one place
const DEFAULT_MC_VERSION: &'static str = "1.19";

// Conditional Compiliation bc .minecraft is in 
// different places depending on target_family
#[cfg(target_family = "windows")]
fn get_mc_dir() -> PathBuf {
    let mut home = match home::home_dir() {
        Some(p) => p,
        None => {
            eprintln!("Impossible to locate home directory...");
            std::process::exit(1);
        }
    };

    home.push("AppData");
    home.push(".minecraft");
    home
}

#[cfg(target_family = "unix")]
fn get_mc_dir() -> PathBuf {
    let mut home = match home::home_dir() {
        Some(p) => p,
        None => {
            eprintln!("Impossible to locate home directory...");
            std::process::exit(1);
        }
    };
    
    home.push(".minecraft");
    home
}

// Custom parsing function for target string
fn parse_target(s: &str) -> Result<String, String> {
    match valid_target_string(s) {
        Ok(_) => Ok(s.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

// Args that will be reused should have a functions for easy maintenance 

fn target_arg() -> Arg<'static> {
    Arg::new("target")
        .short('t')
        .long("target")
        .help("The target Minecraft version")
        .takes_value(true)
        .value_parser(valid_target_string)
        .default_value(DEFAULT_MC_VERSION)
}

fn queries_arg() -> Arg<'static> {
    Arg::new("mod")
        .help("mod(s) to install")
        .takes_value(true)
        .multiple_values(true)
}

fn options() -> Command<'static> {
    let cmd = Command::new("mbrew")
        .name("Minebrew")
        .about("A fast and hassle-free mod package manager for minecraft") 
        .author("Brogrammers")
        .version("0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("install")
                .about("Install specified mod(s)")
                .arg(target_arg())
                .arg(queries_arg())
                .arg_required_else_help(true)
        )
        .arg(
            Arg::new("mc_dir")
                .long("mc_dir")
                .help("location of \".minecraft\" folder")
                .takes_value(true)
                .value_parser(clap::value_parser!(PathBuf))
        ).get_ma
}
