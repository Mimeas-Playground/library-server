use rocket::{Error, fs::{FileServer, relative}, serde::{Serialize, Deserialize}};

#[macro_use] extern crate rocket;

#[cfg(test)]
mod tests;

#[launch]
fn rocket() -> _{
   rocket::build()
    .mount("/", FileServer::from(relative!("web")))
    .mount("/api", routes![api::get_books])
}

mod api {
    use super::Book;
    use rocket::{serde::json::Json, get};

    #[get("/")]
    pub fn get_books() -> Json<Vec<Book>> {
        let book_list = vec![Book{title: String::from("A story")}];

        Json(book_list)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct Book {
    pub title: String
}
