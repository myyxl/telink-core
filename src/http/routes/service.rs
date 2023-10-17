use std::sync::{Arc, Mutex};
use crate::http::model::response::HttpResponse;
use crate::State;

pub fn status(state: Arc<Mutex<State>>) -> Option<HttpResponse> {
    Some(
        HttpResponse::new()
            .body(String::from("{\"core\": true, \"controller\": false}"))
    )
}