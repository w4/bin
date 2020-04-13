use std::ops::Deref;

use actix_web::{FromRequest, HttpRequest, HttpMessage};
use actix_web::dev::Payload;
use actix_web::http::header;

use futures::future::ok;

/// Holds a value that determines whether or not this request wanted a plaintext response.
///
/// We assume anything with the text/plain Accept or Content-Type headers want plaintext,
/// and also anything calling us from the console or that we can't identify.
pub struct IsPlaintextRequest(pub bool);

impl Deref for IsPlaintextRequest {
    type Target = bool;

    fn deref(&self) -> &bool {
        &self.0
    }
}

impl FromRequest for IsPlaintextRequest {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if req.content_type() == "text/plain" {
            return ok(IsPlaintextRequest(true));
        }

        match req
            .headers()
            .get(header::USER_AGENT)
            .and_then(|u| u.to_str().unwrap().splitn(2, '/').next())
        {
            None | Some("Wget") | Some("curl") | Some("HTTPie") => {
                ok(IsPlaintextRequest(true))
            }
            _ => ok(IsPlaintextRequest(false)),
        }
    }
}

/// Gets the Host header from the request.
///
/// The inner value of this `HostHeader` will be `None` if there was no Host header
/// on the request.
pub struct HostHeader(pub Option<String>);

impl Deref for HostHeader {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for HostHeader {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.headers().get(header::HOST) {
            None => ok(Self(None)),
            Some(h) => ok(Self(h.to_str().ok().map(|f| f.to_string())))
        }
    }
}
