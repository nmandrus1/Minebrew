use reqwest::{Client, Response};
use std::fmt::Write;

const BASE_REQUEST: &str = "https://api.modrinth.com/v2/";

trait ToRequest {
    fn to_req(&self) -> String;
}

/// A Struct to represent the facets query parameter for the Modrinth search API
struct Facets {
    categories: Option<Vec<String>>,
    versions: Option<Vec<String>>,
    project_type: Option<String>,
    license: Option<String>,
}

impl Facets {
    /// Appends passed string to categories vec if it exists otherwise 
    /// creates a vector with the passed string as the first entry
    fn category(&mut self, category: String) {
        if let Some(c) = self.categories.as_mut() { c.push(category) }
        else { self.categories = Some(vec![category]) }
    }

    /// Appends passed string to versions vec if it exists otherwise 
    /// creates a vector with the passed string as the first entry
    fn version(&mut self, version: String) {
        if let Some(v) = self.versions.as_mut() { v.push(version) }
        else { self.versions = Some(vec![version]) }
    }
    
    /// Replaces self.project_type with passed string 
    fn project_type(&mut self, p_type: String) {
        self.project_type = Some(p_type);
    }

    /// Replaces self.license with passed string 
    fn license(&mut self, license: String) {
        self.license = Some(license);
    }

    fn to_query_param(&self) -> String {
        let mut ret = String::from("facets=[");

        // For every field in Self check if it exists and if it does append all 
        // the information into ret to complete the facets query parameter

        if let Some(cats) = self.categories.as_ref() {
            cats.iter()
                .for_each(|c| write!(&mut ret, "[\"categories:{}\"],", c).unwrap());
        }

        if let Some(vers) = self.versions.as_ref() {
            vers.iter()
                .for_each(|v| write!(&mut ret, "[versions:{}\"],", v).unwrap());
        }

        if let Some(p) = self.project_type.as_ref() {
            write!(&mut ret, "[\"project_type:{}\"],", p).unwrap()
        }

        if let Some(l) = self.license.as_ref() {
            write!(&mut ret, "[\"license:{}\"],", l).unwrap()
        }

        // if there's a lingering comma remove it
        if ret.ends_with(',') { ret.pop(); }

        ret.push(']');
        ret
    }
}

struct EmptyReq;
// https://api.modrinth.com/v2

#[derive(Default)]
struct Search {
    query: String,
    version: String,
} 

impl ToRequest for Search {
    fn to_req(&self) -> String {
        format!("{}/search?query={}&facets=[[\"versions:{}\"]]", 
            BASE_REQUEST, self.query, self.version)
    }
}

struct Project {}
struct Version {}

struct Modrinth<ReqType> {
    // Client that makes HTTP requests
    client: Client,

    // The generic that determines what methods are available to the caller
    req_type: ReqType,
}

impl Modrinth<EmptyReq> {
    pub fn new() -> Self {
        Self {
            client: Client::default(),
            req_type: EmptyReq,
        }
    }

    pub fn search(&self, search: Search) -> Modrinth<Search> {
        Modrinth { 
            client: self.client.clone(),
            req_type: search, 
        }
    }
}

impl Modrinth<Search> {
    pub fn query(&mut self, query: String) {
        self.req_type.query = query;
    }
}

/// For any type that implements the ToRequest trait (a trait that turns data into a valid API call)
// return the Future for processing elsewhere
impl<R: ToRequest> Modrinth<R> {
    pub async fn get(&mut self) -> impl futures::Future<Output=reqwest::Result<Response>> {
        self.client.get(self.req_type.to_req()).send()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let modrinth = Modrinth::new();
        modrinth.search(Search { query: "sodium".into(), version: "1.19".into() }).get();
        modrinth.search(Search { query: "fabric-api".into(), version: "1.19".into() }).get();
    }
}
