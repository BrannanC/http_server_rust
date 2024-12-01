use crate::Request;

const RES_STATUS_LINE: &str = "HTTP/1.1 200 OK\r\n";
const NOT_FOUND_STATUS_LINE: &str = "HTTP/1.1 404 Not Found\r\n";
const CONTENT_TYPE_TEXT: &str = "Content-Type: text/plain\r\n";

#[derive(Debug)]
pub struct Routes {}

impl Routes {
    pub fn build() -> Vec<Route> {
        vec![Routes::home(), Routes::echo(), Routes::user_agent()]
    }
    fn home() -> Route {
        Route {
            name: "".to_string(),
            subroutes: None,
            visit: |_| {
                let mut res = RES_STATUS_LINE.as_bytes().to_vec();
                res.extend(b"\r\n".to_vec());
                return res;
            },
        }
    }

    fn echo() -> Route {
        Route {
            name: "echo".to_string(),
            subroutes: None,
            visit: |req| {
                        let mut resource = req.resource[1..].split('/');
                        let stuff = match resource.nth(1){
                            Some(word) => word,
                            None => "",
                        };
                        let mut res = RES_STATUS_LINE.as_bytes().to_vec();
                        res.extend(CONTENT_TYPE_TEXT.as_bytes()); 
                        res.extend(format!("Content-Length: {}\r\n\r\n{stuff}", stuff.len()).as_bytes());
                        res
            },
        }
    }

    fn user_agent() -> Route {
        Route {
            name: "user-agent".to_string(),
            subroutes: None,
            visit: |req| {
                        let mut res = RES_STATUS_LINE.as_bytes().to_vec();
                        res.extend(CONTENT_TYPE_TEXT.as_bytes()); 
                        let agent = match req.headers.get("User-Agent") {
                            Some(word) => word,
                            None => "",
                        };
                        res.extend(format!("Content-Length: {}\r\n\r\n{agent}", agent.len()).as_bytes());
                        res
            },
        }
    }
}

#[derive(Debug)]
pub struct Route {
    pub name: String,
    subroutes: Option<Vec<Route>>,
    pub visit: fn(req: Request) -> Vec<u8>,
}