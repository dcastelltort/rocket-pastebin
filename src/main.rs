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
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use rocket::http::RawStr;
use rocket::response::status;
use rocket::response::content;
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
fn upload(paste: Data) -> Result< content::Plain<String> , status::Custom<&'static str> > {
    
    let id = PasteID::new(32);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);
    
    // Write the paste out to the file and return the URL.
    let mut read_size = 0;
    const MAX_UPLOAD_SIZE : usize = 65536;
    let mut file = match File::create(&filename) {
        Ok(f) => f,
        Err(_) => { return Err(status::Custom(Status::InternalServerError, "failed uploading")); }
    };

    loop  {
        let buffer : &[u8] = paste.peek();
        read_size += buffer.len();
        if read_size >= MAX_UPLOAD_SIZE {
            fs::remove_file(filename);
            return Err(status::Custom(Status::PartialContent, "upload size exceeded"));
        }
        file.write(buffer);
        if paste.peek_complete() {
            break;
        }
    }

    file.flush();
    Ok(content::Plain(url))
}

#[get("/<id>")]
fn retrieve(id: &RawStr) -> Option<content::Plain<String>> {
    let filename = format!("upload/{id}", id = id);
    //File::open(&filename).ok()
    let file = match File::open(&filename) {
        Ok(f) => f,
        Err(_) => return None
    };
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    match buf_reader.read_to_string(&mut contents) {
        Ok(c) => c,
        Err(_) => return None
    };
    Some(content::Plain(contents))
}

#[delete("/<id>")]
fn delete(id: &RawStr) -> &str {
    let filename = format!("upload/{id}", id = id);
    match std::fs::remove_file(&filename) {
        Ok(_) => "",
        Err(_) => "delete failed"
    }
}