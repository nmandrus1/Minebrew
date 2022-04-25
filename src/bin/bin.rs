use mbrew_lib::Search;

use reqwest::blocking::get;

fn main() {
    let s = Search::new("sodium", "1.18.2");
    println!("{}", s.search().unwrap().hits[0].description);
}
