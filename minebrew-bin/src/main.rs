use minebrew_lib::modrinth::{ Search, SearchResult, Version, ModFile };
use minebrew_cfg::{ load_options, Options, Subcommands };

fn main() {
    // -------- LOAD CONFIG -------- 
    let opts = load_options();
    match opts.command {
        Subcommands::Install(_) => install(opts),
        _ => todo!(),
    }
}

fn install(mut opts: Options) {
    // unwraping is okay here because we should never not 
    // pass any other Subcommand variant other than Install
    let install_opts = opts.command.install_opts().unwrap();

    let queries = install_opts.queries;
    let target = opts.target.unwrap();
    let mc_dir = opts.mc_dir.unwrap();

    // Loop through every query made 
    // Turns quries into ModFile structs which have a download link
    let downloads: Vec<ModFile> = queries.into_iter().map(|query| {
        println!("Searching modrinth for {} for Minecraft {}", query, &target);
        let s = Search::new(&query, &target);
        let mut results = s.search().unwrap();

        println!("Results found...");
        // filter out results that dont match the query
        results.hits = results.hits.into_iter().filter(|s_res| { 
            let title_lower = s_res.title.to_lowercase();
            let slug_lower = s_res.slug.to_lowercase();
            let query_lower = query.to_lowercase();

            // very basic string comparisons
            // TODO: Replace with homemade method that wouldn't allocate 
            //      - Good first problem for Johnny/Sam

            slug_lower.contains(&query_lower) || title_lower.contains(&query_lower)
        }).collect::<Vec<SearchResult>>();

        // If there are more than 1 results then ask the user for the mod they want
        let res = if results.hits.is_empty() {
            eprintln!("error: {} not found", query);
            std::process::exit(1);
        } else if results.hits.len() == 1 {
            &results.hits[0]
        } else {
            // Numbered list of results
            results.hits.iter().enumerate()
                .for_each(|(i, r)| println!("\t{}) {}", i+1, &r.title));

            print!("\nPick mod (default=1): ");

            // string to hold user input
            let mut input = String::with_capacity(2);

            let choice = loop { // loop until they pick a mod
                std::io::stdin().read_line(&mut input).unwrap(); 
                match input.parse::<usize>() {
                    Ok(num) => { // parsing was good but is it a valid option?
                        if num > results.hits.len() || num == 0 {
                            eprint!("Invalid input, try again: "); 
                            continue;
                        } else { break num }
                    },
                    Err(_)  => { // Parsing didn't work so restart loop
                        eprint!("Invalid input, try again: "); 
                        continue;
                    }
                };
            };

            &results.hits[choice+1]
        };

        // here we could do more complicated processing like beta versions/featured versions
        // .remove() moves a value out of the vector, needed here because if we returned a 
        // reference to data created in this closure then it wouldnt exist anymore once 
        // the closure is done, so we move the data into the "downloads" vector at the top
        // so the data we need is still alive 
        let mut version = Version::search(&res.slug, &target).unwrap().remove(0);
        version.files.remove(0)
    }).collect::<Vec<ModFile>>();

    // path to mods folder
    let mods_folder = mc_dir.join("mods");

    println!("\nSearching for mods folder...");
    // if mods folder doesn't exist then make one 
    if !mods_folder.exists() {
    println!("Not found, creating mods folder...");
        std::fs::create_dir_all(&mc_dir).unwrap();
    } else { 
        println!("Mods folder found...") 
    }

    // download all the files we've gathered
    downloads.iter().for_each(|file| {
        println!("Downloading {}", &file.filename);
        let bytes = Version::download_file(&file.url);
        std::fs::write(mods_folder.join(&file.filename), &bytes).unwrap()
    });
    
    println!("Success!");
}
