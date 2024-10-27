// Importing modules from the rust libaries 
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;

fn handle_client(mut stream: TcpStream) {

    // a buffer to read data from the client
    let mut buffer = [0;1024];

    // reads data from the stream and stores it in the buffer
    stream.read(&mut buffer)
        .expect("Failed to read from client");

    // converts the data in the buffer to utf8 string 
    let request = String::from_utf8_lossy(&buffer[..]);
    
    println!("Recived request: {request}");
    
    let binding  = fs::read_to_string("./index.html")
        .expect("Failed to read file");

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        binding.len(),
        binding
    );

    stream.write_all(response.as_bytes()).expect("Failed to write response");
    stream.flush().expect("Failed to flush stream");

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("failed to bind to address");
    println!("Server lisntening");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                // standard error stream
                eprintln!("Failed to establish connection: {e}");
            }
        }
    }

}

