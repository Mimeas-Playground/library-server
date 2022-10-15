use std::{fs::{File, create_dir_all}, path::PathBuf, io::{Write, Read}};

use anyhow::Result;
use crate::Book;


pub fn load() -> Result<Vec<Book>> {
    let mut file = File::open(PathBuf::from("storage/db.json"))?;

    let mut contents = vec![];
    file.read_to_end(&mut contents)?;
    
    let data = serde_json::from_slice(&contents)?;

    Ok(data)
}

pub fn store(data: Vec<Book>) -> Result<()> {
    create_dir_all(PathBuf::from("storage"))?;
    
    let mut file = File::create(PathBuf::from("storage/db.json"))?;
    file.write_all(serde_json::ser::to_string(&data)?.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}