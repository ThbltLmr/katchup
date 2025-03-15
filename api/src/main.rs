use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

use api::ThreadPool;

fn handle_stream(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let http_request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::build(4).expect("Could not build thread pool");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_stream(stream);
        });
    }
}
