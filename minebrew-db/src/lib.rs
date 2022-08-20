use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::error;

use minebrew_lib::Version;

use serde::{Serialize, Deserialize};

/// The minebrew mod database that lives on the users local machine
/// where the Key to the map is the Version's project_id and 
/// the value is the Version struct
#[derive(Serialize, Deserialize)]
pub struct ModDB{
    db: HashMap<String, Version>,
    // the parent directory of the database (the .minecraft folder)
    dir: PathBuf
}

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

        let db = match serde_json::from_str::<HashMap<String, Version>>(&string) {
            Ok(db) => db,
            Err(e) => return Err(DBError::SerializationErr(e))
        };
        
        let mut dir = path.to_path_buf();
        dir.pop();
        
        Ok(Self {db, dir})
    }
    
    pub fn get(&self, key: &str) -> Option<&Version> {
        self.db.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Version> {
        self.db.get_mut(key)
    }
    
    pub fn insert(&mut self, key: String, val: Version) -> Option<Version> {
        self.db.insert(key, val)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.db.contains_key(key)
    }

    pub fn entry(&mut self, key: String) -> std::collections::hash_map::Entry<String, Version> {
        self.db.entry(key)
    }

    /// replace a mod in the database with a new one if its being updated
    /// or insert a new mod into the database
    pub fn replace_or_insert(&mut self, new: Version) {
        self.dir.push("mods");
        match self.db.get_mut(new.pid()) {
            Some(old_v) => {
                self.dir.push(old_v.file().file_name());
                std::fs::remove_file(&self.dir);
                self.dir.pop(); // return its state back to the (.minecraft folder)
                self.dir.pop();
                *old_v = new
            },
            None => { self.db.insert(new.pid().to_string(), new); }
        }
    }

    /// save the contents of the Mod Database to the path passed to the fn
    pub fn save_to_file(&mut self) -> Result<(), DBError>{
        self.dir.push("minebrew.json");
        
        let json = match serde_json::to_string(self) {
            Ok(s) => s,
            Err(e) => return Err(DBError::SerializationErr(e))
        };

        println!("DEBUGING DELETE LATER!");
        self.db.iter().for_each(|(k, v)| println!("pid: {k}, mod: {}", v.name));

        std::fs::write(&self.dir, &json).map_err(|e| DBError::IOError(e))
    }
}

impl Default for ModDB {
    fn default() -> Self {
        Self{
            db: HashMap::with_capacity(5),
            dir: PathBuf::default(),
        }
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
