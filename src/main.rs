#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;
extern crate crossbeam;

mod paste_id;
use paste_id::PasteID;

mod cleanup_worker;
use cleanup_worker::CleanupWorker;

#[cfg(test)] mod tests;

use rocket::Data;

use std::fs::{self, File};
use std::path::Path;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use std::time::Duration;

use rocket::Rocket;
use rocket::response::status;
use rocket::response::content;
use rocket::http::Status;

const UPLOAD_SUB_DIR : &'static str = "upload";

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![index, upload, retrieve, delete, update])
}
fn main() {
    let worker = CleanupWorker::new(cleanup_worker::DEFAULT_CLEANUP_INTERVAL_SECONDS, 
                                    UPLOAD_SUB_DIR.to_string(), 
                                    Duration::from_secs(cleanup_worker::DEFAULT_RETENTION_SECONDS));
    worker.start();
    rocket().launch();

    worker.stop();
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with the <id> of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`

      PUT /<id>

        updates the content of the paste with id `<id>``

      DELETE /<id>

        delete the paste with the id `<id>`
    "
}

fn upsert(id: PasteID, paste: Data) -> Result< content::Plain<String> , status::Custom<&'static str> > {
    
    let filename = format!("{}/{id}", UPLOAD_SUB_DIR, id = id);

    // Write the paste out to the file and return the URL.
    const MAX_UPLOAD_SIZE : usize = 65536;
    let mut file = match File::create(&filename) {
        Ok(f) => f,
        Err(_) => { return Err(status::Custom(Status::InternalServerError, "failed uploading")); }
    };

    let write_size = match paste.stream_to(&mut file) {
        Ok(size) => size as usize,
        Err(_) => { return Err(status::Custom(Status::InternalServerError, "failed uploading")); }
    };
    if write_size >= MAX_UPLOAD_SIZE {
        fs::remove_file(filename).unwrap();
        return Err(status::Custom(Status::PartialContent, "upload size exceeded"));
    }   

    match file.flush() {
        Ok(v) => v,
        Err(_) => { return Err(status::Custom(Status::InternalServerError, "failed uploading")); }
    };

    Ok(content::Plain(id.as_string()))
}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> Result< content::Plain<String> , status::Custom<&'static str> > {
    
    let id = PasteID::new(32);
    upsert(id, paste)
}

#[put("/<id>", data = "<paste>")]
fn update(id: PasteID, paste: Data) -> Result< content::Plain<String> , status::Custom<&'static str> > {
    
    let filename = format!("{}/{id}", UPLOAD_SUB_DIR, id = id);
    if Path::new(&filename).is_file() {
        return upsert(id, paste);
    }
    Err(status::Custom(Status::NotFound, "id no match"))
}

#[get("/<id>")]
fn retrieve(id: PasteID) -> Option<content::Plain<String>> {
    let filename = format!("{}/{id}", UPLOAD_SUB_DIR, id = id);
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
fn delete(id: PasteID) -> &str {
    let filename = format!("{}/{id}", UPLOAD_SUB_DIR, id = id);
    match std::fs::remove_file(&filename) {
        Ok(_) => "",
        Err(_) => "delete failed"
    }
}