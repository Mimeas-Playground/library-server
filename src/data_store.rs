use std::{fs::{File, create_dir_all}, path::PathBuf, io::{Write, BufReader}};

use anyhow::Result;
use crate::Book;


pub fn load() -> Result<Vec<Book>> {
    let file = File::open(PathBuf::from("storage/db.json"))?;
    let reader = BufReader::new(file);
    
    let data = serde_json::from_reader(reader)?;

    Ok(data)
}

pub fn store(data: Vec<Book>) -> Result<()> {
    create_dir_all(PathBuf::from("storage"))?;
    
    let mut file = File::create(PathBuf::from("storage/db.json"))?;
    file.write_all(serde_json::ser::to_string(&data)?.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}