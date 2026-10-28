use std::collections::HashMap;
use std::fmt;
use std::io::{Read, Write};

#[allow(unused_imports)]
use std::net::TcpListener;

use anyhow::Error;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0_u8; 1024];

                // read from stream into buffer
                stream.read(&mut buffer).unwrap();

                let request_string = String::from_utf8_lossy(&buffer)
                    .to_string()
                    .replace("\0", "");

                let request = Request::from_bytes(&buffer);

                println!("incomming request: {:?}", request_string);
                // println!("incomming request: {:?}", request);

                match request.request_line.path.as_str() {
                    "/" => {
                        let response = b"HTTP/1.1 200 OK\r\n\r\n";
                        stream.write_all(response).unwrap();
                    }
                    two_part
                        if two_part.starts_with("/echo/") && two_part.split("/").count() == 3 =>
                    {
                        let parts = two_part.split("/").collect::<Vec<&str>>();

                        let mut response = Response::default();

                        response.set_status(200).unwrap();

                        response
                            .headers
                            .set(String::from("Content-Type"), &String::from("Text/Plain"));

                        if let Some(slug) = parts.get(2) {
                            response.set_body(&slug.to_string()).unwrap();
                        }

                        let response_string = response.to_string();

                        println!("The response is: {:?}", response_string);

                        stream.write_all(&response_string.into_bytes()).unwrap();
                    }
                    _ => {
                        let response = b"HTTP/1.1 404 NOT FOUND\r\n\r\n";
                        stream.write_all(response).unwrap();
                    }
                }

                // flush stream, so the response is sent to the client
                stream.flush().unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

#[derive(Debug)]
struct RequestLine {
    method: String,
    path: String,
    version: String,
}

#[derive(Debug, Default)]
struct HTTPHeaders {
    headers: HashMap<String, String>,
}

impl HTTPHeaders {
    fn set(&mut self, key: String, value: &String) {
        self.headers.insert(key, value.to_owned());
    }
}

impl fmt::Display for HTTPHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format each key-value pair and join them with a comma and space
        let formatted_pairs: Vec<String> = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();

        write!(f, "{}", formatted_pairs.join("\r\n"))
    }
}

#[derive(Debug)]
struct Request {
    request_line: RequestLine,
    headers: HTTPHeaders,
    body: String,
}

impl Request {
    fn from_bytes(bytes: &[u8]) -> Self {
        let request_string = String::from_utf8_lossy(bytes).to_string().replace("\0", "");

        let request_line = RequestLine::from_string(request_string.split("\r\n").next().unwrap());
        let headers = HTTPHeaders::default();
        let body = String::new();
        Self {
            request_line,
            headers,
            body,
        }
    }
}

impl RequestLine {
    fn from_string(string: &str) -> Self {
        let parts = string.split(" ").collect::<Vec<&str>>();
        Self {
            method: parts[0].to_string(),
            path: parts[1].to_string(),
            version: parts[2].to_string(),
        }
    }
}

#[derive(Debug)]
struct StatusLine {
    http_version: String,  // later we can use enum or extend this
    status_code: u16,      // 200
    reason_phrase: String, // OK
}

impl Default for StatusLine {
    fn default() -> Self {
        Self {
            http_version: String::from("HTTP/1.1"),
            status_code: 200,
            reason_phrase: String::from("OK"),
        }
    }
}

impl StatusLine {
    fn version(&mut self, version: String) {
        self.http_version = version;
    }

    fn code(&mut self, code: u16) {
        self.status_code = code;
    }

    fn phrase(&mut self, phrase: String) {
        self.reason_phrase = phrase;
    }
}

#[derive(Debug, Default)]
struct ResponseBody {
    body: String,
}

impl fmt::Display for ResponseBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body)
    }
}

#[derive(Debug, Default)]
struct Response {
    status_line: StatusLine,
    headers: HTTPHeaders,
    body: ResponseBody,
}

impl Response {
    fn set_body(&mut self, body: &String) -> Result<(), Error> {
        let body_size = body.len().to_string();
        self.body = ResponseBody { body: body.clone() };

        self.headers.set(String::from("Content-Length"), &body_size);

        Ok(())
    }

    fn set_status(&mut self, status_code: u16) -> Result<(), Error> {
        self.status_line.code(status_code);

        match status_code {
            200 => {
                self.status_line.phrase(String::from("OK"));
            }
            404 => {
                self.status_line.phrase(String::from("NOT FOUND"));
            }
            _ => {
                self.status_line.phrase(String::from("OK"));
            }
        }
        Ok(())
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_line_string = format!(
            "{} {} {}",
            self.status_line.http_version,
            self.status_line.status_code,
            self.status_line.reason_phrase
        );

        let headers_string = self.headers.to_string();

        write!(
            f,
            "{}\r\n{}\r\n\r\n{}",
            status_line_string, headers_string, self.body
        )
    }
}
