use super::rocket;
use rocket::local::Client;
use rocket::http::Status;
use rocket::http::ContentType;
use super::paste_id;

#[test]
fn base_test() {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
}
#[test]
fn test_upload_retrieve() {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client.post("/")
                        .body("this is an upload test")
                        .header(ContentType::FormData)
                        .dispatch();
    
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    let id = response.body_string().unwrap();
    assert!(paste_id::valid_id(id.as_str()));

    let url = format!("/{id}", id = id);
    response = client.get(url).dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert!(response.body_string().unwrap().contains("this is an upload test"));
}