use std::collections::HashMap;
use crate::http::model::status::HttpStatus;

pub struct HttpResponse {
    pub status: HttpStatus,
    pub body: String,
    pub header: HashMap<String, String>
}

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse {
            status: HttpStatus::Ok,
            body: String::new(),
            header: HashMap::new()
        }
    }

    pub fn status(mut self, status: HttpStatus) -> Self {
        self.status = status;
        self
    }

    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.header.insert(String::from(name), String::from(value));
        self
    }

    pub fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }


    pub fn build(self) -> Box<[u8]> {
        let status_string = self.status.to_string();
        let status_code = self.status as u32;
        let mut header_string = String::new();
        self.header
            .iter()
            .map(|entry| (entry.0.as_str(), entry.1.as_str()))
            .for_each(|entry| {
                header_string.push_str(format!("{}: {}\n", entry.0, entry.1).as_str())
            });
        Box::from(
            format!(
                "HTTP/1.1 {} {}\r\n{}\r\n{}",
                status_code,
                status_string,
                header_string,
                &self.body
            ).as_bytes()
        )
    }
}