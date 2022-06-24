use clap::Parser;

// TODO: Think of a way to have one big configuration struct so that we can 
// "merge" the command line arguments and the config file and just have one 
// location for configuration

/// Struct to hold the arguments passed through the command line
#[derive(Parser)]
#[clap(name = "Minebrew")]
#[clap(author = "Niels Mandrus, Sam King, and Johnny Wilkes")]
#[clap(version = "0.1")]
#[clap(about = "A fast and hassle-free mod package manager for minecraft")]
pub struct Args {
    // "free" arguments, those that are not options
    // without this any "free" args will cause an error
    #[clap(required = true, value_parser)]
    pub queries: Vec<String>,

    #[clap(short, long, value_parser = valid_target_string)]
    pub target: Option<String>,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
fn valid_target_string(s: &str) -> Result<String, String> {
    // check start, end, and whether it contains required info
    if !s.contains('.') 
        || !s.contains(|c: char| c.is_ascii_digit()) 
        || s.starts_with('.') 
        || s.ends_with('.') {

            return Err(format!("Invalid target version passed: {}", s))
    }

    if !s.chars().enumerate().all(|(i, c)| {
        // if c is a digit or ( c is '.' and the char before is not "." )
        c.is_ascii_digit() || (c == '.' && (&s[i-1..i] != "."))
    }) { return  Err(format!("Invalid target version passed: {}", s)) }

    Ok(s.to_owned())
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
