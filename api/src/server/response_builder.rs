use std::collections::HashMap;

use super::router::RouterResponse;

pub struct HttpResponse {
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
        map.insert(429, "Method not allowed".to_string());
        map.insert(500, "Internal Server Error".to_string());
        map.insert(503, "Service Unavailable".to_string());
        map
    }

    fn format_response(&self, code: (u16, &str), body: RouterResponse) -> String {
        match body {
            RouterResponse::None => {
                format!(
                    "HTTP/1.1 {} {}\r\nAccess-Control-Allow-Headers: *\r\nAccess-Control-Allow-Methods: *\r\norigin: *\r\nAccess-Control-Allow-Origin: http://localhost:5173\r\n\r\n",
                    code.0,
                    code.1,
                )
            }
            _ => match serde_json::to_string(&body) {
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
            },
        }
    }

    fn build_500(&self) -> HttpResponse {
        let code: u16 = 500;
        let message: String = self.http_code_map.get(&code).unwrap().to_string();

        HttpResponse {
            format_string: self.format_response((code, &message), RouterResponse::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adapters::tmdb_adapter::{ShowDetails, ShowDetailsSeason};

    use super::*;

    #[test]
    fn test_build_500() {
        let response_builder = HttpResponseBuilder::new();

        let response = response_builder.build_500();

        assert_eq!(response.format_string, "HTTP/1.1 500 Internal Server Error\r\nAccess-Control-Allow-Headers: *\r\nAccess-Control-Allow-Methods: *\r\norigin: *\r\nAccess-Control-Allow-Origin: http://localhost:5173\r\n\r\n");
    }

    #[test]
    fn test_format_response_success() {
        let response_builder = HttpResponseBuilder::new();
        let code = (200, "OK");
        let body = RouterResponse::ShowDetails(ShowDetails {
            number_of_episodes: 2,
            number_of_seasons: 1,
            seasons: vec![ShowDetailsSeason {
                id: 2,
                name: "test".to_string(),
                episode_count: 3,
            }],
        });

        let formatted_response = response_builder.format_response(code, body);
        assert!(formatted_response.contains("HTTP/1.1 200 OK"));
        assert!(formatted_response.contains("content-type: application/json"));
        assert!(formatted_response.contains("content-length"));
        assert!(formatted_response.contains("OK"));
    }
}
