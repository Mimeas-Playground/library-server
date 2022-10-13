use rocket::{Error, fs::{FileServer, relative}, serde::{self, ser::SerializeStruct}};

#[macro_use] extern crate rocket;

#[main]
async fn main() -> Result<(), Error>{
    let _ = rocket::build()
    .mount("/", FileServer::from(relative!("web")))
    .mount("/api", routes![api::get_books])
    .launch().await?;

    Ok(())
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

pub struct Book {
    pub title: String
}

impl serde::Serialize for Book {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut s =serializer.serialize_struct("Book", 1)?;
        s.serialize_field("title", &self.title)?;
        s.end()
    }
}