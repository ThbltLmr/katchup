mod server {
    pub mod request_parser;
    pub mod response_builder;
    pub mod router;
    pub mod thread_pool;
}

mod adapters {
    pub mod gemini_adapter;
    pub mod ollama_adapter;
    pub mod tmdb_adapter;
}

use std::{
    error::Error,
    fmt,
    fmt::{Display, Formatter},
    io,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use server::{
    request_parser::RequestParser,
    response_builder::HttpResponseBuilder,
    router::{Router, RouterResponse},
    thread_pool::ThreadPool,
};

/* Exposing 0.0.0.0:8000 */
const PORT_NUMBER: &str = "8000";
const LOCALHOST: &str = "0.0.0.0";

/* TODO: Improve error naming */
#[derive(Debug)]
enum HandleRequestError {
    InvalidRequestLineError,
    MethodNotAllowedError,
    UnknownRouteError,
    IoError(io::Error),
    NetworkError,
}

impl Display for HandleRequestError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidRequestLineError => write!(f, "Invalid request line"),
            Self::MethodNotAllowedError => write!(f, "Method not allowed"),
            Self::UnknownRouteError => write!(f, "Unknown route"),
            Self::IoError(error) => write!(f, "IO error: {error:?}"),
            Self::NetworkError => write!(f, "Could not access external service"),
        }
    }
}

impl Error for HandleRequestError {}

fn handle_stream(mut stream: TcpStream) -> Result<(), HandleRequestError> {
    let response_builder = HttpResponseBuilder::new();
    let router = Router::new();

    let Ok(http_request) = read_until_empty_line(&mut stream) else {
        let response = response_builder.build_or_default_to_500(400, RouterResponse::None);
        stream.write_all(response.format_string.as_bytes()).unwrap();

        return Err(HandleRequestError::IoError(
            read_until_empty_line(&mut stream).unwrap_err(),
        ));
    };

    println!("Request received: {http_request:#?}");

    let Ok(request) = RequestParser::parse_request(http_request) else {
        let response = response_builder.build_or_default_to_500(400, RouterResponse::None);
        stream.write_all(response.format_string.as_bytes()).unwrap();

        println!("Could not parse request");
        return Err(HandleRequestError::InvalidRequestLineError);
    };

    println!("Request parse: {request:#?}");

    if request.method != "GET" {
        let response = response_builder.build_or_default_to_500(429, RouterResponse::None);
        stream.write_all(response.format_string.as_bytes()).unwrap();

        println!("Invalid method");
        return Err(HandleRequestError::MethodNotAllowedError);
    }

    let Some(route) = router.get_route(&request.uri) else {
        let response = response_builder.build_or_default_to_500(404, RouterResponse::None);
        stream.write_all(response.format_string.as_bytes()).unwrap();

        println!("Unknown route");
        return Err(HandleRequestError::UnknownRouteError);
    };

    println!("Route to call: {}", request.uri.path);

    let Ok(body) = router.respond(&route) else {
        let response = response_builder.build_or_default_to_500(503, RouterResponse::None);
        stream.write_all(response.format_string.as_bytes()).unwrap();

        println!("Could not access external api");
        return Err(HandleRequestError::NetworkError);
    };

    let response = response_builder.build_or_default_to_500(200, body);

    stream.write_all(response.format_string.as_bytes()).unwrap();
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
    /* TODO: implement API key authentication */
    let listener = TcpListener::bind(format!("{}:{}", LOCALHOST, PORT_NUMBER)).unwrap();
    let pool = ThreadPool::build(4).expect("Could not build thread pool");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            let _ = handle_stream(stream);
        });
    }
}
