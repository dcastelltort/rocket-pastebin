use super::rocket;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn base_test() {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
}