// Copyright (c) 2021 Exopteron, Galaxtone
// Licensed under GNU General Public License v3

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        std::thread::spawn(move || {
            on_connect(stream.unwrap()); 
        });
    }
}

#[non_exhaustive]
enum Method {
    Get
}

// Request-Line = Method SP Request-URI SP HTTP-Version CRLF
#[derive(Default)]
struct RequestLine {
    method: String,
    path: String,
    version: String,
}

impl RequestLine {
    pub fn new(line: &str) -> RequestLine {
        let parts: Vec<&str> = line.split(" ").collect();
        RequestLine {
            method: parts[0].to_string(),
            path: parts[1].to_string(),
            version: parts[2].to_string(),
        }
    }
}

// https://www.w3.org/Protocols/rfc2616/rfc2616-sec5.html
// Official documentation on HTTP, with grammar lines

fn on_connect(mut stream: TcpStream) {
    println!("Connection!");
    let mut bytes = vec![0; 1024];
    let bytescount = stream.read(&mut bytes).unwrap();
    bytes.drain(bytescount..);
    println!("Bytes: {:?}", String::from_utf8_lossy(&bytes));

    let raw_request = String::from_utf8_lossy(&bytes);
    let lines: Vec<&str> = raw_request.split("\r\n").collect();
    let request_line = RequestLine::new(lines[0]);

    let file = get_dynamic_page(request_line.path);
    if file.is_none() {
        let text = std::fs::read_to_string("404.html").unwrap();
        let response = format!("HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", text.len(), text);
        stream.write(&response.as_bytes());
    }
}

fn get_dynamic_page(path: &str) -> Option<&str> {
    None
    // returns None if it doesn't exist, serve file
}