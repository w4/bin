#![feature(proc_macro_hygiene, decl_macro)]
#![feature(uniform_paths)]
#![feature(type_alias_enum_variants)]

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

use rocket::{Request, Data};
use rocket::request::{Form, FromRequest, Outcome};
use rocket::response::Redirect;
use rocket::response::content::Content;
use rocket::http::ContentType;

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
    Redirect::to(uri!(render: id))
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
    content: MarkupDisplay<Html, String>,
}

/// Holds a value that determines whether or not this request wanted a plaintext response.
///
/// We assume anything with the text/plain Accept or Content-Type headers want plaintext,
/// and also anything calling us from the console or that we can't identify.
struct IsPlaintextRequest(bool);
impl<'a, 'r> FromRequest<'a, 'r> for IsPlaintextRequest {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<IsPlaintextRequest, ()> {
        if let Some(format) = request.format() {
            if format.is_plain() {
                return Outcome::Success(IsPlaintextRequest(true));
            }
        }

        match request.headers().get_one("User-Agent").and_then(|u| u.splitn(2, '/').next()) {
            None | Some("Wget") | Some("curl") | Some("HTTPie") => Outcome::Success(IsPlaintextRequest(true)),
            _ => Outcome::Success(IsPlaintextRequest(false))
        }
    }
}

#[get("/<key>")]
fn render(key: String, plaintext: IsPlaintextRequest) -> Option<Content<String>> {
    let mut splitter = key.splitn(2, '.');
    let key = splitter.next()?;
    let ext = splitter.next();

    // get() returns a read-only lock, we're not going to be writing to this key
    // again so we can hold this for as long as we want
    let entry = get_paste(key)?;

    if plaintext.0 {
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
