use crate::Book;

mod api {
    use super::*;
    use rocket::{local::blocking::Client, http::Status};

    #[test]
    fn should_get_book() {
        let client = Client::tracked(crate::rocket()).unwrap();
        let response = client.get("/api").dispatch();
        assert_eq!(response.status(), Status::Ok);

        if let Some(data) = response.into_json::<Vec<Book>>() {
            assert!(data.contains(&Book{title:"A story".to_string()}))
        }
        else {panic!("response returned nothing")}
    }

    #[test]
    fn should_post_book() {
        let book = Book {title:  String::from("A Post")};

        let client = Client::tracked(crate::rocket()).unwrap();
        let post_response = client.post("/api")
            .json(&book)
            .dispatch();
        assert_eq!(post_response.status(), Status::Ok);

        let get_response = client.get("/api").dispatch();
        assert_eq!(get_response.status(), Status::Ok);
        if let Some(data) = get_response.into_json::<Vec<Book>>() {
            assert!(data.contains(&book))
        } else {panic!("Got no data")}
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
        let result = store();
        assert!(result.is_ok());
        let loaded = load();
        assert!(loaded.is_ok());
    }
}