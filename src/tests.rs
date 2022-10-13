use rocket::local::blocking::Client;
mod api {
    use rocket::{http::Status};
    use super::*;

    #[test]
    fn should_get_book() {
        let client = Client::tracked(crate::rocket()).unwrap();
        let response = client.get("/api").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let data = response.into_string();
        if let Some(dat) = data {
            assert!(dat.contains("\"title\":\"A story\""))
        }
        else {panic!("response returned nothing")}
    }
}