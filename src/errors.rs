use actix_web::{web, http::header, body::Body, HttpResponse, ResponseError, http::StatusCode};

use std::fmt::{Write, Formatter};

macro_rules! impl_response_error_for_http_resp {
    ($tt:tt) => {
        impl ResponseError for $tt {
            fn error_response(&self) -> HttpResponse {
                HtmlResponseError::error_response(self)
            }
        }
    }
}

#[derive(Debug)]
pub struct NotFound;
impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", include_str!("../templates/404.html"))
    }
}
impl HtmlResponseError for NotFound {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }
}
impl_response_error_for_http_resp!(NotFound);

#[derive(Debug)]
pub struct InternalServerError(pub Box<dyn std::error::Error>);
impl std::fmt::Display for InternalServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", include_str!("../templates/500.html"))
    }
}
impl HtmlResponseError for InternalServerError {}
impl_response_error_for_http_resp!(InternalServerError);

pub trait HtmlResponseError: ResponseError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        let mut resp = HttpResponse::new(HtmlResponseError::status_code(self));
        let mut buf = web::BytesMut::new();
        let _ = write!(&mut buf, "{}", self);
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("text/html; charset=utf-8"),
        );
        resp.set_body(Body::from(buf))
    }
}