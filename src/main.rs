#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;

mod paste_id;
use paste_id::PasteID;

use std::io;
use std::path::Path;

use rocket::Data;

use std::fs::File;
use rocket::http::RawStr;

fn main() {
    rocket::ignite().mount("/", routes![index, upload, retrieve]).launch();
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
fn upload(paste: Data) -> io::Result<String> {
    
    let id = PasteID::new(32);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);
    
    // Write the paste out to the file and return the URL.
    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}

#[get("/<id>")]
fn retrieve(id: &RawStr) -> Option<File> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).ok()
}