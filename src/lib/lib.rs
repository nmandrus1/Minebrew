// Modules Used -- These are all the files within the lib directory
mod shared;
mod project;
mod version;
mod search;

// These make it so that anyone who imports our library will be 
// able to use these types directly without having to import the 
// project module or search module
pub use project::Project;
pub use search::Search;
