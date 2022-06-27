/// The Goal of this module is to provide the binary with a concrete set of data
/// that leaves no ambiguity or questions as to whether there are other options 
/// to consider. In other words, the arguments and config file should be merged
/// and have no Options or Results in the struct. The binary should do no 
/// validation checking because if it made it to the config struct it IS valid.
/// If a field is unable to be produced then the program should not run, it 
/// should exit cleanly and with a descriptive  error message that is clear 
/// and helpful to the user

mod args;
mod config_file;

use config_file::{load_config_file, ConfigFile};
use args::{ Options, Subcommands };

use anyhow::Result;

/// loads the Configuration Options
// Parses command line arguments and then attempts to read the 
// config file into a struct exiting the program if there is an 
// error reading the file
pub fn load() -> Subcommands {
    let args = Options::parse_args(); // let clap do all the work of parsing

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

    // Combine config file options with arguments 
    // passed through with a preference for the values passed
    let command = match args.command {
        Subcommands::Install(i) => ,
        _ => todo!()
    };

    let target = match args.target {
        Some(t) => t,
        None  => match valid_target_string(&config_file.target) {
            // Now we attempt to use the config file's target 
            // but we have to check if the target string is valid
            Ok(_)  => config_file.target,
            Err(e) => { eprintln!("{e}"); std::process::exit(1) }
        },
    };
    
    // dont match for queries bc it should always be passed (for now)
}



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
