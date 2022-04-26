use mbrew_lib::Search;

fn main() {
    let query = std::env::args().nth(1).unwrap();
    let s = Search::new(&query, "1.18.2");

    let res = s.search().unwrap(); 
    res.iter().for_each(|s| println!("{} - {}", s.slug(), s.description()));
}
