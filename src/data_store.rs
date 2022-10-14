use std::{fs::File, path::PathBuf, io::{Write, BufReader}};

use anyhow::Result;
use rocket::{serde::json::serde_json, log::private::debug};
use crate::Book;


pub fn load() -> Result<Vec<Book>> {
    let file = File::open(PathBuf::from("storage/db.json"))?;
    debug!("Loading data from: {:?}", file);
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

pub fn store(data: Vec<Book>) -> Result<()> {
    let mut file = File::create(PathBuf::from("storage/db.json"))?;
    debug!("Storing data to: {:?}", file);
    file.write_all(serde_json::ser::to_string(&data)?.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}