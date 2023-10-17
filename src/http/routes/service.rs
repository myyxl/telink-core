use crate::http::model::response::HttpResponse;
use crate::http::model::status::HttpStatus;

pub fn status() -> Option<HttpResponse> {
    Some(
        HttpResponse {
            status: HttpStatus::Ok,
            header: Default::default(),
            body: String::from("{\"core\": true, \"controller\": false}"),
        }
    )
}