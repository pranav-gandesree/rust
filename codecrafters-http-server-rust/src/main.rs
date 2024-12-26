use std::io::prelude::*;
use std::io::Write;
#[allow(unused_imports)]
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {

    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection (mut stream: TcpStream) {
    let mut buffer = [0; 2024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));


    let response_contents = fs::read_to_string("index.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response_contents.len(),
        response_contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}