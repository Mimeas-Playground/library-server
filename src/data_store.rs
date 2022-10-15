use std::{fs::{File, create_dir_all}, path::PathBuf, io::{Write, Read}};

use anyhow::Result;
use crate::Book;

lazy_static! {
    static ref STORAGE_PATH: PathBuf = std::env::current_dir().unwrap().join("storage").join("db.json");
}


pub fn load() -> Result<Vec<Book>> {
    let mut file = File::open(STORAGE_PATH.clone())?;

    let mut contents = vec![];
    file.read_to_end(&mut contents)?;
    
    let data = serde_json::from_slice(&contents)?;

    Ok(data)
}

pub fn store(data: Vec<Book>) -> Result<()> {
    create_dir_all(PathBuf::from("storage"))?;
    
    let mut file = File::create(STORAGE_PATH.clone())?;
    file.write_all(serde_json::ser::to_string(&data)?.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}