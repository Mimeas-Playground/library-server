use rocket::{fs::{FileServer, relative}, serde::{Serialize, Deserialize}};

#[macro_use] extern crate rocket;

#[cfg(test)]
mod tests;

mod api;

#[launch]
fn rocket() -> _{
   rocket::build()
    .mount("/", FileServer::from(relative!("web")))
    .mount("/api", routes![api::get_books])
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct Book {
    pub title: String
}
