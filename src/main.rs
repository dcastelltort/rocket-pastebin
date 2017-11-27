#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;

mod paste_id;
use paste_id::PasteID;

use std::io;
use std::path::Path;

use rocket::Data;

use std::fs::{self, File};
use rocket::http::RawStr;
use rocket::response::status;
use rocket::http::Status;

fn main() {
    rocket::ignite().mount("/", routes![index, upload, retrieve, delete]).launch();
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> Result<String, status::Custom<&'static str>> {
    
    let id = PasteID::new(32);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);
    
    // Write the paste out to the file and return the URL.
    match paste.stream_to_file(Path::new(&filename)) {
        Ok(_) => Ok(url),
        Err(_) => Err(status::Custom(Status::InternalServerError, "failed uploading"))
    }
    
    File::Write()
}

#[get("/<id>")]
fn retrieve(id: &RawStr) -> Option<File> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).ok()
}

#[delete("/<id>")]
fn delete(id: &RawStr) -> &str {
    let filename = format!("upload/{id}", id = id);
    match std::fs::remove_file(&filename) {
        Ok(_) => "",
        Err(_) => "delete failed"
    }
}