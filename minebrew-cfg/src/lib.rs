/// Configuration Goal: Load a default version of 

mod args;
mod config_file;

use config_file::ConfigFile;
pub use args::{ Options, Subcommands };

use anyhow::Result;

use std::path::PathBuf;

pub fn load_options() -> Options {
    let mut opts = Options::parse_args();
    let config_file = ConfigFile::load();

    match opts.command {
        Subcommands::Install(_) => {
            opts.target.get_or_insert(config_file.target.unwrap());
            opts.mc_dir.get_or_insert(config_file.mc_dir.unwrap());
        },
        _ => todo!()
    };
    opts
}

/// loads the Configuration Options
// Parses command line arguments and then attempts to read the 
// config file into a struct exiting the program if there is an 
// error reading the file


/// Function to determine whether a string that is supposed to 
/// represent a version number is valid
fn valid_target_string(s: &str) -> Result<()> {
    // check start, end, and whether it contains required info

    if !s.contains('.') 
        || !s.contains(|c: char| c.is_ascii_digit()) 
        || s.starts_with(|c: char| !c.is_ascii_digit()) 
        || s.ends_with(|c: char| !c.is_ascii_digit()) {

        anyhow::bail!("\"target\" isn't a valid version number")
    }

    // if c is a digit or ( c is '.' and the char before is not "." )
    if !s.chars().enumerate().all(|(i, c)| { 
        c.is_ascii_digit() || (c == '.' && (&s[i-1..i] != "."))}) { 

        anyhow::bail!("\"target\" contains invalid sequence of characters")
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
/// Unix: finds $HOME and then appends .minecraft
#[cfg(target_family = "windows")]
fn get_mc_dir() -> PathBuf {
    let mut home = match dirs::home_dir() {
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

/// Finds the ".minecraft" folder and exits if 
/// it could not find it.
///
/// Windows: finds %USERPROFILE% and then appends AppData before .minecraft
///
/// Unix: finds $HOME and then appends .minecraft

#[cfg(target_family = "unix")]
fn get_mc_dir() -> PathBuf {
    let mut home = match dirs::home_dir() {
        Some(p) => p,
        None => {
            eprintln!("Impossible to locate home directory...");
            std::process::exit(1);
        }
    };
    
    home.push(".minecraft");
    home
}

#[cfg(test)]
mod tests {
    use super::valid_target_string;

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
        let vb = "..";
        let vc = ".";
        let vd = "a.b.c";

        assert!(valid_target_string(v6).is_err());
        assert!(valid_target_string(v7).is_err());
        assert!(valid_target_string(v8).is_err());
        assert!(valid_target_string(v9).is_err());
        assert!(valid_target_string(va).is_err());
        assert!(valid_target_string(vb).is_err());
        assert!(valid_target_string(vc).is_err());
        assert!(valid_target_string(vd).is_err());
    }
}
