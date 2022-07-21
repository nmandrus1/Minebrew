use std::{io::Write, fs::OpenOptions};

use minebrew_lib::modrinth::{ Minebrew, Search, SearchResult };
use minebrew_cfg::{ Options, Subcommands };

#[tokio::main]
async fn main() {
    // -------- LOAD CONFIG -------- 
    let opts = Options::parse();

    match opts.command {
        Subcommands::Install(_) => install(opts),
        _ => todo!(),
    }.await
}

async fn install(mut opts: Options) {
    let mbrew = Minebrew::default();
    // unwraping is okay here because we should never not 
    // pass any other Subcommand variant other than Install
    let i_opts = opts.command.install_opts().unwrap();

    // Loop through every query made 
    // Turns quries into ModFile structs which have a download link
    let searches = Search::new(&i_opts.queries, &i_opts.target);

    println!("Searching modrinth for {} mods", &i_opts.target);

    // Make requests and serialize them
    let resps = mbrew.search(&searches).await;

    // filter out and choose search results
    let results: Vec<SearchResult> = resps.into_iter()
        .map(|mut sr| { 
            // filter out search results from each response
            sr.filter(2);
            sr.pick_result()
        }).collect();

    let files = mbrew.files_from_results(&results, &i_opts.target).await;

    // List all the mods ready to be downloaded and ask
    // the user to confirm
    let mut chars_left: usize = 0;
    println!("\nMods ({})", files.len());
    files.iter().for_each(|f| {
        match chars_left.checked_sub(&f.filename.len() + 2) {
            Some(left) => {
                print!("{}  ", f);
                chars_left = left;
            },
            None => {
                print!("\n\t{}  ", f);
                chars_left = 80 - &f.filename.len();
            },
        }
    });

    print!("\n\nBegin Installation? [y/n]");
    std::io::stdout().flush().unwrap(); // flush buffer to print everything

    // get user input
    let mut input = String::with_capacity(1);
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim().chars().nth(0) {
        Some('y') | Some('Y') | None => {},
        _ => std::process::exit(1),
    };

    // path to mods folder
    let mods_folder = i_opts.mc_dir.join("mods");

    println!("\nSearching for mods folder...");
    // if mods folder doesn't exist then make one
    if !mods_folder.exists() {
    println!("Not found, creating mods folder...");
        std::fs::create_dir_all(&mods_folder).unwrap();
    } else {
        println!("Mods folder found...")
    }

    // download all the files we've gathered
    mbrew.download_files(&files, &mods_folder).await;

    println!("\nSuccess!");
}
