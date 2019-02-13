#![feature(proc_macro_hygiene, decl_macro)]
#![feature(type_alias_enum_variants)]

#[macro_use] extern crate lazy_static;

#[macro_use] extern crate rocket;

extern crate askama;
extern crate askama_escape;

mod io;
mod highlight;
mod params;

use io::{store_paste, get_paste};
use highlight::highlight;
use params::{IsPlaintextRequest, HostHeader};

use askama::Template;
use askama_escape::{MarkupDisplay, Html};

use rocket::Data;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::response::content::Content;
use rocket::http::ContentType;

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
    val: String
}

#[post("/", data = "<input>")]
fn submit(input: Form<IndexForm>) -> Redirect {
    let id = store_paste(input.into_inner().val);
    Redirect::to(uri!(render: id))
}

#[put("/", data = "<input>")]
fn submit_raw(input: Data, host: HostHeader) -> std::io::Result<String> {
    let mut data = String::new();
    input.open().take(1024 * 1000).read_to_string(&mut data)?;

    let paste = store_paste(data);

    match *host {
        Some(host) => Ok(format!("https://{}/{}", host, paste)),
        None => Ok(paste)
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
fn render(key: String, plaintext: IsPlaintextRequest) -> Option<Content<String>> {
    let mut splitter = key.splitn(2, '.');
    let key = splitter.next()?;
    let ext = splitter.next();

    // get() returns a read-only lock, we're not going to be writing to this key
    // again so we can hold this for as long as we want
    let entry = get_paste(key)?;

    if *plaintext {
        Some(Content(ContentType::Plain, entry))
    } else {
        Some(Content(ContentType::HTML, Render {
            content: match ext {
                None => MarkupDisplay::new_unsafe(entry, Html),
                Some(extension) => MarkupDisplay::new_safe(highlight(&entry, extension)?, Html)
            },
        }.render().unwrap()))
    }
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index, submit, submit_raw, render])
        .launch();
}
