use std::{env, fs};

use crate::{Request, Response};


// TODO: make these an enums, avoid cating to string..bytes..vec
const RES_STATUS_LINE: &str = "HTTP/1.1 200 OK";
// const RES_CREATED: &str = "HTTP/1.1 201 Created\r\n\r\n";
// const RES_NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n\r\n";
// const CONTENT_TYPE_TEXT: &str = "Content-Type: text/plain\r\n";
// const CONTENT_TYPE_APP: &str = "Content-Type: application/octet-stream\r\n";

#[derive(Debug)]
pub struct Routes {}

impl Routes {
    pub fn build() -> Vec<Route> {
        vec![
            Routes::home(),
            Routes::echo(),
            Routes::user_agent(),
            Routes::files(),
        ]
    }

    fn home() -> Route {
        Route {
            name: "".to_string(),
            visit: |_| {
                let res = Response::new(RES_STATUS_LINE.to_string());
                return res.to_vec_u8();
            },
        }
    }

    fn echo() -> Route {
        Route {
            name: "echo".to_string(),
            visit: |req| {
                let mut resource = req.resource[1..].split('/');
                let stuff = match resource.nth(1) {
                    Some(word) => word,
                    None => "",
                };
                let mut res = Response::new(RES_STATUS_LINE.to_string());
                res.add_header("Content-Type".to_string(), "text/plain".to_string());
                res.add_header("Content-Length".to_string(), stuff.len().to_string());
                res.body = Some(stuff.as_bytes().to_vec());
                res.encode(req);
                res.to_vec_u8()
            },
        }
    }

    fn user_agent() -> Route {
        Route {
            name: "user-agent".to_string(),
            visit: |req| {
                let agent = match req.headers.get("User-Agent") {
                    Some(word) => word,
                    None => "",
                };
                let mut res = Response::new(RES_STATUS_LINE.to_string());
                res.add_header("Content-Type".to_string(), "text/plain".to_string());
                res.add_header("Content-Length".to_string(), agent.len().to_string());
                res.body = Some(agent.as_bytes().to_vec());
                res.to_vec_u8()
            },
        }
    }

    fn files() -> Route {
        Route {
            name: "files".to_string(),
            visit: |req| match req.method {
                crate::HttpMethod::Get => Routes::get_files(req),
                crate::HttpMethod::Post => Routes::post_files(req),
                _ => return Routes::not_found(),
            },
        }
    }

    fn get_files(req: Request) -> Vec<u8> {
        let mut res = Response::new(RES_STATUS_LINE.to_string());
        res.add_header("Content-Type".to_string(), "application/octet-stream".to_string());
        let file_path = req.resource[1..].split('/').nth(1);
        let file_bytes = match file_path {
            Some(path) => {
                let dir: String = env::args().nth(2).unwrap();
                let full_path = format!("{}/{}", &dir, path);
                match fs::read(full_path) {
                    Ok(fb) => fb,
                    _ => return Routes::not_found(),
                }
            }
            None => return Routes::not_found(),
        };
        res.add_header("Content-Length".to_string(), file_bytes.len().to_string());
        res.body = Some(file_bytes);
        res.to_vec_u8()
    }

    fn post_files(req: Request) -> Vec<u8> {
        let file_path = req.resource[1..].split('/').nth(1);
        match file_path {
            Some(path) => {
                let dir: String = env::args().nth(2).unwrap();
                let full_path = format!("{}/{}", &dir, path);
                match fs::write(full_path, &req.body) {
                    Ok(_) => return "HTTP/1.1 201 Created\r\n\r\n".as_bytes().to_vec(),
                    _ => return Routes::not_found(),
                }
            }
            None => return Routes::not_found(),
        };
    }

    fn not_found() -> Vec<u8> {
        "HTTP/1.1 404 Not Found\r\n\r\n".as_bytes().to_vec()
    }
}

#[derive(Debug)]
pub struct Route {
    pub name: String,
    pub visit: fn(req: Request) -> Vec<u8>,
}
