#[macro_use] extern crate lazy_static;

use std::sync::RwLock;
use actix_web::{HttpServer, Scope, web, App};
use actix_files as fs;
use anyhow::Result;

#[cfg(test)]
mod tests;
mod api;
mod data_store;

lazy_static! {
    static ref BOOKS: RwLock<Vec<Book>> = {
        if let Ok(books) = data_store::load() {
            RwLock::new(books)
        } else {
            RwLock::new(vec![Book{title: String::from("Default book")}])
        }
    };
}

#[actix_web::main]
async fn main() -> Result<()>{

    let books;
    if let Ok(loaded) = data_store::load() {
        books = loaded;
    } else {
        books = vec![Book{title: String::from("Default book")}]
    }

    let server = HttpServer::new(
        App::new()
            .app_data(web::Data::new(RwLock::new(&mut books)))
            .service(web::scope("/api")
                .service(api::books)
                .service(api::add_book)
            )
            .service(fs::Files::new("/", "./web").index_file("index.html"))
        )
        .bind(("0.0.0.0", 8080))?
        .run()
    .await;

    data_store::store(books)?;

    Ok(())
}


#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(crate="rocket::serde")]
pub struct Book {
    pub title: String
}
