use std::net::{TcpStream};
use std::io::{Read, Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8001")
        .expect("Unable to connect to service");
    let request = "GET / HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n";
    stream.write_all(request.as_bytes())
        .expect("Failed to write to stream");

    // Buffer to store the response
    let mut buffer = Vec::new();

    // Read the response into the buffer
    stream.read_to_end(&mut buffer)
        .expect("Failed to read from stream");

    // Convert the buffer to a string and print the response
    let response = String::from_utf8_lossy(&buffer);
    println!("Response from server:\n{}", response);
 

}
