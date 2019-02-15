#![feature(proc_macro_hygiene, decl_macro)]
#![feature(type_alias_enum_variants)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket;

extern crate askama;
extern crate askama_escape;

mod highlight;
mod io;
mod params;

use highlight::highlight;
use io::{generate_id, get_paste, store_paste};
use params::{HostHeader, IsPlaintextRequest};

use askama::Template;
use askama_escape::{Html, MarkupDisplay};

use rocket::http::{ContentType, Status};
use rocket::request::Form;
use rocket::response::content::Content;
use rocket::response::Redirect;
use rocket::Data;

use std::io::Read;

///
/// Homepage
///

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

#[get("/")]
fn index() -> Index {
    Index {}
}

///
/// Submit Paste
///

#[derive(FromForm)]
struct IndexForm {
    val: String,
}

#[post("/", data = "<input>")]
fn submit(input: Form<IndexForm>) -> Redirect {
    let id = generate_id();
    store_paste(id.clone(), input.into_inner().val);
    Redirect::to(uri!(render: id))
}

#[put("/", data = "<input>")]
fn submit_raw(input: Data, host: HostHeader) -> std::io::Result<String> {
    let mut data = String::new();
    input.open().take(1024 * 1000).read_to_string(&mut data)?;

    let id = generate_id();
    store_paste(id.clone(), data);

    match *host {
        Some(host) => Ok(format!("https://{}/{}", host, id)),
        None => Ok(id),
    }
}

///
/// Show paste page
///

#[derive(Template)]
#[template(path = "paste.html")]
struct Render {
    content: MarkupDisplay<Html, String>,
}

#[get("/<key>")]
fn render(key: String, plaintext: IsPlaintextRequest) -> Result<Content<String>, Status> {
    let mut splitter = key.splitn(2, '.');
    let key = splitter.next().ok_or_else(|| Status::NotFound)?;
    let ext = splitter.next();

    // get() returns a read-only lock, we're not going to be writing to this key
    // again so we can hold this for as long as we want
    let entry = get_paste(key).ok_or_else(|| Status::NotFound)?;

    if *plaintext {
        Ok(Content(ContentType::Plain, entry))
    } else {
        let template = Render {
            content: match ext {
                None => MarkupDisplay::new_unsafe(entry, Html),
                Some(extension) => highlight(&entry, extension)
                    .map(|h| MarkupDisplay::new_safe(h, Html))
                    .ok_or_else(|| Status::NotFound)?,
            },
        };

        template
            .render()
            .map(|html| Content(ContentType::HTML, html))
            .map_err(|_| Status::InternalServerError)
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, submit, submit_raw, render])
        .launch();
}
