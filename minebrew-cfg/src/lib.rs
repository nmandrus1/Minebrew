/// Configuration Goal: Load a default version of 

mod args;
mod config_file;

use config_file::ConfigFile;
pub use args::{ Options, Subcommands };

use std::path::PathBuf;

#[derive(Debug)]
pub enum ConfigError {
    TargetInvalidNum,
    TargetInvalidChars,
}

impl std::error::Error for ConfigError {} 

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::TargetInvalidNum => 
                write!(f, "isn't a valid version number"),

            ConfigError::TargetInvalidChars => 
                write!(f, "contains invalid sequence of characters"),
        }
    }
}

/// Convience functino for ending the program with an error message
pub fn exit_with_msg(msg: &str) -> ! {
    eprintln!("{msg}");
    std::process::exit(1);
}

/// loads the Configuration Options
// Parses command line arguments and then attempts to read the 
// config file into a struct exiting the program if there is an 
// error reading the file


/// Function to determine whether a string that is supposed to 
/// represent a version number is valid
fn valid_target_string(s: &str) -> Result<(), ConfigError> { 
    // check start, end, and whether it contains required info

    if !s.contains('.') 
        || !s.contains(|c: char| c.is_ascii_digit()) 
        || s.starts_with(|c: char| !c.is_ascii_digit()) 
        || s.ends_with(|c: char| !c.is_ascii_digit()) 
        || s.chars().filter(|c| c == &'.').count() >= 3 {

        // anyhow::bail!("\"target\" isn't a valid version number")
        return Err(ConfigError::TargetInvalidNum);
    }

    // if c is a digit or ( c is '.' and the char before is not "." )
    if !s.chars().enumerate().all(|(i, c)| { 
        c.is_ascii_digit() || (c == '.' && (&s[i-1..i] != "."))}) { 

        // anyhow::bail!("\"target\" contains invalid sequence of characters")
        return Err(ConfigError::TargetInvalidChars);
    }

    Ok(())
}

// Conditional Compiliation bc .minecraft is in 
// different places depending on target_family
/// Finds the ".minecraft" folder and exits if 
/// it could not find it.
///
/// Windows: finds %USERPROFILE% and then appends AppData before .minecraft
///
/// macOS: finds $HOME directory and from there minecraft folder is at 
///  ~/Library/Application Support/minecraft
///
/// Unix: finds $HOME and then appends .minecraft
/// Finds the ".minecraft" folder and exits if 
/// it could not find it.
#[cfg(target_os = "windows")]
fn get_mc_dir() -> PathBuf {
    // Should put us in C:\Users\USERNAME\AppData\Roaming
    match dirs::config_dir() {
        Some(config_dir) => config_dir.join(".minecraft"),
        None =>  exit_with_msg("Impossible to locate home directory..."),
    }
}

#[cfg(target_os = "macos")]
fn get_mc_dir() -> PathBuf {
    // should be $HOME/Library/Application Support/
    match dirs::config_dir() {
        Some(config_dir) => config_dir.join("minecraft"),
        None =>  exit_with_msg("Impossible to locate home directory..."),
    }
}

#[cfg(target_os = "linux")]
fn get_mc_dir() -> PathBuf {
    // should be in /home/USER/
    match dirs::home_dir() {
        Some(home_dir) => home_dir.join(".minecraft"),
        None =>  exit_with_msg("Impossible to locate home directory..."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parser() {
        // should work
        let v1 = "1.1";
        let v2 = "1.18";
        let v3 = "1.18.2";
        let v4 = "1.18.20";
        let v5 = "12.18.20";

        assert!(valid_target_string(v1).is_ok());
        assert!(valid_target_string(v2).is_ok());
        assert!(valid_target_string(v3).is_ok());
        assert!(valid_target_string(v4).is_ok());
        assert!(valid_target_string(v5).is_ok());


        // should not work
        let v6 = "";
        let v7 = "1";
        let v8 = "1.";
        let v9 = ".1";
        let va = ".1.";
        let vb = "1.2.3.4";
        let vc = "..";
        let vd = ".";
        let ve = "a.b.c";

        assert!(valid_target_string(v6).is_err());
        assert!(valid_target_string(v7).is_err());
        assert!(valid_target_string(v8).is_err());
        assert!(valid_target_string(v9).is_err());
        assert!(valid_target_string(va).is_err());
        assert!(valid_target_string(vb).is_err());
        assert!(valid_target_string(vc).is_err());
        assert!(valid_target_string(vd).is_err());
        assert!(valid_target_string(ve).is_err());
    }
}
