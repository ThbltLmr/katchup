use std::collections::HashMap;

pub struct RequestParser;

#[derive(Debug)]
pub struct Uri {
    pub path: String,
    pub query: Option<String>,
}

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub uri: Uri,
    headers: HashMap<String, String>,
    body: Option<String>,
}

#[derive(Debug)]
pub enum ParseRequestError {
    InvalidRequestLineError,
}

impl RequestParser {
    pub fn parse_request(request_string: String) -> Result<Request, ParseRequestError> {
        let mut lines = request_string.lines();

        let request_line = lines
            .next()
            .ok_or(ParseRequestError::InvalidRequestLineError)?;

        let mut request_line_parts = request_line.split_whitespace();

        let method = request_line_parts
            .next()
            .ok_or(ParseRequestError::InvalidRequestLineError)?
            .to_string();

        let uri_string = request_line_parts
            .next()
            .ok_or(ParseRequestError::InvalidRequestLineError)?
            .to_string();

        let mut uri_parts = uri_string.splitn(2, '?');
        let path = uri_parts.next().unwrap().to_string();
        let query = uri_parts.next().map(|s| s.to_string());
        let uri = Uri { path, query };

        let mut headers = HashMap::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break; // Headers end with an empty line
            }
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        let body = if let Some(content) = lines.next() {
            Some(content.to_string())
        } else {
            None
        };

        Ok(Request {
            method,
            uri,
            headers,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_request_with_headers_and_body() {
        let request_string = "GET / HTTP/1.1\nHeader1: Value1\nHeader2: Value2\n\nThis is the body";
        let result = RequestParser::parse_request(request_string.to_string());
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.method, "GET");
        assert_eq!(request.uri.path, "/");
        assert_eq!(request.headers.get("Header1").unwrap(), "Value1");
        assert_eq!(request.headers.get("Header2").unwrap(), "Value2");
        assert_eq!(request.body.unwrap(), "This is the body");
    }

    #[test]
    fn test_parse_valid_request_with_only_request_line() {
        let request_string = "GET / HTTP/1.1";
        let result = RequestParser::parse_request(request_string.to_string());
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.method, "GET");
        assert_eq!(request.uri.path, "/");
        assert_eq!(request.headers.len(), 0);
        assert_eq!(request.body, None);
    }

    #[test]
    fn test_parse_valid_request_with_headers_no_body() {
        let request_string = "GET / HTTP/1.1\nHeader1: Value1\nHeader2: Value2\n";
        let result = RequestParser::parse_request(request_string.to_string());
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.method, "GET");
        assert_eq!(request.uri.path, "/");
        assert_eq!(request.headers.get("Header1").unwrap(), "Value1");
        assert_eq!(request.headers.get("Header2").unwrap(), "Value2");
        assert_eq!(request.body, None);
    }

    #[test]
    fn test_parse_valid_request_with_query_parameter() {
        let request_string = "GET /search?param1=value1&param2=value2 HTTP/1.1";
        let result = RequestParser::parse_request(request_string.to_string());
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.method, "GET");
        assert_eq!(request.uri.path, "/search");
        assert_eq!(request.uri.query.unwrap(), "param1=value1&param2=value2");
    }
}
