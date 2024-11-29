use std::io::Write;
#[allow(unused_imports)]
use std::net::TcpListener;

const res_status_line: &str = "HTTP/1.1 200 OK\r\n\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                _stream.write(res_status_line.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
