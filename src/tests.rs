use crate::Book;

mod api {
    use super::*;
    use crate::{api as endpoint, BOOKS};

    use actix_web::{test, App, web};

    #[actix_web::test]
    async fn should_get_book() {
        let book = Book { title: "The Hobbit".to_string() };

        BOOKS.write().unwrap()
            .push(book.clone());

        let app = test::init_service(
            App::new()
            .service(web::scope("/api")
                .service(endpoint::books)
            )
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/books")
        .to_request();
        
        let response: Vec<Book> = test::call_and_read_body_json(&app, req).await;
        assert!(response.contains(&book));
    }

    #[actix_web::test]
    async fn should_post_book() {
        let book = Book {title:  String::from("A Post")};

        let app = test::init_service(
            App::new()
                .service(endpoint::add_book)
                .service(endpoint::books)
        ).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&book)
        .to_request();
        
        let response = test::call_service(&app, req).await;
        assert!(response.status().is_success());
        
        //  Verify that book actually was added to server
        assert!(BOOKS.read().unwrap().contains(&book));
    }
}

mod data_store {
    use super::*;
    use anyhow::Result;
    use crate::data_store::{load, store};

    #[test]
    fn should_load_stored_data() {
        assert!(load().is_ok());
    }

    #[test]
    fn should_store_data() {
        let result = store(vec![Book {title: String::from("A story")}]);
        if result.is_err() {
            panic!("{:?}", result)
        }
        if let Ok(loaded) = load() {
            assert!(loaded.contains(&Book {title: String::from("A story")}));
        } else {
            panic!()
        }
    }
}