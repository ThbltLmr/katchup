mod request_parser;
mod router;
mod thread_pool;

use request_parser::RequestParser;
use router::get_route;
use std::io;
use thread_pool::ThreadPool;

use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

const PORT_NUMBER: &str = "8000";
const LOCALHOST: &str = "127.0.0.1";

#[derive(Debug)]
enum HandleRequestError {
    InvalidRequestLineError,
    MethodNotAllowedError,
    UnknownRouteError,
    IoError(io::Error),
}

impl std::fmt::Display for HandleRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidRequestLineError => write!(f, "Invalid request line"),
            Self::MethodNotAllowedError => write!(f, "Method not allowed"),
            Self::UnknownRouteError => write!(f, "Unknown route"),
            Self::IoError(error) => write!(f, "io error: {error:?}"),
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

    if request.method != "GET" {
        println!("Invalid method");
        return Err(HandleRequestError::MethodNotAllowedError);
    }

    let Some(route) = get_route(&request.uri) else {
        println!("Invalid method");
        return Err(HandleRequestError::UnknownRouteError);
    };

    println!("Route to call: {}", request.uri.path);

    let response = format!(
        "HTTP/1.1 200 OK\r\n\r\n{}",
        request.uri.query.unwrap_or("no query params".to_string())
    );

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
