use std::fmt::{Write, write};
use std::convert::From;

use reqwest::{Client, Response};

use super::shared::{ProjectType, Category};

// have to import this here bc stupid publicity rules abt traits
use super::traits::ToRequest;

const BASE_REQUEST: &str = "https://api.modrinth.com/v2";



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

        if ret.len() == 10 {
            ret.clear();
        }

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
        format!("{}/search?query={}&limit={}&index={}{}", 
            BASE_REQUEST, self.query, self.limit, self.index, self.facets.to_query_param())
    }
}

/// ProjectReq struct that implements ToRequest
struct ProjectReq<'a> {
    // slug: &'a str
    id: &'a str,
}

impl<'a> ProjectReq<'a> {
    /// Default creation of a Project API call
    fn new(slug: &'a str) -> Self {
        Self { id: slug }
    }
}

impl<'a> ToRequest for ProjectReq<'a> {
    fn to_req(&self) -> String {
        format!("{}/project/{}", BASE_REQUEST, self.id)
    }
}

struct ListVersionReq<'a> {
    id: &'a str,
    loaders: Option<Vec<&'a str>>,
    game_versions: Option<Vec<&'a str>>,
    featured: bool,
}

impl<'a> ListVersionReq<'a> {
    fn new(id: &'a str) -> Self {
        Self {
            id, 
            loaders: None,
            game_versions: None,
            featured: false,
        }
    }
}

impl<'a> ToRequest for ListVersionReq<'a> {
    fn to_req(&self) -> String {
        let mut req = String::from(format!("{}/project/{}/version", BASE_REQUEST, self.id));
        // the prefix of the query depends on whether or not its the first parameter
        let mut param_prefix = '?';

        // loop through loaders and add them to filter
        if let Some(loaders) = &self.loaders {
            write!(&mut req, "{}loaders=[", param_prefix);
            param_prefix = '&';
            
            loaders.iter().for_each(|l| write!(&mut req, "\"{}\",", l).unwrap());
            // after each loader is a comma, and since we're done there are no more 
            // loaders the comma is deleted and replaced with a bracket
            req.pop();
            req.push(']');
        }
        
        // loop through game versions and add them to filter
        if let Some(versions) = &self.game_versions {
            write!(&mut req, "{}game_versions=[", param_prefix);
            param_prefix = '&';

            versions.iter().for_each(|v| write!(&mut req, "\"{}\",", v).unwrap());
            req.pop();
            req.push(']');
        }

        if self.featured { write!(&mut req, "{}featured=true", param_prefix).unwrap() }

        req
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

    fn based(&self) {}

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
    /// A function to create an API call to 
    /// list all of the project's versions
    ///
    /// # Example:
    /// ```rust
    /// let modrinth = Modrinth::new()
    ///
    /// // the returned JSON will have an array of all the project's versions
    /// modrinth.project("sodium").list_versions().get();
    /// ```
    pub fn list_versions(&self) -> Modrinth<ListVersionReq<'a>> {
        Modrinth {
            client: self.client.clone(),
            req_type: ListVersionReq::new(self.req_type.id),
        }
    }
}

impl <'a> Modrinth<ListVersionReq<'a>> {
    /// Add a loader filter to the list of versions
    ///
    /// # Example:
    /// ```rust
    /// let modrinth = Modrinth::new()
    ///
    /// // the returned JSON will have an array of all the project's versions
    /// modrinth.project("sodium").list_versions().loader("fabric").get();
    /// ```
    pub fn loader(&mut self, loader: &'a str) -> &mut Self {
        // add loader to loaders or create first entry if there weren't any before
        if let Some(loaders) = &mut self.req_type.loaders {
            loaders.push(loader);
        } else { 
            self.req_type.loaders = Some(vec![loader]); 
        }

        self
    }
    
