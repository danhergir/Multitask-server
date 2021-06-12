use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;

// Simple server using TCP. Works for HTTP requests 
fn main() {

	let listener = TcpListener::bind("127.0.0.1:7373").unwrap();


    	for stream in listener.incoming() {
        	let stream = stream.unwrap();

        	handle_connection(stream);
    	}
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 256];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("src/index.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap(); 
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    	let contents = fs::read_to_string("src/404.html").unwrap();

    	let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);

    	stream.write(response.as_bytes()).unwrap();
    	stream.flush().unwrap();
    }
}
