#[macro_use] extern crate lazy_static;

use anyhow::Result;
#[allow(unused_imports)]
use log::{warn, error, info};
use serde_derive::{Serialize, Deserialize};
use std::{sync::RwLock};
use actix_web::{HttpServer, web, App, middleware::Logger};
use actix_files::Files as FileServer;
use env_logger::Env;

#[cfg(test)]
mod tests;
mod api;
mod data_store;

#[tokio::main]
async fn main() -> Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let app_path = match std::env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to get working dir {:}", e);
            panic!();
        }
    };

    let mut web_path = app_path.clone();
    web_path.push("web");
        
    let port_env = match std::env::var("PORT") {
        Ok(str) => str,
        Err(_) => {
            warn!("No PORT env");
            String::from("8080")
        }
    };

    let port: u16 = match port_env.parse() {
        Ok(port) => port,
        Err(_) => {
            warn!("Failed to parse PORT: {:}", port_env);
            8080
        }
    };

    if let Ok(books) = data_store::load() {
        BOOKS.write().unwrap().extend(books);
    }

    let server = HttpServer::new(move || {
        App::new()
            // add  default logging middleware
            .wrap(Logger::default())
            //  Add Api endpoints
            .service(web::scope("/api")
                .service(api::books)
                .service(api::add_book)
            )
            //  Add static fileserver
            .service(FileServer::new("/", web_path.to_str().unwrap())
                .index_file("index.html")
            )
    }).bind(("0.0.0.0", port))?
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
