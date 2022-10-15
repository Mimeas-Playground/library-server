use crate::Book;

mod api {
    use super::*;
    use crate::api as endpoint;

    use actix_web::{test, App, web};

    #[actix_web::test]
    async fn should_get_book() {
        let app = test::init_service(
            App::new()
            .service(web::scope("/api")
                .service(endpoint::books)
            )
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/books")
        .to_request();
        
        let response = test::call_service(&app, req).await;
        assert!(response.status().is_success());
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
        let verify_req = test::TestRequest::get()
            .uri("/")
        .to_request();

        let verify_response: Vec<Book> = test::call_and_read_body_json(&app, verify_req).await;
        assert!(verify_response.contains(&book));
    }
}

mod data_store {
    use super::*;
    use anyhow::Result;
    use crate::data_store::{load, store};

    #[test]
    fn should_load_stored_data() {
        let loaded: Result<Vec<Book>> = load();
        
        if let Ok(data) = loaded {
            assert!(!data.is_empty());
        }
        else {
            panic!("{:?}", loaded)
        }
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