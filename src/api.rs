use super::Book;
use rocket::{serde::json::Json, get};

#[get("/")]
pub fn get_books() -> Json<Vec<Book>> {
    let book_list = vec![Book{title: String::from("A story")}];

    Json(book_list)
}
