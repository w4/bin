use actix_web::{body::BoxBody, http::header, http::StatusCode, web, HttpResponse, ResponseError};

use std::fmt::{Formatter, Write};

macro_rules! impl_response_error_for_http_resp {
    ($ty:ty, $path:expr, $status:expr) => {
        impl ResponseError for $ty {
            fn error_response(&self) -> HttpResponse {
                HtmlResponseError::error_response(self)
            }
        }

        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", include_str!($path))
            }
        }

        impl HtmlResponseError for $ty {
            fn status_code(&self) -> StatusCode {
                $status
            }
        }
    };
}

#[derive(Debug)]
pub struct NotFound;

impl_response_error_for_http_resp!(NotFound, "../templates/404.html", StatusCode::NOT_FOUND);

#[derive(Debug)]
pub struct InternalServerError(pub Box<dyn std::error::Error>);

impl_response_error_for_http_resp!(
    InternalServerError,
    "../templates/500.html",
    StatusCode::INTERNAL_SERVER_ERROR
);

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
        resp.set_body(BoxBody::new(buf))
    }
}
