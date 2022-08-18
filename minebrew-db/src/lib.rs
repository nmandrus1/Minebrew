use std::collections::HashMap;
use std::path::Path;
use std::error;

use minebrew_lib::Version;

use serde::{Serialize, Deserialize};

/// The minebrew mod database that lives on the users local machine
/// where the Key to the map is the Version's project_id and 
/// the value is the Version struct
#[derive(Serialize, Deserialize)]
pub struct ModDB(HashMap<String, Version>);

impl ModDB {
    /// Loads the local database from a given path
    pub fn load(path: &Path) -> Result<Self, DBError> {
        let string = match std::fs::read_to_string(path) {
            Ok(s) => { 
                println!("Found local database...");
                s
            },
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => { 
                    println!("Database not found...\nCreating local database...");
                    return Ok(ModDB::default())
                },
                _ => return Err(DBError::IOError(e)),
            }
        };

        serde_json::from_str::<Self>(&string)
            .map_err(|e| DBError::SerializationErr(e))
    }
    
    pub fn get(&self, key: &str) -> Option<&Version> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Version> {
        self.0.get_mut(key)
    }
    
    pub fn insert(&mut self, key: String, val: Version) -> Option<Version> {
        self.0.insert(key, val)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn entry(&mut self, key: String) -> std::collections::hash_map::Entry<String, Version> {
        self.0.entry(key)
    }

    /// save the contents of the Mod Database to the path passed to the fn
    pub fn save_to_file(&self, path: &Path) -> Result<(), DBError>{
        let json = match serde_json::to_string(self) {
            Ok(s) => s,
            Err(e) => return Err(DBError::SerializationErr(e))
        };

        println!("DEBUGING DELETE LATER!");
        self.0.iter().for_each(|(k, v)| println!("pid: {k}, mod: {}", v.name));

        std::fs::write(path, &json).map_err(|e| DBError::IOError(e))
    }
}

impl Default for ModDB {
    fn default() -> Self {
        Self(HashMap::with_capacity(5))
    }
}

/// Custom Error types for the errors that could occur while 
/// loading the Database
#[derive(Debug)]
pub enum DBError {
   IOError(std::io::Error),
   SerializationErr(serde_json::Error)
}

impl error::Error for DBError {}

impl std::fmt::Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(e) => write!(f, "{}", e),
            Self::SerializationErr(e) => write!(f, "{}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
