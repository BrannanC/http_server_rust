use std::collections::HashMap;
use std::io::Write;
use flate2::Compression;
use flate2::write::GzEncoder;

use crate::Request;



pub struct Response {
    pub status: String,
    pub content_type: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(status: String) -> Response {
        Response {
            status,
            content_type: None,
            headers: None,
            body: None
        }
    }

    pub fn add_header(&mut self, key: String, value: String) {
        if self.headers.is_none() {
            self.headers = Some(HashMap::new());
        }

        if let Some(headers) = &mut self.headers {
            headers.insert(key, value);
        }
    }

    pub fn to_vec_u8(&self) -> Vec<u8> {
        let mut res = self.status.as_bytes().to_vec();
        res.extend(b"\r\n");

        if let Some(ct) = &self.content_type {
            res.extend(ct.as_bytes());
        }

        if let Some(headers) = &self.headers {
            for (k, v) in headers.iter() {
                res.extend(format!("{k}: {v}\r\n").as_bytes());
            }
        }

        res.extend(b"\r\n");
        if let Some(body) = &self.body {
            res.extend(body);
        }

        res
    }

    pub fn encode(&mut self, req: Request) {
        if let Some(encodings) = req.headers.get("Accept-Encoding") {
            for enc in encodings.split(',') {
                if Response::is_valid_enc(enc.trim()) {
                    self.add_header("Content-Encoding".to_string(), enc.to_string());
                    self.gzip_body();
                    break;
                }
            }
        }
    }

    fn gzip_body(&mut self) {
        if let Some(body) = &self.body {
            let mut encoder = GzEncoder::new(vec![], Compression::default());
            encoder.write_all(&body).unwrap();
            let compressed = encoder.finish().unwrap();
            self.add_header("Content-Length".to_string(), compressed.len().to_string());
            self.body = Some(compressed);
        }
    }

    fn is_valid_enc(enc: &str) -> bool {
        if enc == "gzip" {
            return true;
        }
        return false;
    }
}