use crate::http::model::response::HttpResponse;

pub fn status() -> Option<HttpResponse> {
    Some(
        HttpResponse::new()
            .body(String::from("{\"core\": true, \"controller\": false}"))
    )
}