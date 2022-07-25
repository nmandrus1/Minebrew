use std::fmt::Write;
use std::convert::From;

use reqwest::{Client, Response};

use super::shared::{ProjectType, Category};

// have to import this here bc stupid publicity rules abt traits
use super::traits::ToRequest;

const BASE_REQUEST: &str = "https://api.modrinth.com/v2/";



/// A Struct to represent the facets query parameter for the Modrinth search API
#[derive(Default)]
struct Facets<'a> {
    categories: Vec<Category>,
    versions: Vec<&'a str>,
    project_type: ProjectType,
    license: String,
}

impl<'a> Facets<'a> {
    /// Turns the Facet Struct into a query parameter for an API call
    fn to_query_param(&self) -> String {
        let mut ret = String::from("&facets=[");

        // For every field in Self check if it exists and if it does append all 
        // the information into ret to complete the facets query parameter

        self.categories.iter()
            .for_each(|c| match c {
                Category::None => {},
                c => write!(&mut ret, "[\"categories:{}\"],", c.to_str()).unwrap()
        });

        self.versions.iter()
            .for_each(|v| write!(&mut ret, "[versions:{}\"],", v).unwrap());

        match &self.project_type {
            ProjectType::None => {},
            p => write!(&mut ret, "[\"project_type:{}\"],", p.to_str()).unwrap()
        }

        if !self.license.is_empty() {
            write!(&mut ret, "[\"license:{}\"],", &self.license).unwrap()
        }

        // if there's a lingering comma remove it
        if ret.ends_with(',') { ret.pop(); }

        ret.push(']');
        ret
    }
}

struct EmptyReq;

impl ToRequest for EmptyReq {
    fn to_req(&self) -> String {
        BASE_REQUEST.to_string()
    }
}

/// SearchReq struct that implements ToRequest
struct SearchReq<'a> {
    // each of these fields represents a query 
    // parameter for the modrinth search API
    query: &'a str,
    limit: usize,
    index: &'a str,
    facets: Facets<'a>,
} 

impl<'a> SearchReq<'a> {
    /// Takes a query and contructs a default search Struct
    fn new(query: &'a str) -> Self {
        Self {
            query,
            limit: 5,
            index: "relevance",
            facets: Facets::default(),
        }
    }
}

impl<'a> ToRequest for SearchReq<'a> {
    fn to_req(&self) -> String {
        // the "&" before facets is taken care of by to_query_param()
        format!("{}/search?query={}{}", 
            BASE_REQUEST, self.query, self.facets.to_query_param())
    }
}

/// ProjectReq struct that implements ToRequest
struct ProjectReq<'a> {
    // slug: &'a str
    id: &'a str,
    // list versions
    version: bool,
    // revisit to make ID a strong type
}

impl<'a> ProjectReq<'a> {
    /// Default creation of a Project API call
    fn new(slug: &'a str) -> Self {
        Self { id: slug, version: false }
    }
}

impl<'a> ToRequest for ProjectReq<'a> {
    fn to_req(&self) -> String {
        match self.version {
            true => format!("{}/project/{}/version", BASE_REQUEST, self.id),
            false => format!("{}/project/{}", BASE_REQUEST, self.id)
        }
    }
}

struct VersionReq<'a> {
    id: &'a str
}

impl<'a> VersionReq<'a> {
    fn new(id: &'a str) -> Self {
        Self{ id }
    }
}

impl<'a> ToRequest for VersionReq<'a> {
    fn to_req(&self) -> String {
        format!("{}/version/{}", BASE_REQUEST, self.id)
    }
}

/// The asynchronous interface to the Modrinth api
///
/// This type was designed in its use to be similar to creating an API call 
/// by hand. To make a search API call you would first start with the base HTTPS link:
/// `https://api.modrinth.com/v2/` and from there would build up your search with a query, index, 
/// and any other valid search parameters to get this: 
///
/// `https://api.modrinth.com/v2/search?query=sodium&facets=[["versions:1.19"]]`
///
/// With this struct creating a search would look like this:
///
/// ```rust
/// let modrinth = Modrinth::new();
/// modrinth.search("sodium").get().await?
/// ```
///
/// This mod is designed to not allow an invalid API call, this is done via the use 
/// of Generics and impl blocks. Modrinth has a generic type `ReqType` that implements an internal 
/// trait `ToRequest`. Methods to modify a Search API call are only available for Modrinth<SearchReq>. 
/// This makes it a compiler error to add the wrong query parameter to an API call. 
///
/// # Example: 
/// ```compile_fail
/// let modrinth = Modrinth::new();
/// // A project API call has no parameter involving versions 
/// // and as a result this method call is a compile error
/// modrinth.project().version("1.19");
/// ```
///
/// **NOTE** You only need one instance of a Modrinth type use it properly
///
/// # Example:
/// ```rust
/// let modrinth = Modrinth::new();
/// modrinth.search("sodium").get().await?
/// modrinth.search("fabric-api").version("1.19").get().await?
/// ```
pub struct Modrinth<ReqType> {
    // Client that makes HTTP requests
    client: Client,

