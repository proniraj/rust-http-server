use std::collections::HashMap;
use std::io::{Read, Write};

#[allow(unused_imports)]
use std::net::TcpListener;

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
                println!("incomming request: {:?}", request);

                match request.request_line.path.as_str() {
                    "/" => {
                        let response = b"HTTP/1.1 200 OK\r\n\r\n";
                        stream.write_all(response).unwrap();
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

#[derive(Debug)]
struct Request {
    request_line: RequestLine,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    fn from_bytes(bytes: &[u8]) -> Self {
        let request_string = String::from_utf8_lossy(bytes).to_string().replace("\0", "");

        let request_line = RequestLine::from_string(request_string.split("\r\n").next().unwrap());
        let headers = HashMap::new();
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
