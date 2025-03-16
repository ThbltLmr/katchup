mod router;
mod thread_pool;

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
}

impl std::fmt::Display for HandleRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidRequestLineError => write!(f, "Invalid Request Line Error"),
            Self::MethodNotAllowedError => write!(f, "Method Not Allowed Error"),
        }
    }
}

fn handle_stream(mut stream: TcpStream) -> Result<(), HandleRequestError> {
    let reader = BufReader::new(&stream);
    let http_request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request received: {http_request:#?}");

    let request_line = http_request[0].to_string();
    let (method, url, _version) = parse_request_line(request_line)?;

    if method != "GET" {
        return Err(HandleRequestError::MethodNotAllowedError);
    }

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}

fn parse_request_line(
    request_line: String,
) -> Result<(String, String, String), HandleRequestError> {
    let mut split = request_line.split_whitespace();
    let (Some(method), Some(url), Some(version), None) =
        (split.next(), split.next(), split.next(), split.next())
    else {
        println!("Unexpected request format");
        return Err(HandleRequestError::InvalidRequestLineError);
    };

    Ok((method.to_string(), url.to_string(), version.to_string()))
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
