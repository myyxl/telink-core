pub enum HttpStatus {
    Ok = 200,
    NotFound = 404
}

impl ToString for HttpStatus {
    fn to_string(&self) -> String {
        match &self {
            HttpStatus::Ok => String::from("OK"),
            HttpStatus::NotFound => String::from("Not Found")
        }
    }
}