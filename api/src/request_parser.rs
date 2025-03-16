use std::collections::HashMap;

struct RequestParser;

struct Uri {
    path: String,
    query: Option<String>,
}

struct Request {
    method: String,
    uri: Uri,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl RequestParser {
    fn parse_request
}