    // The generic that determines what methods are available to the caller
    req_type: ReqType,
}

impl Modrinth<EmptyReq> {
    /// Creates a new Modrinth instance with an empty Request, calling `get()` on this 
    /// type is possible but is not reccommended because it is useless
    ///
    /// # Example:
    /// ```rust
    /// let modrinth = Modrinth::new() // new modrinth instance
    /// ```
    pub fn new() -> Self {
        Self {
            client: Client::default(),
            req_type: EmptyReq,
        }
    }

    /// Cheaply creates a Modrinth<SearchReq> instance 
    /// and takes a query as a required parameter
    ///
    /// # Example:
    ///
    /// ```rust
    /// let modrinth = Modrinth::new() // new modrinth instance
    /// // Creates a Modrinth<SearchReq> to begin building an API call
    /// let search = modrinth.search("sodium"); 
    /// ```
    pub fn search<'a>(&self, query: &'a str) -> Modrinth<SearchReq<'a>> {
        Modrinth {
            client: self.client.clone(),
            req_type: SearchReq::new(query),
        }
    }

    /// Cheaply creates a Modrinth<ProjectReq> instance and 
    /// takes a slug or id as a required parameter
    ///
    ///
    /// # Example:
    /// ```rust
    /// let modrinth = Modrinth::new() // new modrinth instance
    /// // Creates a Modrinth<ProjectReq> to begin building an API call
    /// let project =  modrinth.project("sodium") 
    /// ```
    ///
    /// **NOTE** Slugs can change but a project's id is constant
    pub fn project<'a>(&self, id: &'a str) -> Modrinth<ProjectReq<'a>> {
        Modrinth {
            client: self.client.clone(),
            req_type: ProjectReq::new(id),
        }
    }
    
    /// Cheaply creates a Modrinth<VersionReq> instance and 
    /// takes an id as a required parameter
    ///
    ///
    /// # Example:
    /// ```rust
    /// let modrinth = Modrinth::new() // new modrinth instance
    /// // Creates a Modrinth<VersionReq> to begin building an API call
    /// let project =  modrinth.version("AABBCCDD") 
    /// ```
    ///
    /// **NOTE** Slugs can change but a project's id is constant
    pub fn version<'a>(&self, id: &'a str) -> Modrinth<VersionReq<'a>> {
        Modrinth { 
            client: self.client.clone(), 
            req_type: VersionReq::new(id) 
        }
    }
}

impl<'a> Modrinth<SearchReq<'a>> {
    /// Add a version to filter search results 
    ///
    /// # Example:
    ///
    /// ```rust
    /// let modrinth = Modrinth::new()
    ///
    /// // Filters out mods that don't have 1.18 or 1.18.1 versions
    /// modrinth.search("sodium").version("1.18").version(1.18.2).get();
    /// ```
    /// 
    /// Each call to `version()` adds its passed value to a `Facet` 
    /// struct that holds all the data to construct a [facet](https://docs.modrinth.com/docs/tutorials/api_search/#facets) for the
    /// api call.
    pub fn version(&mut self, version: &'a str) -> &mut Self {
        self.req_type.facets.versions.push(version);
        self
    }

    /// Add an index to filter search results 
    ///
    /// # Example:
    /// ```rust
    /// let modrinth = Modrinth::new()
    ///
    /// // Filters out mods based on the index
    /// modrinth.search("sodium").index("downloads").get();
    /// ```
    /// 
    /// Each call to `index()` overwrites whatever value is already 
    /// there, "relevance" is the default index
    pub fn index(&mut self, index: &'a str) -> &mut Self {
        self.req_type.index = index;
        self
    }
}

impl <'a> Modrinth<ProjectReq<'a>> {
    /// A simple flag to list the projects versions
    ///
    /// # Example:
    /// ```rust
    /// let modrinth = Modrinth::new()
    ///
    /// // the returned JSON will have an array of all the project's versions
    /// modrinth.project("sodium").list_versions(true).get();
    /// ```
    pub fn list_versions(&mut self, yes: bool) -> &mut Self {
        self.req_type.version = yes;
        self
    }
}

impl<R: ToRequest> Modrinth<R> {
    /// sends a request to the Modrinth API and returns a Future to be awaited
    pub async fn get(&mut self) -> impl futures::Future<Output=reqwest::Result<Response>> {
        self.client.get(self.req_type.to_req()).send()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use httpmock;

    #[test]
    fn it_works() {
        let modrinth = Modrinth::new();
        modrinth.search("sodium").version("1.19").index("relevance").get();

   }
}