    /// Add a game_version filter to the list of versions
    ///
    /// # Example:
    /// ```rust
    /// let modrinth = Modrinth::new()
    ///
    /// // the returned JSON will have an array of all the project's versions
    /// modrinth.project("sodium").list_versions().game_version("1.19").get();
    /// ```
    pub fn game_version(&mut self, version: &'a str) -> &mut Self {
        // add version to game_versions or create first entry if there weren't any before
        if let Some(versions) = &mut self.req_type.game_versions {
            versions.push(version);
        } else { 
            self.req_type.game_versions = Some(vec![version]); 
        }

        self
    }
    
    /// Only include versions that are featured on the Mod's webpage
    /// in the returned list of versions
    ///
    /// # Example:
    /// ```rust
    /// let modrinth = Modrinth::new()
    ///
    /// // the returned JSON will have an array of all the project's versions
    /// modrinth.project("sodium").list_versions().featured(true).get();
    /// ```
    pub fn featured(&mut self, yes: bool) -> &mut Self {
        self.req_type.featured = true;
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

    use httpmock::prelude::*;

    const EMPTY_REQ_RESPONSE: &str = r#"{"about":"Welcome traveler!","documentation":"https://docs.modrinth.com","name":"modrinth-labrinth","version":"2.4.4"}"#;

    const SEARCH_REQ_RESPONSE: &str = r#"{"hits":[{"project_id":"AANobbMI","project_type":"mod","slug":"sodium","author":"jellysquid3","title":"Sodium","description":"Modern rendering engine and client-side optimization mod for Minecraft","categories":["optimization","fabric"],"versions":["1.16.3","1.16.4","1.16.5","1.17.1","1.18","1.18.1","1.18.2","1.19"],"downloads":354677,"follows":2293,"icon_url":"https://cdn.modrinth.com/data/AANobbMI/icon.png","date_created":"2021-01-03T00:53:34+00:00","date_modified":"2022-06-07T17:15:10+00:00","latest_version":"1.19","license":"lgpl-3","client_side":"required","server_side":"unsupported","gallery":[]}],"offset":0,"limit":1,"total_hits":2}"#;
    const SEARCH_REQ_RESPONSE_NO_FACET: &str = r#"{"hits":[{"project_id":"AANobbMI","project_type":"mod","slug":"sodium","author":"jellysquid3","title":"Sodium","description":"Modern rendering engine and client-side optimization mod for Minecraft","categories":["optimization","fabric"],"versions":["1.16.3","1.16.4","1.16.5","1.17.1","1.18","1.18.1","1.18.2","1.19"],"downloads":354677,"follows":2293,"icon_url":"https://cdn.modrinth.com/data/AANobbMI/icon.png","date_created":"2021-01-03T00:53:34+00:00","date_modified":"2022-06-07T17:15:10+00:00","latest_version":"1.19","license":"lgpl-3","client_side":"required","server_side":"unsupported","gallery":[]}],"offset":0,"limit":1,"total_hits":12}"#;

    const SLUG: &str = "sodium";
    const PID: &str = "AANobbMI";
    const PROJECT_REQ_RESPONSE: &str = r#"{"id":"AANobbMI","slug":"sodium","project_type":"mod","team":"4reLOAKe","title":"Sodium","description":"Modern rendering engine and client-side optimization mod for Minecraft","body":"![Sodium 0.4 Comparison](https://i.imgur.com/TzLurlG.png)\n\nSodium is a **free and open-source** rendering engine replacement for the Minecraft client which greatly improves frame rates and stuttering while fixing many graphical issues. It boasts wide compatibility with the Fabric mod ecosystem when compared to other mods, and it does so without compromising on how the game looks, giving you that authentic block game feel.\n\n## 📥 Installation guide\n\nMake sure you have the latest version of [Fabric Loader](https://fabricmc.net/use/) installed. Afterwards, all you need to do is simply drop the mod into your mods folder. No other mods or downloads are required in order to use Sodium. You do not need to create new worlds in order to take advantage of the mod.\n\nNot sure if you installed the mod correctly? Try checking your _Video Settings_ screen in Minecraft, which should show our new and improved user interface for changing settings.\n\nOut of the box, Sodium will enable all optimizations which are supported on your system, meaning no additional configuration is needed. You should generally only change video settings related to performance and other advanced features if you are experiencing issues.\n\n➡️ **Note:** By design, Sodium only optimizes the client rendering code. You should also install our other mods, such as [Lithium](https://modrinth.com/mod/lithium) and [Phosphor](https://modrinth.com/mod/phosphor), to optimize the other parts of your game. This is done so that players can pick and choose which mods they want to use, but we generally recommend using [our entire collection](https://modrinth.com/user/jellysquid3).\n\n## 🔥 Performance\n\nThe following performance comparisons have been contributed by our community, and show how Sodium can improve frame rates for a wide range of computers, whether fast or slow. Many of our players report a **250% to 500% increase** in average frame rates.\n\n- AMD Ryzen 5 2600 / AMD Radeon RX 580 ([before](https://cdn.discordapp.com/attachments/948635624729112596/949043152138956850/2022-03-03_21.34.58.png) 88 fps, [after](https://cdn.discordapp.com/attachments/948635624729112596/949043152810041384/2022-03-03_21.36.59.png) 418 fps)\n- Intel Core i3-6100U / Intel HD Graphics 520 ([before](https://cdn.discordapp.com/attachments/948635624729112596/948649845651025930/2022-03-02_13.31.51.png) 17 fps, [after](https://cdn.discordapp.com/attachments/948635624729112596/948649845961396294/2022-03-02_13.24.50.png) 73 fps)\n- AMD Ryzen 7 3700X / NVIDIA RTX 3080 ([before](https://cdn.discordapp.com/attachments/948635624729112596/948678492634120292/2022-03-02_21.28.37.png) 61 fps, [after](https://cdn.discordapp.com/attachments/948635624729112596/948678830430769182/2022-03-02_21.26.02.png) 251 fps)\n- AMD Ryzen 3 3200G / AMD Vega 8 Graphics ([before](https://cdn.discordapp.com/attachments/948635624729112596/948659691125350410/2022-03-02_19.13.34.png) 58 fps, [after](https://cdn.discordapp.com/attachments/948635624729112596/948659690802380840/2022-03-02_19.10.59.png) 173 fps)\n- Intel Core i5-3330 / NVIDIA GeForce GT 1030 ([before](https://cdn.discordapp.com/attachments/948635624729112596/948704894758580284/2022-03-02_23.13.44.png) 36 fps, [after](https://cdn.discordapp.com/attachments/948635624729112596/948704895362551879/2022-03-02_23.03.29.png) 89 fps) \n- Intel Core i7-10700K / NVIDIA GTX 1660 SUPER ([before](https://cdn.discordapp.com/attachments/948635624729112596/948645822730489906/2022-03-02_19.16.28.png) 81 fps, [after](https://cdn.discordapp.com/attachments/948635624729112596/948645834067689483/2022-03-02_19.18.16.png) 256 fps)\n- Intel Core i7-1165G7 / NVIDIA GeForce MX450 ([before](https://cdn.discordapp.com/attachments/948635624729112596/948643441926086746/2022-03-02_13.09.37.png) 45 fps, [after](https://cdn.discordapp.com/attachments/948635624729112596/948643441305333800/2022-03-02_13.04.25.png) 156 fps)\n\nEven for very slow or unusual machines, people often report significant improvements.\n\n- AMD Athlon II M300 / ATI Mobility Radeon HD 4500/5100 ([before](https://cdn.discordapp.com/attachments/948635624729112596/948660512244244490/2022-03-03_00.00.28.png) 9 fps, [after](https://cdn.discordapp.com/attachments/948635624729112596/948660512487518318/2022-03-03_00.12.49.png) 47 fps)\n- Nintendo Switch / ARMv8 Quad-Core CPU / NVIDIA Tegra X1 ([before](https://cdn.discordapp.com/attachments/948635624729112596/949130022797586452/2022-03-03_21.19.39.png) 37 fps, [after](https://cdn.discordapp.com/attachments/948635624729112596/949130039285383208/2022-03-03_21.11.34.png) 108 fps)\n\n## ✅ Compatibility\n\nSodium is mostly stable at this point, but it does not yet contain support for the Fabric Rendering API, which a small number of mods currently use. If you try to use these mods with Sodium, your game may crash or behave unexpectedly.\n\nWe try to ensure compatibility with most graphics cards that have up-to-date drivers for OpenGL 4.6 Core, which covers most graphics cards released after mid-2010.\n\n- INTEL HD Graphics 500 Series (Skylake) or newer\n- NVIDIA GeForce 400 Series (Fermi) or newer\n- AMD Radeon HD 7000 Series (GCN 1) or newer\n \nOlder graphics cards may also work (so long as they have up-to-date drivers for at least OpenGL 3.3 Core), but hardware requirements are subject to change in future releases.\n\nIf you encounter issues with Sodium, you should [make sure that your graphics drivers are up-to-date](https://help.minecraft.net/hc/en-us/articles/4409137348877-Minecraft-Java-Edition-Game-Crashes-and-Performance-Issues-FAQ#h_01FFNR5S1APTFHGR9K2W3JANZ4), as this is the most often the culprit of poor performance, crashes, and rendering bugs.\n\n## ⚙️ Technical details\n\nOf course, we can't just say that the game is magically faster without providing some kind of explanation. This list tries to cover some of the most significant changes which are responsible for performance improvements, but it's not complete or exhaustive of everything Sodium does.\n\n- A modern OpenGL rendering pipeline for chunk rendering that takes advantage of multi-draw techniques, allowing for a significant reduction in CPU overhead (~90%) when rendering the world. This can make a huge difference to frame rates for most computers that are not bottle-necked by the GPU or other components. Even if your GPU can't keep up, you'll experience much more stable frame times thanks to the CPU being able to work on other rendering tasks while it waits.\n\n- Vertex data for rendered chunks is made much more compact, allowing for video memory and bandwidth requirements to be cut by almost 40%.\n\n- Nearby block updates now take advantage of multi-threading, greatly reducing lag spikes caused by chunks needing to be updated. ([before](https://streamable.com/lm5sp5), [after](https://streamable.com/nsdl0r))\n\n- Chunk faces which are not visible (or facing away from the camera) are culled very early in the rendering process, eliminating a ton of geometry that would have to be processed on the GPU only to be immediately discarded. For integrated GPUs, this can greatly reduce memory bandwidth requirements and provide a modest speedup even when GPU-bound.\n\n- Plentiful optimizations for chunk loading and block rendering, making chunk loading significantly faster and less damaging to frame rates. ([before](https://streamable.com/3taw22), [after](https://streamable.com/4pesh2))\n\n- Many optimizations for vertex building and matrix transformations, speeding up block entity, mob, and item rendering significantly for when you get carried away placing too many chests in one room.\n\n- Many improvements to how the game manages memory and allocates objects, which in turn reduces memory consumption and lag spikes caused by garbage collector activity.\n\n- Many graphical fixes for smooth lighting effects, making the game run better while still applying a healthy amount of optimization. For example, take this [before and after](https://i.imgur.com/lYmlmgq.png) of a white concrete room in vanilla, or this [comparison while underwater](https://i.imgur.com/QQMuOTy.png).\n\n- Smooth lighting for fluids and other special blocks. ([comparison](https://i.imgur.com/z9HBcvq.png))\n\n- Smooth biome blending for blocks and fluids, providing greatly improved graphical quality that is significantly less computationally intensive.  ([comparison](https://i.imgur.com/Fud5oyF.png))\n\n- Animated textures which are not visible in the world are not updated, speeding up texture updating on most hardware (especially AMD cards.)\n\n... and much more, this list is still being written after the initial release.\n\n## 🐛 Reporting Issues\n\nPlease use the [issue tracker](https://github.com/jellysquid3/sodium-fabric/issues) linked at the top of the page to report bugs, crashes, and other issues.\n\n## ❓ Frequently Asked Questions\n\nWe have a short wiki with some of the most frequently asked questions [here](https://github.com/CaffeineMC/caffeine-meta/wiki/FAQ). More likely than not, your question already has an answer here.","body_url":"https://cdn.modrinth.com/data/AANobbMI/description.md","published":"2021-01-03T00:53:34+00:00","updated":"2022-06-07T17:15:10+00:00","status":"approved","moderator_message":null,"license":{"id":"lgpl-3","name":"GNU Lesser General Public License v3","url":"https://cdn.modrinth.com/licenses/lgpl-3.txt"},"client_side":"required","server_side":"unsupported","downloads":354690,"followers":2293,"categories":["optimization"],"versions":["3JJvf9Kn","YAGZ1cCS","Yp8wLY1P","5JyduDNN","xuWxRZPd","yaoBL9D9","6YGRDUVT","74Y5Z8fo","80jYkEVr","1b0GhKHj","Fz37KqRh"],"icon_url":"https://cdn.modrinth.com/data/AANobbMI/icon.png","issues_url":"https://github.com/CaffeineMC/sodium-fabric/issues","source_url":"https://github.com/CaffeineMC/sodium-fabric","wiki_url":null,"discord_url":"https://jellysquid.me/discord","donation_urls":[{"id":"ko-fi","platform":"Ko-fi","url":"https://ko-fi.com/jellysquid_"}],"gallery":[]}"#;
    const PROJECT_REQ_LIST_VERSION: &str = r#""#;
    
    // version id for sodium
    const VID: &str = "Yp8wLY1P";
    const VERSION_REQ_RESPONSE: &str = r#"{"id":"Yp8wLY1P","project_id":"AANobbMI","author_id":"DzLrfrbK","featured":true,"name":"Sodium 0.4.2","version_number":"mc1.19-0.4.2","changelog":"Sodium 0.4.2 for Minecraft 1.19 is now out.\nThis is a straight port of the previous release for Minecraft 1.19.","changelog_url":null,"date_published":"2022-06-07T17:15:16+00:00","downloads":72116,"version_type":"release","files":[{"hashes":{"sha1":"6c1b055bce99d0bf64733e0ff95f347e4cd171f3","sha512":"95589fcca80f77aca8e38634927bfb7a5bd5b31b7f34c09352cc7724541b9efe8bbe1d7c1a39afcdbf67fa38f5871355ccb56817027bf6028255393c7174e450"},"url":"https://cdn.modrinth.com/data/AANobbMI/versions/mc1.19-0.4.2/sodium-fabric-mc1.19-0.4.2%2Bbuild.16.jar","filename":"sodium-fabric-mc1.19-0.4.2+build.16.jar","primary":true,"size":1364166}],"dependencies":[],"game_versions":["1.19"],"loaders":["fabric"]}"#;

    #[track_caller]
    fn check<R: ToRequest>(req_type: R, expected: String) {
        let actual = req_type.to_req();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_empty_req_resp() {
        let server = MockServer::start();
        let m = server.mock(|when, then| {
            when.method(GET)
                .path("");
            then.status(201)
                .header("content-type", "application/json")
                .json_body(EMPTY_REQ_RESPONSE);
        });

        let modrinth = Modrinth::new();
    }

    #[test]
    fn empty_req() {
        let expected = "https://api.modrinth.com/v2";
        check(Modrinth::new().req_type, expected.into());
    }

    fn basic_search() {
        let expected = "https://api.modrinth.com/v2/search?query=sodium&limit=5&index=relevance";
        let search_req = Modrinth::new().search("sodium").req_type;
        check(search_req, expected.into());
    }

    #[test]
    fn to_req_test() {
        let modrinth = Modrinth::new();

        // searches
        let search_basic = modrinth.search("sodium").req_type.to_req();
        let search_index = modrinth.search("sodium").index("downloads").req_type.to_req();
        let search_facet = modrinth.search("sodium").version("1.19").req_type.to_req();
        
        // projects
        let project_basic = modrinth.project("sodium").req_type.to_req();
        let project_list_versions = modrinth.project("sodium").list_versions().req_type.to_req();
        let project_list_versions_parameters = 
            modrinth.project("sodium").list_versions().game_version("1.19").featured(true).req_type.to_req();
    }
}
