use std::sync::RwLock;
use rocket::{serde::json::Json, get, http::Status};
use super::Book;

lazy_static! {
    static ref BOOKS: RwLock<Vec<Book>> = RwLock::new(vec!(Book {title:String::from("A story")}));
}

#[get("/")]
pub fn books() -> (Status, Option<Json<Vec<Book>>>) {
    
    if let Ok(list) = BOOKS.read() {
        (Status::Ok, Some(Json(list.clone())))
    }
    else {
        (Status::InternalServerError, None)
    }
}

#[post("/", format="json", data="<book>")]
pub fn add_book(book: Json<Book>) -> Status {
    if let Ok(mut list) = BOOKS.write() {
        list.push(book.0);
        Status::Ok
    }
    else {
        Status::InternalServerError
    }
}