use std::sync::RwLock;

use actix_web::{web, HttpResponse, get, post};

use super::Book;

#[get("/")]
pub async fn books(books: web::Data<RwLock<Vec<Book>>>) -> HttpResponse {
    
    if let Ok(list) = books.read() {
        HttpResponse::Ok().json(list.clone())
    }
    else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/")]
pub async fn add_book(book: web::Json<Book>, book_list: web::Data<RwLock<Vec<Book>>>) -> HttpResponse {
    if let Ok(mut list) = book_list.write() {
        list.push(book.into_inner());

        HttpResponse::Ok().finish()
    }
    else {
        HttpResponse::InternalServerError().finish()
    }
}