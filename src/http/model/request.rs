use std::collections::HashMap;

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub header: HashMap<String, String>
}