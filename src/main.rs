#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket;

extern crate askama;

mod highlight;
mod io;
mod params;

use highlight::highlight;
use io::{generate_id, get_paste, store_paste};
use params::{HostHeader, IsPlaintextRequest};

use askama::{Html as AskamaHtml, MarkupDisplay, Template};

use rocket::http::{ContentType, RawStr, Status};
use rocket::request::Form;
use rocket::response::content::{Content, Html};
use rocket::response::Redirect;
use rocket::Data;

use std::borrow::Cow;

use tokio::io::AsyncReadExt;

///
/// Homepage
///

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[get("/")]
fn index() -> Result<Html<String>, Status> {
    Index
        .render()
        .map(Html)
        .map_err(|_| Status::InternalServerError)
}

///
/// Submit Paste
///

#[derive(FromForm)]
struct IndexForm {
    val: String,
}

#[post("/", data = "<input>")]
async fn submit(input: Form<IndexForm>) -> Redirect {
    let id = generate_id();
    let uri = uri!(show_paste: &id);
    store_paste(id, input.into_inner().val).await;
    Redirect::to(uri)
}

#[put("/", data = "<input>")]
async fn submit_raw(input: Data, host: HostHeader<'_>) -> Result<String, Status> {
    let mut data = String::new();
    input.open().take(1024 * 1000)
        .read_to_string(&mut data).await
        .map_err(|_| Status::InternalServerError)?;

    let id = generate_id();
    let uri = uri!(show_paste: &id);

    store_paste(id, data).await;

    match *host {
        Some(host) => Ok(format!("https://{}{}", host, uri)),
        None => Ok(format!("{}", uri)),
    }
}

///
/// Show paste page
///

#[derive(Template)]
#[template(path = "paste.html")]
struct ShowPaste<'a> {
    content: MarkupDisplay<AskamaHtml, Cow<'a, String>>,
}

#[get("/<key>")]
async fn show_paste(key: String, plaintext: IsPlaintextRequest) -> Result<Content<String>, Status> {
    let mut splitter = key.splitn(2, '.');
    let key = splitter.next().ok_or_else(|| Status::NotFound)?;
    let ext = splitter.next();

    let entry = &*get_paste(key).await.ok_or_else(|| Status::NotFound)?;

    if *plaintext {
        Ok(Content(ContentType::Plain, entry.to_string()))
    } else {
        let code_highlighted = match ext {
            Some(extension) => match highlight(&entry, extension) {
                Some(html) => html,
                None => return Err(Status::NotFound),
            },
            None => String::from(RawStr::from_str(entry).html_escape()),
        };

        // Add <code> tags to enable line numbering with CSS 
        let html = format!(
            "<code>{}</code>",
            code_highlighted.replace("\n", "\n</code><code>")
        );

        let content = MarkupDisplay::new_safe(Cow::Borrowed(&html), AskamaHtml);

        let template = ShowPaste { content };
        match template.render() {
            Ok(html) => Ok(Content(ContentType::HTML, html)),
            Err(_) => Err(Status::InternalServerError),
        }
    }
}

#[tokio::main]
async fn main() {
    let result = rocket::ignite()
        .mount("/", routes![index, submit, submit_raw, show_paste])
        .launch()
        .await;

    if let Err(e) = result {
        eprintln!("Failed to launch Rocket: {:#?}", e);
    }
}
