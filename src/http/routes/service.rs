use crate::http::model::response::HttpResponse;

pub fn status() -> HttpResponse {
    HttpResponse {
        status: 200,
        header: Default::default(),
        body: "Hello World!".to_string(),
    }
}