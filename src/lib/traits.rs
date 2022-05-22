// This file contains all the traits that will let us build our binary 
// in a platform (at the moment modrinth vs curseforge) generic way 

pub trait ModSearchResult {
    fn slug(&self) -> &str;
    fn description(&self) -> &str;
    fn title(&self) -> &str;
    fn id(&self) -> &str;
}
