use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use sled::{Db, IVec};
#[derive(Clone)]
pub struct Cache {
    file_path : String,
    index : Vec<String>
} 
 
enum Error {
    NotFound(String)
}

impl Cache{
    pub fn open(self) -> sled::Result<Db>{
        // works like std::fs::open
        let db = sled::open(self.file_path);
        db
    }
    pub fn insert(mut self,key : String, value : String)-> Result<std::option::Option<IVec>, sled::Error> {
        let db = self.clone().open();
        if let Ok(db) = db{
            self.index.push(key.clone());
            db.insert(key,value.as_bytes())
        }else{
            Err(sled::Error::CollectionNotFound(IVec::from(key.as_bytes())))
        }
    }
    pub fn get(self,key : &str) -> sled::Result<Option<IVec>> {
        let db = self.open();
        if let Ok(db) = db{
            db.get(key)
        }else{
            Err(sled::Error::CollectionNotFound(IVec::from(key.as_bytes())))
        }
    }

    fn get_index(&self) -> Vec<String>{
        self.index.iter().map(move|f| f.to_owned()).collect()
    }

    pub fn get_all(&self) -> HashMap<String,String> {
        let indexs = &self.clone().get_index();
        indexs.iter().map(|f|match &self.clone().get(f.clone().as_str()) {
            Ok(c) => (f.to_owned(),format!("{}",String::from_utf8_lossy(&c.as_ref().unwrap()))),
            Err(_) => (f.to_owned(),format!("{}","")),
        }).collect::<HashMap<String,String>>()
    }

    pub fn search(&self,keyword: String ) -> Result<String,Error>{

        self.clone().get_all().par_iter().map(|(k,v)| v) 
    }
} 
impl Default for Cache {
    fn default()-> Self{
        Cache{
            file_path : "data_cache".to_string(),
            index: vec![String::from("");usize::MAX],
        }
    }
}