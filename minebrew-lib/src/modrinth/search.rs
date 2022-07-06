use std::io::Write;

use super::shared::*;

use serde::Deserialize;

/// A struct used to build a query and then run a search 
/// for a query and then handle the result
pub struct Search <'a> {
    /// The string this will be the query
    pub queries: Vec<String>,

    /// limit the number of search results per query
    pub limit: u8,

    /// sorting method for search results
    pub index: &'a str,

    /// The version to filter minecraft searches by
    pub version: &'a str,
}

impl <'a> Search <'a> {
    /// Construct a search to make on the modrinth database
    pub fn new(queries: &[String], version: &'a str) -> Self {
        Self {
            queries: queries.to_vec(),
            limit: 5,
            index: "relevance",
            version, 
        }
    }

    /// Returns an iterator over the urls each of which is a search 
    /// on the modrinth database
    pub fn urls(&self) -> impl Iterator<Item=String> + '_ {
        self.queries.iter().map(|q| {
            format!(
                "https://api.modrinth.com/v2/search?query={}&limit={}&index={}&facets=[[\"versions:{}\"]]",
                q, self.limit, self.index, self.version)
        })
    }
}

/// Struct that represents the JOSN Response from a search 
#[derive(Deserialize)]
pub struct SearchResponse {
    pub hits: Vec<SearchResult>,

    #[serde(skip)]
    #[serde(rename = "offset")]
    _offset: u8,

    #[serde(skip)]
    #[serde(rename = "limit")]
    _limit: u8,

    #[serde(skip)]
    #[serde(rename = "total_hits")]
    _total_hits: u8,
}

impl SearchResponse {
    /// Function that takes a query and an lienence value and filters 
    /// out any search result that isnt withint a `lenience` levenshtein 
    /// distance
    pub fn filter(&mut self, query: &str, lenience: usize) {
        self.hits.retain(|res| {
            levenshtein(&query, &res.title) <= lenience 
            || levenshtein(&query, &res.slug) <= lenience 
        })
    }

    /// Function that narrows down the search results of a response 
    /// to a single one
    pub fn pick_result(mut self, query: &str) -> SearchResult {
        if self.hits.is_empty() {
            eprintln!("error: {} not found", query);
            std::process::exit(1);
        } else if self.hits.len() == 1 {
            self.hits.remove(0)
        } else {
            // Numbered list of results
            self.hits.iter().enumerate()
                .for_each(|(i, r)| println!("\t{}) {}", i+1, &r.title));

            print!("\nPick mod (default=1): ");
            std::io::stdout().flush().unwrap();

            // string to hold user input
            let mut input = String::with_capacity(2);

            let choice = loop { // loop until they pick a mod
                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim().len() == 0 { break 1; }
                match input.trim().parse::<usize>() {
                    Ok(num) => { // parsing was good but is it a valid option?
                        if num > self.hits.len() || num == 0 {
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

            self.hits.remove(choice-1)
        }
    }
}

fn levenshtein(s1: &str, s2:&str) -> usize {
    let (short, long) = if s1.len() > s2.len() {
        (s2, s1)
    } else {
        (s1, s2)
    };

    let n = long.len();

    let mut costs: Vec<usize> = (0..n+1).collect();
    for (i, c) in short.chars().enumerate() {
        costs[0] = i + 1;
        let mut corner = i;
        for (j, c2) in long.chars().enumerate() {
        let upper = costs[j + 1];
            if c == c2 {
                costs[j + 1] = corner;
            } else {
                costs[j + 1] = 1 + [costs[j], upper, corner].iter().min().unwrap();
            }
            corner = upper
        }
    }
    costs[n]
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
    pub slug: String,

    pub title: String,

    pub description: String,

    #[serde(skip)]
    #[serde(rename = "categories")]
    _categories: Vec<String>,

    pub client_side: Support,

    pub server_side: Support,

    pub project_type: ProjectType,

    #[serde(skip)]
    #[serde(rename = "downloads")]
    _downloads: usize,

    #[serde(skip)]
    #[serde(rename = "icon_url")]
    _icon_url: Option<String>,

    #[serde(skip)]
    #[serde(rename = "project_id")]
    _project_id: String,

    #[serde(skip)]
    #[serde(rename = "author")]
    _author: String,

    #[serde(skip)]
    #[serde(rename = "follows")]
    _follows: usize,

    pub versions: Vec<String>,

    #[serde(skip)]
    #[serde(rename = "date_created")]
    _date_created: String,

    #[serde(skip)]
    #[serde(rename = "date_modified")]
    _date_modified: String,

    #[serde(skip)]
    #[serde(rename = "latest_version")]
    _latest_version: Option<String>,

    #[serde(skip)]
    #[serde(rename = "license")]
    _license: String,

    #[serde(skip)]
    #[serde(rename = "gallery")]
    _gallery: Option<Vec<String>>,
}

// These are unit tests for this module
#[cfg(test)]
mod search_tests {}
