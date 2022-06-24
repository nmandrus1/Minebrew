use minebrew_lib::modrinth::{Search, SearchResult};
use minebrew_cfg::Config;
use std::io::Write;

fn main() {
    // -------- LOAD CONFIG -------- 
    let config = Config::load();

    // set up query -- config.queries contains the users query
    // if the vector is empty then no query has been made 
    let queries = &config.queries; 
    let target  = &config.target;

    // make query
    let s = Search::new(&query, version);

    // check if it was successful
    println!("Searching database...");
    let res = s.search().unwrap();

    // filter out mods that don't fit the query
    let results: Vec<&SearchResult> = res
        .iter()
        .filter(|s| s.slug.contains(&query) || s.title.to_lowercase().contains(&query))
        .collect();

    // if there are no valid query results
    if results.is_empty() {
        eprintln!("target not found: \"{}\" with version: {}", usr_in, version);
        std::process::exit(1)
    } else if results.len() > 0 {
        // List the candidate mods and let the user choose one
        println!("{} mods found...\n", results.len());
        let mut writer = std::io::BufWriter::new(std::io::stdout()); // bufferring outp

        // iterate and write to stdout
        results.iter().enumerate().for_each(|(i, search_res)| {
            let title: &str = &search_res.title;
            writeln!(writer, "  {})  {title}", i+1).unwrap()
        });

        write!(writer, "\nChoose package (default=1): ").unwrap();
        writer.flush().unwrap();

        // grab user input
        let mut input = String::with_capacity(3);
        std::io::stdin().read_line(&mut input).unwrap();

        // trim the newline and parse
        let input = input.trim();
        match input.parse::<usize>() {
            Err(_) => { // basic error handlingquery
                eprintln!("\n\nExpected a positive number got: \"{}\"", input);
                std::process::exit(1);
            },
            Ok(num) => {
                if num > results.len() || num == 0 { // if out of range...
                    eprintln!("\n\nNot in range...");
                    std::process::exit(1);
                } else {
                    println!("\n\n {} = {}!!!", num, results[num-1].title);
                }
            }
        }
    }
}
