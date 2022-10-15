#[macro_use] extern crate lazy_static;

use anyhow::Result;
use serde_derive::{Serialize, Deserialize};
use std::sync::RwLock;
use actix_web::{HttpServer, web, App};
use actix_files as fs;

#[cfg(test)]
mod tests;
mod api;
mod data_store;

#[tokio::main]
async fn main() -> Result<()>{

    if let Ok(books) = data_store::load() {
        BOOKS.write().unwrap().extend(books);
    }

    let server = HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
                .service(api::books)
                .service(api::add_book)
            )
            .service(fs::Files::new("/", "./web").index_file("index.html"))
    }).bind(("0.0.0.0", 8080))?
    .run()
    .await;

    data_store::store(BOOKS.read().unwrap().clone())?;

    server.map_err(|e| e.into())
}

lazy_static! {
    static ref BOOKS: RwLock<Vec<Book>> = RwLock::new(vec![]);
}


#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Book {
    pub title: String
}
