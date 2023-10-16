use std::collections::HashMap;
use crate::http::model::status::HttpStatus;

pub struct HttpResponse {
    pub status: HttpStatus,
    pub body: String,
    pub header: HashMap<String, String>
}