use crate::Book;

mod api {
    use super::*;
    use crate::{api as endpoint, BOOKS};

    use actix_web::{test, App};

    #[actix_web::test]
    async fn should_get_book() {
        let book = Book {
            title: "The Hobbit".to_string()
        };
        // Ensures that the list has at least one element
        BOOKS.write().unwrap().push(book.clone());

        let app = test::init_service(
            App::new()
                .service(endpoint::books)
        ).await;

        let req = test::TestRequest::get()
            .uri("/")
        .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: Vec<Book> = test::read_body_json(resp).await;
        assert_eq!(body, BOOKS.read().unwrap().clone());
    }

    #[actix_web::test]
    async fn should_post_book() {
        let book = Book {title:  String::from("A Post")};

        BOOKS.write().unwrap().clear();

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