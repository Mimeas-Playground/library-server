use rocket::local::blocking::Client;
use crate::Book;

mod api {
    use rocket::{http::Status};

    use super::*;

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