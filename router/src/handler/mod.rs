use http::{httprequest::HttpRequest, httpresponse::HttpResponse};

pub mod api;
pub mod staticres;

pub trait HttpReqHandler {
    fn handle(&self, request: &HttpRequest) -> HttpResponse;
}
