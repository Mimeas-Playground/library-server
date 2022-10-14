#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use std::net::{Ipv4Addr, IpAddr};
use rocket::{fs::{FileServer, relative}, serde::{Serialize, Deserialize}, Config};

#[cfg(test)]
mod tests;
mod api;
mod data_store;

#[launch]
fn rocket() -> _{
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
