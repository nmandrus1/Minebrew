use mbrew_lib::{
    modrinth::{Search, SearchResult},
    traits::*,
};

fn main() {
    let query = std::env::args().nth(1).unwrap().to_lowercase();
    let version = "1.18.2";
    let s = Search::new(&query, version);

    if let Some(s) = res
        .iter()
        .find(|s| s.slug() == query || s.title().to_lowercase() == query)
    {
        println!("{} id: {}", s.slug(), s.id())
    } else {

    }

    let results: Vec<&SearchResult> = res
        .iter()
        .filter(|s| s.slug().contains(&query) || s.title().to_lowercase().contains(&query))
        .collect();

    if results.is_empty() {
        eprintln!("target not found: \"{}\" with version: {}", query, version);
    }
}
