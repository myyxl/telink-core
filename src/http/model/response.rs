use std::collections::HashMap;

pub struct HttpResponse {
    pub status: u32,
    pub body: String,
    pub header: HashMap<String, String>
}