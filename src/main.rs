mod highlight;
mod io;
mod params;
mod errors;

use highlight::highlight;
use io::{generate_id, get_paste, store_paste, PasteStore};
use params::{HostHeader, IsPlaintextRequest};
use errors::{NotFound, InternalServerError};

use askama::{Html as AskamaHtml, MarkupDisplay, Template};
use actix_web::{web, http::header, web::Data, App, HttpResponse, HttpServer, Responder, Error, HttpRequest};

use std::borrow::Cow;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use log::error;
use actix_web::web::{PayloadConfig, FormConfig};

#[derive(argh::FromArgs, Clone)]
/// a pastebin.
pub struct BinArgs {
    /// socket address to bind to (default: 127.0.0.1:8080)
    #[argh(
        positional,
        default = "SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)"
    )]
    bind_addr: SocketAddr,
    /// maximum amount of pastes to store before rotating (default: 1000)
    #[argh(option, default = "1000")]
    buffer_size: usize,
    /// maximum paste size in bytes (default. 32kB)
    #[argh(option, default = "32 * 1024")]
    max_paste_size: usize,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "INFO");
    }
    pretty_env_logger::init();

    let args: BinArgs = argh::from_env();

    let store = Data::new(PasteStore::default());

    let server = HttpServer::new({
        let args = args.clone();

        move || {
            App::new()
                .app_data(store.clone())
                .app_data(PayloadConfig::default().limit(args.max_paste_size))
                .app_data(FormConfig::default().limit(args.max_paste_size))
                .wrap(actix_web::middleware::Compress::default())
                .route("/", web::get().to(index))
                .route("/", web::post().to(submit))
                .route("/", web::put().to(submit_raw))
                .route("/", web::head().to(|| HttpResponse::MethodNotAllowed()))
                .route("/{paste}", web::get().to(show_paste))
                .route("/{paste}", web::head().to(|| HttpResponse::MethodNotAllowed()))
                .default_service(web::to(|req: HttpRequest| -> HttpResponse {
                    error!("Couldn't find resource {}", req.uri());
                    HttpResponse::from_error(NotFound.into())
                }))
        }
    });

    server.bind(args.bind_addr)?
        .run()
        .await
}

///
/// Homepage
///

#[derive(Template)]
#[template(path = "index.html")]
struct Index;
async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    render_template(&req, Index)
}

///
/// Submit Paste
///

#[derive(serde::Deserialize)]
struct IndexForm {
    val: String,
}

async fn submit(input: web::Form<IndexForm>, store: Data<PasteStore>) -> impl Responder {
    let id = generate_id();
    let uri = format!("/{}", &id);
    store_paste(&store, id, input.into_inner().val).await;
    HttpResponse::Found().header(header::LOCATION, uri).finish()
}

async fn submit_raw(data: String, host: HostHeader, store: Data<PasteStore>) -> Result<String, Error> {
    let id = generate_id();
    let uri = format!("/{}", &id);

    store_paste(&store, id, data).await;

    match &*host {
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

async fn show_paste(req: HttpRequest, key: actix_web::web::Path<String>, plaintext: IsPlaintextRequest, store: Data<PasteStore>) -> Result<HttpResponse, Error> {
    let mut splitter = key.splitn(2, '.');
    let key = splitter.next().unwrap();
    let ext = splitter.next();

    let entry = &*get_paste(&store, key).await.ok_or_else(|| NotFound)?;

    if *plaintext {
        Ok(HttpResponse::Ok().content_type("text/plain; charset=utf-8").body(entry))
    } else {
        let code_highlighted = match ext {
            Some(extension) => match highlight(&entry, extension) {
                Some(html) => html,
                None => return Err(NotFound.into()),
            },
            None => htmlescape::encode_minimal(entry),
        };

        // Add <code> tags to enable line numbering with CSS 
        let html = format!(
            "<code>{}</code>",
            code_highlighted.replace("\n", "</code><code>")
        );

        let content = MarkupDisplay::new_safe(Cow::Borrowed(&html), AskamaHtml);

        render_template(&req, ShowPaste { content })
    }
}

///
/// Helpers
///

fn render_template<T: Template>(req: &HttpRequest, template: T) -> Result<HttpResponse, Error> {
    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok().body(html)),
        Err(e) => {
            error!("Error while rendering template for {}: {}", req.uri(), e);
            Err(InternalServerError(Box::new(e)).into())
        }
    }
}
