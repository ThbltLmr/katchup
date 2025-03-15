use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

use api::ThreadPool;

const PORT_NUMBER: &str = "8000";
const IP: &str = "127.0.0.1";

fn handle_stream(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let http_request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request received: {http_request:#?}");
    let request_line = reader.lines().next().unwrap().unwrap();

    let (Some(method), Some(url), Some(version), None) = request_line.split_whitespace() else {
        println!("Unexpected request format");
        return;
    }

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", IP, PORT_NUMBER)).unwrap();
    let pool = ThreadPool::build(4).expect("Could not build thread pool");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_stream(stream);
        });
    }
}
