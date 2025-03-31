use std::collections::HashMap;

use super::router::RouterResponse;

pub struct HttpResponse {
    code: (u16, String),
    body: RouterResponse,
    pub format_string: String,
}

pub struct HttpResponseBuilder {
    http_code_map: HashMap<u16, String>,
}

impl HttpResponseBuilder {
    pub fn new() -> Self {
        let http_code_map = HttpResponseBuilder::create_http_code();
        HttpResponseBuilder { http_code_map }
    }

    pub fn build_or_default_to_500(&self, code: u16, body: RouterResponse) -> HttpResponse {
        match self.http_code_map.get(&code) {
            Some(message) => HttpResponse {
                code: (code, message.clone()),
                body: body.clone(),
                format_string: self.format_response((code, &message), body),
            },

            None => self.build_500(),
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

    fn format_response(&self, code: (u16, &str), body: RouterResponse) -> String {
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
            Err(_) => "Failed to serialize response body".to_string(),
        }
    }

    fn build_500(&self) -> HttpResponse {
        let code: u16 = 500;
        let message: String = self.http_code_map.get(&code).unwrap().to_string();

        HttpResponse {
            code: (code, message.clone()),
            body: RouterResponse::None,
            format_string: self.format_response((code, &message), RouterResponse::None),
        }
    }
}
