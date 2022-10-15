use actix_web::{web, HttpResponse, get, post};

use crate::BOOKS;

use super::Book;

#[get("/")]
pub async fn books() -> HttpResponse {

    
    if let Ok(list) = BOOKS.read() {
        HttpResponse::Ok().json(list.clone())
    }
    else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/")]
pub async fn add_book(book: web::Json<Book>) -> HttpResponse {
    if let Ok(mut list) = BOOKS.write() {
        list.push(book.into_inner());

        HttpResponse::Ok().finish()
    }
    else {
        HttpResponse::InternalServerError().finish()
    }
}