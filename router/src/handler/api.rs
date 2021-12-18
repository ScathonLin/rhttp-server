use http::{httprequest::HttpRequest, httpresponse::HttpResponse};

use super::HttpReqHandler;

#[derive(Default)]
pub struct ApiHandler {}

impl HttpReqHandler for ApiHandler {
    fn handle(&self, _request: &HttpRequest) -> HttpResponse {
        todo!("to implement")
    }
}
