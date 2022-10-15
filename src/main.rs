#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use rocket::{fs::{FileServer, relative}, serde::{Serialize, Deserialize}, Config, Rocket, Build};
use std::sync::RwLock;
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

#[main]
async fn main() -> Result<()>{
    let serv = rocket().launch().await;

    if let Ok(books) = BOOKS.read() {
        data_store::store(books.clone())?;
    }

    if let Err(err) = serv {
        error!("Error: {:?}", err);
    }

    Ok(())
}

fn rocket() -> Rocket<Build> {
    let mut config: Config;
    #[cfg(debug_assertions)]
    {
        config = rocket::Config::debug_default();
        config.port = 8080;
    }
    #[cfg(not(debug_assertions))]
    {
        config = rocket::Config::release_default();
        config.port = 0;
        config.log_level = rocket::log::LogLevel::Normal;
        config.address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    }

    rocket::build()
    .configure(config)
    .mount("/", FileServer::from(relative!("web")))
    .mount("/api", routes![api::books, api::add_book])
}


#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(crate="rocket::serde")]
pub struct Book {
    pub title: String
}
