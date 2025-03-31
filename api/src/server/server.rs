use serde_json::Value;
use std::collections::HashMap;

struct HttpResponse {
    pub code: (u16, String),
    pub body: Value,
    pub format_string: String,
}

struct HttpResponseBuilder {
    http_code_map: HashMap<u16, String>,
}

impl HttpResponseBuilder {
    pub fn new() -> Self {
        let http_code_map = HttpResponseBuilder::create_http_code();
        HttpResponseBuilder { http_code_map }
    }

    pub fn build_or_default_to_500(&self, code: u16, body: Value) -> HttpResponse {
        match self.http_code_map.get(&code) {
            Some(message) => HttpResponse {
                code: (code, message),
                body,
                format_string: S,
            },
            None => self.format_response((500, "Internal Server Error"), body),
        }
    }

    fn create_http_code() -> HashMap<u16, String> {
        let mut map = HashMap::new();
        map.insert(200, "OK".to_string());
        map.insert(400, "Bad Request".to_string());
        map.insert(404, "Not Found".to_string());
        map.insert(500, "Internal Server Error".to_string());
        map.insert(503, "Service Unavailable".to_string());
        map
    }

    fn format_response(&self, code: (u16, &str), body: Value) -> String {
        match serde_json::to_string(&body) {
            Ok(formatted_body) => {
                format!(
                    "HTTP/1.1 {} {}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nAccess-Control-Allow-Headers: *\r\nAccess-Control-Allow-Methods: *\r\norigin: *\r\nAccess-Control-Allow-Origin: http://localhost:5173\r\n\r\n{}",
                    code.0,
                    code.1,
                    formatted_body.len(),
                    formatted_body,
                )
            }
            Err(_) => "failed to serialize response body".to_string(),
        }
    }

    fn build_500(&self) -> HttpResponse {
        let code: u16 = 500;
        let message: String = self.http_code_map.get(&code).unwrap().to_string();
        let body = serde_json::Value::String("".to_string());

        HttpResponse {
            code: (code, message.clone()),
            body: body.clone(),
            format_string: self.format_response((code, &message), body),
        }
    }
}
