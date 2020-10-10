use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let s = stream.unwrap();
        handle_connection(s);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get_test = b"GET /test HTTP/1.1\r\n";

    if buffer.starts_with(get_test) {
        test_reponse(stream)
    } else {
        hello_reponse(stream)
    }
}

fn hello_reponse(mut stream: TcpStream) {
    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn test_reponse(mut stream: TcpStream) {
    let contents = fs::read_to_string("test.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}