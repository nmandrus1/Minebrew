use mbrew_lib::Search;

use reqwest::blocking::get;

fn main() {
    let s = Search::new("sodium", "1.18.2");
    s.search().unwrap().iter().for_each(|s| println!("{}", s.slug))
}
