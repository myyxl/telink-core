use crate::http::model::response::HttpResponse;
use crate::http::model::status::HttpStatus;

pub fn status() -> HttpResponse {
    HttpResponse {
        status: HttpStatus::Ok,
        header: Default::default(),
        body: "Hello World!".to_string(),
    }
}