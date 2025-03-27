mod server {
    pub mod request_parser;
    pub mod router;
    pub mod thread_pool;
}

mod adapters {
    pub mod ollama_adapter;
    pub mod tmdb_adapter;
}

use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

use server::thread_pool::ThreadPool;
use server::{request_parser::RequestParser, router::Router};
use std::io;

const PORT_NUMBER: &str = "8000";
const LOCALHOST: &str = "127.0.0.1";

#[derive(Debug)]
enum HandleRequestError {
    InvalidRequestLineError,
    MethodNotAllowedError,
    UnknownRouteError,
    IoError(io::Error),
    NetworkError,
}

impl std::fmt::Display for HandleRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidRequestLineError => write!(f, "Invalid request line"),
            Self::MethodNotAllowedError => write!(f, "Method not allowed"),
            Self::UnknownRouteError => write!(f, "Unknown route"),
            Self::IoError(error) => write!(f, "io error: {error:?}"),
            Self::NetworkError => write!(f, "Could not access external service"),
        }
    }
}

fn handle_stream(mut stream: TcpStream) -> Result<(), HandleRequestError> {
    let Ok(http_request) = read_until_empty_line(&mut stream) else {
        return Err(HandleRequestError::IoError(
            read_until_empty_line(&mut stream).unwrap_err(),
        ));
    };

    println!("Request received: {http_request:#?}");

    let Ok(request) = RequestParser::parse_request(http_request) else {
        println!("Could not parse request");
        return Err(HandleRequestError::InvalidRequestLineError);
    };

    println!("Request parse: {request:#?}");

    if request.method != "GET" {
        println!("Invalid method");
        return Err(HandleRequestError::MethodNotAllowedError);
    }

    let router = Router::new();
    let Some(route) = router.get_route(&request.uri) else {
        println!("Unknown route");
        return Err(HandleRequestError::UnknownRouteError);
    };

    println!("Route to call: {}", request.uri.path);

    let Ok(body) = router.respond(&route) else {
        println!("Could not access external api");
        return Err(HandleRequestError::NetworkError);
    };

    let body_string = serde_json::to_string(&body).unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n{}",
        body_string.len(),
        body_string
    );

    println!("{response}");

    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}

pub fn read_until_empty_line(stream: &mut TcpStream) -> io::Result<String> {
    let reader = BufReader::new(stream);
    let mut result = String::new();

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        result.push_str(&line);
        result.push('\n');
    }

    Ok(result)
}

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", LOCALHOST, PORT_NUMBER)).unwrap();
    let pool = ThreadPool::build(4).expect("Could not build thread pool");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            let _ = handle_stream(stream);
        });
    }
}
