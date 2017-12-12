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
fn test_upload_retrieve_update() {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    
    //test upload
    let mut response = client.post("/")
                        .body("this is an upload test")
                        .header(ContentType::FormData)
                        .dispatch();
    
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
   
    //test retrieve
    let id = response.body_string().unwrap();
    assert!(paste_id::valid_id(id.as_str()));

    let url = format!("/{id}", id = id);
    response = client.get(url.clone()).dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert!(response.body_string().unwrap().contains("this is an upload test"));

    //test update
    response = client.put(url.clone())
                .body("updating the file")
                .header(ContentType::FormData)
                .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    let same_id = response.body_string().unwrap();
    assert_eq!(id, same_id);

    response = client.get(url.clone()).dispatch(); //check it's updated for real
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body_string().unwrap().contains("updating the file"));

    //test delete
    response = client.delete(url.clone()).dispatch();
    assert_eq!(response.status(), Status::Ok);
    response = client.get(url.clone()).dispatch(); //check it's deleted for real
    assert_eq!(response.status(), Status::NotFound);

}