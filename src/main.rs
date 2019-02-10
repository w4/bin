#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate lazy_static;

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

extern crate gpw;
extern crate syntect;
extern crate chashmap;

use rocket_contrib::templates::Template;
use rocket::response::Redirect;
use rocket::request::Form;

use serde::Serialize;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::easy::HighlightLines;
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};

use chashmap::CHashMap;
use std::borrow::Cow;
use std::cell::RefCell;

lazy_static! {
    static ref ENTRIES: CHashMap<String, String> = CHashMap::new();
}


#[derive(FromForm)]
struct IndexForm {
    val: String
}

#[get("/")]
fn index() -> Template {
    #[derive(Serialize)]
    struct Index {}

    Template::render("index", Index {})
}


/// Generates a randomly generated id, stores the given paste under that id and then returns the id.
fn store_paste(content: String) -> String {
    thread_local!(static KEYGEN: RefCell<gpw::PasswordGenerator> = RefCell::new(gpw::PasswordGenerator::default()));

    let id = KEYGEN.with(|k| k.borrow_mut().next().unwrap());
    ENTRIES.insert(id.clone(), content);
    id
}

#[post("/", data = "<input>")]
fn submit(input: Form<IndexForm>) -> Redirect {
    let id = store_paste(input.into_inner().val);
    Redirect::to(format!("/{}", id))
}

#[put("/", data = "<input>")]
fn submit_raw(input: String) -> String {
    format!("https://{}/{}", "localhost:8000", store_paste(input))
}


/// Takes the content of a paste and the extension passed in by the viewer and will return the content
/// highlighted in the appropriate format in HTML.
fn highlight(content: &str, ext: &str) -> Option<String> {
    lazy_static! {
        static ref SS: SyntaxSet = SyntaxSet::load_defaults_newlines();
        static ref TS: ThemeSet = ThemeSet::load_defaults();
    }

    let syntax = SS.find_syntax_by_extension(ext)?;
    let mut h = HighlightLines::new(syntax, &TS.themes["base16-ocean.dark"]);
    let regions = h.highlight(content, &SS);

    Some(styled_line_to_highlighted_html(&regions[..], IncludeBackground::No))
}

#[get("/<key>")]
fn render<'a>(key: String) -> Option<Template> {
    let mut splitter = key.splitn(2, ".");
    let key = splitter.next().unwrap();
    let ext = splitter.next();

    // get() returns a read-only lock, we're not going to be writing to this key
    // again so we can hold this for as long as we want
    let entry = ENTRIES.get(key)?;

    #[derive(Serialize)]
    struct Render<'a> {
        content: Cow<'a, String>
    }

    Some(Template::render("paste", Render {
        content: match ext {
            None => Cow::Borrowed(&*entry),
            Some(extension) => Cow::Owned(highlight(&*entry, extension)?)
        }
    }))
}


fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, submit, submit_raw, render])
        .launch();
}
