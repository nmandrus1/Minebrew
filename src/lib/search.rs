use super::shared::*;

use serde::Deserialize;

/// A struct used to build a query and then run a search 
/// for a query and then handle the result
pub struct Search <'a> {
    limit: u8,
    /// The string this will be the query
    query: &'a str,
    index: &'a str,
    /// The version to filter minecraft searches by
    version: &'a str,
}

impl <'a> Search <'a> {
    /// Create a new Search based on a query and version number
    ///
    /// # Arguments
    ///
    /// * `query` - a string slice that holds the query
    /// * `version` - a string slice that holds the version number
    ///
    /// # Example
    ///
    /// ```
    ///  use mbrew_lib::Search;
    ///
    ///  fn main() {
    ///     let s = Search::new("sodium", "1.18.12");
    ///  }
    /// ```
    pub fn new(query: &'a str, version: &'a str) -> Self {
        Self {
            limit: 3,
            query,
            index: "relevance",
            version,
        }
    }

    pub fn version (mut self, version: &'a str) -> Self {
        self.version = version;
        self
    }

    /// Take the search struct and query the modrinth database
    ///
    /// # Arguments
    ///
    /// * `&self` - an immutable reference to self
    ///
    /// # Example
    ///
    /// ```
    /// use mbrew_lib::{Search, SearchResponse};
    ///
    /// fn main() {
    ///     let s = Search::new("sodium", "1.18.2");
    ///     
    ///     // Sends a HTTP Request to API and expects a JSON with 
    ///     // a list of the "hits" for the query
    ///     let res: SearchResponse = s.search().unwrap();
    ///     
    ///     // do stuff with response...
    /// }
    /// ```
    pub fn search(&self) -> Result<SearchResponse, Box<dyn std::error::Error>> { 
        Ok(reqwest::blocking::get(self.to_url())?.json()?)
    }

    

    /// Turns the search struct into a url that will return the mods
    /// that fit the search
    fn to_url(&self) -> String {
        format!(
            "https://api.modrinth.com/v2/search?query={}&limit={}&index={}&facets=[[\"versions:{}\"]]",
            self.query, self.limit, self.index, self.version)
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

// The implementation of SearchResponse
impl SearchResponse {
    /// Create an iterator over the responses
    ///
    /// # Arguments 
    ///
    /// * `&self` - immutable reference to self
    ///
    /// # Example
    ///
    /// ```
    /// use mbrew_lib::{Search, SearchResponse};
    ///
    /// fn main() {
    ///     let s = Search::new("sodium", "1.18.2").search().unwrap();
    ///
    ///     // iterator over SearchResults
    ///     let mut iter = s.iter();
    ///
    ///     // 
    ///     assert_eq!("sodium", iter.next().unwrap().slug());
    /// }
    /// ```
    pub fn iter(&self) -> SearchResIter {
        SearchResIter { res: self, offset: 0 }
    }

    /// A simple function to test if there were no hits on the search
    ///
    /// # Arguments
    ///
    /// * `&self` - immutable reference to self 
    ///
    /// # Example
    ///
    /// ```
    /// use mbrew_lib::Search;
    ///
    /// fn main() {
    ///
    ///     let s = Search::new("No Mod has This Name", "1.30.17");
    ///
    ///     assert_eq!(s.search().unwrap().is_empty(), true);
    /// }
    /// ```
    pub fn is_empty(&self) -> bool {
        self.hits.is_empty()
    }
}

/// Iterator over the hits of a SearchResponse
pub struct SearchResIter<'a> {
    res: &'a SearchResponse,
    offset: usize,
}

// implementation of the iterator, because its just a wrapper over a vector
// we just keep track of what index we are at and increment it each time
// we call next
impl<'a> Iterator for SearchResIter<'a> {
    type Item = &'a SearchResult;

    /// Returns an Option<&SearchResult> that would be the next element
    fn next(&mut self) -> Option<Self::Item> {
        self.offset += 1;
        self.res.hits.get(self.offset - 1)
    }
}

//#[serde(skip)] tells the serde library to not 
// bother to deserialize or serialize that
// field into a JSON, this will save some 
// time because most of the info we're not using
//
// The "_" prefixing the name of the fields has a similar
// effect, that tells the rust compiler that we aren't 
// planning to use that variable at the moment but we may
// in the future, it mostly just stops all the warnings

/// A Struct that represents the information given to the api caller contained 
/// in the response of a search more info here: https://docs.modrinth.com/docs/tutorials/api_search/
#[derive(Deserialize)]
pub struct SearchResult {
    slug: String,
    title: String,
    description: String,
    #[serde(skip)]
    categories: Vec<String>,
    client_side: Support,
    server_side: Support,
    project_type: ProjectType,
    #[serde(skip)]
    downloads: usize,
    #[serde(skip)]
    icon_url: Option<String>,
    project_id: String,
    #[serde(skip)]
    author: String,
    #[serde(skip)]
    follows: usize,
    #[serde(skip)]
    versions: Vec<String>,
    #[serde(skip)]
    date_created: String,
    #[serde(skip)]
    date_modified: String,
    #[serde(skip)]
    latest_version: Option<String>,
    #[serde(skip)]
    license: String,
    #[serde(skip)]
    gallery: Option<Vec<String>>,
}

impl SearchResult {
    /// Get the slug of the project
    pub fn slug(&self) -> &str {
        &self.slug
    }
    
    pub fn description(&self) -> &str {
        &self.description
    }
    
    pub fn title(&self) -> &str {
        &self.title
    }
    
    pub fn id(&self) -> &str {
        &self.project_id
    }
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

        path.push("res/tests/search.json");

        std::fs::read_to_string(path).unwrap()
    }

    // Test to make sure construction works as expected
    #[test]
    fn search_new_test() {
        let s = Search::new("sodium", "1.18.2");
        assert_eq!(s.limit, 3);
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
            "https://api.modrinth.com/v2/search?query=sodium&limit=3&index=relevance&facets=[[\"versions:1.18.2\"]]"
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
