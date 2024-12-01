use std::{
    io, collections::HashMap, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, time::Duration
};
#[allow(unused_imports)]

mod routes;
use routes::{Route, Routes};






struct Server {
    listener: TcpListener,
    routes: HashMap<String, Route>
}

impl Server {
    fn new(addr: &str) -> Server {
        let listener = TcpListener::bind(addr).unwrap();

        let mut server = Server { listener, routes: HashMap::new() };
        for route in Routes::build() {

            server.routes.insert(route.name.clone(), route );
        }
        server
    }

    pub fn listen(&self) {
        for mut stream in self.listener.incoming().flatten() {
            stream.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
            if let Ok(req) = Request::new(&mut stream) {
                let mut resource = req.resource[1..].split('/');
                match self.routes.get(&resource.next().unwrap().to_string()){
                    Some(route) => { stream.write_all(&(route.visit)(req)).unwrap(); },
                    None => { stream.write_all(("HTTP/1.1 404 Not Found\r\n".to_string() + "\r\n").as_bytes()).unwrap(); },
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Request {

    pub resource: String,
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}


// ["GET /asdf HTTP/1.1", "Host: 127.0.0.1:4221", "User-Agent: curl/8.4.0", "Accept: */*"]
impl Request {
    fn new(stream: &mut TcpStream) -> io::Result<Request> {

        let mut http_req_parts = BufReader::new(stream)
            .lines()
            .map(|result| result.unwrap());

        let header_parts = http_req_parts.next().unwrap();
        let mut header_parts = header_parts.split_ascii_whitespace();
        let method = match header_parts.next().unwrap(){
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            "DELETE" => HttpMethod::Delete,
            "PUT" => HttpMethod::Put,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "unsupported HTTP method")),
        };

        let resource = header_parts.next().unwrap().to_string();
        // let _version = ...

        let mut headers = HashMap::new();
        loop {
            let line = http_req_parts.next().unwrap();
            if line.is_empty() {
                break;
            }

            let mut parts = line.split(": ");
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().to_string();

            headers.insert(key, value);
        }
        Ok(Request {resource, method, headers, body: vec![]})
    }
}

struct Response {
}

#[derive(Debug)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

fn main() {
    
    let server = Server::new("127.0.0.1:4221");
    server.listen();
}

