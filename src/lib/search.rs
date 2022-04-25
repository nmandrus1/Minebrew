use super::shared::*;

use serde::Deserialize;

/// A struct used to build a query and then run a search 
/// for a query and then handle the result
pub struct Search <'a> {
    limit: u8,
    query: &'a str,
    index: &'a str,
    version: &'a str,
}

impl <'a> Search <'a> {
    /// Create a new Search based on a query and Version number
    pub fn new(query: &'a str, version: &'a str) -> Self {
        Self {
            limit: 5,
            query,
            index: "relevance",
            version,
        }
    }

    /// Take a mutable reference to self and modify the version field and return Self for chaining
    pub fn version (mut self, version: &'a str) -> Self {
        self.version = version;
        self
    }

    /// Take the search struct and query the modrinth database
    pub fn search(&self) -> Result<SearchResponse, Box<dyn std::error::Error>> { 
        Ok(reqwest::blocking::get(self.to_url())?.json()?)
    }

    /// Turns the search struct into a url that will return the mods
    /// that fit the search
    fn to_url(&self) -> String {
        let url = format!(
            "https://api.modrinth.com/v2/search?query={}&limit={}&index={}&facets=[[\"versions:{}\"]]",
            self.query, self.limit, self.index, self.version);
        url
    }
}

// When making a GET request from a search url we get a json that has an array 
// of search results, as well as some other information about the response, that 
// is represented in this struct

/// Struct that represents the JOSN Response from a search 
#[derive(Deserialize)]
pub struct SearchResponse {
    pub hits: Vec<SearchResult>,
    offset: u8,
    limit: u8,
    total_hits: u8,
}

/// A Struct that represents the information given to the api caller contained 
/// in the response of a search more info here: https://docs.modrinth.com/docs/tutorials/api_search/
#[derive(Deserialize)]
pub struct SearchResult {
    slug: String,
    title: String,
    pub description: String,
    categories: Vec<String>,
    client_side: Support,
    server_side: Support,
    project_type: ProjectType,
    downloads: usize,
    icon_url: Option<String>,
    project_id: String,
    author: String,
    follows: usize,
    versions: Vec<String>,
    date_created: String,
    date_modified: String,
    latest_version: Option<String>,
    license: String,
    gallery: Option<Vec<String>>,
}

// These are unit tests for this module
#[cfg(test)]
mod search_tests {
    use super::*;

    /// function that reads a json into a string from a file at res/search.json
    fn search_json_string() -> String {
        let mut path = std::env::current_dir().unwrap();
        while path.file_stem().unwrap() != "Minebrew" && path.pop() {}

        if path.file_name().is_none() { panic!("Couldn't locate Minebrew directory") }

        path.push("res/search.json");

        std::fs::read_to_string(path).unwrap()
    }

    // Test to make sure construction works as expected
    #[test]
    fn search_new_test() {
        let s = Search::new("sodium", "1.18.2");
        assert_eq!(s.limit, 5);
        assert_eq!(s.query, "sodium");
        assert_eq!(s.index, "relevance");
        assert_eq!(s.version, "1.18.2");
    }

    // Test to make sure url generation from Search struct works as expected
    #[test]
    fn search_url_test() {
        let s = Search::new("sodium", "1.18.2");
        assert_eq!(
            s.to_url(),
            "https://api.modrinth.com/v2/search?query=sodium&limit=5&index=relevance&facets=[[\"versions:1.18.2\"]]"
            )
    }

    // Test to make sure that Search can be deserialized properly
    #[test]
    fn deserialize_test_search_result() {
        let s = search_json_string();
        // the "_" tells rust that we are not planning on using this variable, this is 
        // just to get no warnings
        let _: SearchResult = serde_json::from_str(&s).unwrap();
    }
}
