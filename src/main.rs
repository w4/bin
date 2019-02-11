#![feature(proc_macro_hygiene, decl_macro)]
#![feature(uniform_paths)]

#[macro_use] extern crate lazy_static;

#[macro_use] extern crate rocket;

extern crate askama;
extern crate askama_escape;

mod io;
mod highlight;

use io::{store_paste, get_paste};
use highlight::highlight;

use askama::Template;
use askama_escape::{MarkupDisplay, Html};

use rocket::response::Redirect;
use rocket::request::Form;
use rocket::Data;

use std::io::Read;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

#[get("/")]
fn index() -> Index {
    Index {}
}


#[derive(FromForm)]
struct IndexForm {
    val: String
}
#[post("/", data = "<input>")]
fn submit(input: Form<IndexForm>) -> Redirect {
    let id = store_paste(input.into_inner().val);
    Redirect::to(format!("/{}", id))
}

#[put("/", data = "<input>")]
fn submit_raw(input: Data) -> std::io::Result<String> {
    let mut data = String::new();
    input.open().take(1024 * 1000).read_to_string(&mut data)?;
    Ok(format!("https://{}/{}", "localhost:8000", store_paste(data)))
}


#[derive(Template)]
#[template(path = "paste.html")]
struct Render {
    content: MarkupDisplay<Html, String>
}

#[get("/<key>")]
fn render(key: String) -> Option<Render> {
    let mut splitter = key.splitn(2, '.');
    let key = splitter.next().unwrap();
    let ext = splitter.next();

    // get() returns a read-only lock, we're not going to be writing to this key
    // again so we can hold this for as long as we want
    let entry = get_paste(key)?;

    Some(Render {
        content: match ext {
            None => MarkupDisplay::new_unsafe(entry, Html),
            Some(extension) => MarkupDisplay::new_safe(highlight(&entry, extension)?, Html)
        }
    })
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index, submit, submit_raw, render])
        .launch();
}
