use std::sync::Arc;
use std::{
    io, collections::HashMap, time::Duration
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
#[allow(unused_imports)]

mod routes;
use routes::{Route, Routes};

struct Server {
    listener: TcpListener,
    routes: Arc<HashMap<String, Route>>
}

impl Server {
    async fn new(addr: &str) -> Server {
        let listener = TcpListener::bind(addr).await.unwrap();
        let mut routes = HashMap::new();

        for route in Routes::build() {
            routes.insert(route.name.clone(), route );
        }
        Server { listener, routes: Arc::new(routes) }

    }

    pub async fn listen(&self) {
        loop {
            let (mut socket, _) = self.listener.accept().await.unwrap();
            {
                let routes = self.routes.clone();
                tokio::spawn(async move {
                    if let Ok(req) = Request::new(&mut socket).await {
                        let mut resource = req.resource[1..].split('/');
                        match routes.clone().get(&resource.next().unwrap().to_string()){
                            Some(route) => { socket.write_all(&(route.visit)(req)).await; },
                            None => { socket.write_all(("HTTP/1.1 404 Not Found\r\n".to_string() + "\r\n").as_bytes()).await; },
                        }
                    }
                });
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
    async fn new(stream: &mut TcpStream) -> io::Result<Request> {

        let mut http_req_parts = BufReader::new(stream)
            .lines();

        let header_parts = http_req_parts.next_line().await.unwrap().unwrap();
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
            let line = http_req_parts.next_line().await.unwrap().unwrap();
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

#[tokio::main]
async fn main() {
    
    let server = Server::new("127.0.0.1:4221").await;
    server.listen().await;
}

