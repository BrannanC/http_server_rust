use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
#[allow(unused_imports)]

const RES_STATUS_LINE: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_SATUS_LINE: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                handle_conn(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Request: {:#?}", &http_request[0][..]);
    let status_line = match &http_request[0][..] {
        "GET / HTTP/1.1" => RES_STATUS_LINE,
        _ => NOT_FOUND_SATUS_LINE,
    };

    stream.write_all(status_line.as_bytes()).unwrap();
}
