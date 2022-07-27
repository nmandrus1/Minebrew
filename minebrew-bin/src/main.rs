use futures::stream::{self, StreamExt};

use minebrew_lib::{ Minebrew, SearchResponse };
use minebrew_cfg::{ Options, Subcommands };

type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> { 
   // -------- LOAD CONFIG -------- 
    let opts = Options::parse();

    match opts.command {
        Subcommands::Install(_) => install(opts),
        _ => todo!(),
    }.await?;

    Ok(())
}

// search for mods 
//   - Make API calls
//
// filter results
//   - Filter out results that have
//     too high a levenshtein dist.
//   - If more than 1 option remains
//     get user input
//
// find the correct verion to install
//   - Get a list of all the 
//   versions of the mod
//   - Pick the most recent version that 
//     is acceptable to user
//
// download file
async fn install(mut opts: Options) -> Result<(), Error> {
    // unwraping is okay here because we should never not 
    // pass any other Subcommand variant other than Install
    let i_opts = opts.command.install_opts().unwrap();
    let num_queries = i_opts.queries.len();

    let target = i_opts.target;

    // Loop through every query made 
    // Turns quries into ModFile structs which have a download link
    // let searches = Search::new(&i_opts.queries, &i_opts.target);

    let mut mbrew = Minebrew::default();

    println!("Searching modrinth for {} mods", &target);

    // Make API calls
    let mut resps: Vec<SearchResponse> = stream::iter(i_opts.queries)
        .map(|q| mbrew.search(q, &target))
        .buffer_unordered(num_queries)
        .collect().await;

    // filter results
    let results = resps.iter_mut()
        .map(|sr| { 
            // filter out search results from each response
            sr.filter(2);
            sr.pick_result()
        });

    // queue versions to download
    // TEMPORARY until we can figure out how to do this better
    mbrew.add_to_queue(results, &target).await?;

    // path to mods folder
    let mods_folder = i_opts.mc_dir.join("mods");

    println!("\nSearching for mods folder...");
    // if mods folder doesn't exist then make one
    if !mods_folder.exists() {
    println!("Not found, creating mods folder...");
        std::fs::create_dir_all(&mods_folder)?;
    } else {
        println!("Mods folder found...")
    }

    // download all the files we've gathered
    mbrew.download_files(&mods_folder).await?;

    println!("\nSuccess!");
    Ok(())
}
