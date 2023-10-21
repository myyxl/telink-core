pub fn get_request_methods() -> Vec<String> {
    vec![
        String::from("GET"),
        String::from("POST"),
        String::from("HEAD"),
        String::from("PUT"),
        String::from("DELETE"),
        String::from("CONNECT"),
        String::from("OPTIONS"),
        String::from("TRACE"),
        String::from("PATCH")
    ]
}