use std::{fs::File, path::PathBuf};

use anyhow::Result;
use rocket::{serde::json::serde_json, log::private::debug};
use crate::Book;


pub fn load() -> Result<Vec<Book>> {
    let file = File::open(PathBuf::from("storage/db.json"))?;
    debug!("Loading data from: {:?}", file);
    let data = serde_json::from_reader(file)?;
    Ok(data)
}

pub fn store() -> Result<()> {
    unimplemented!()
}