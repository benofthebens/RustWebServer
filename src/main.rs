// Importing modules from the rust libaries 
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use serde::Deserialize;

fn handle_client(mut stream: TcpStream, services: &Vec<Service>) {
    // A buffer to read data from the client
    let mut buffer = [0; 1024];

    // Reads data from the client stream and stores it in the buffer
    stream.read(&mut buffer).expect("Failed to read from client");

    // Converts the data in the buffer to a UTF-8 string
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);
    let path = request
		.lines()
		.next()
		.and_then(|line| line.split_whitespace().nth(1)).unwrap_or("/");

    if let Some(service) = services.iter().find(|s| s.location == path) {
        let service_socket = format!("{}:{}", service.host, service.port);
        println!("Connecting to service at: {}", service_socket);

        if let Ok(mut service_stream) = TcpStream::connect(&service_socket) {
            // Forward the client request to the service
            service_stream.write_all(request.as_bytes())
				.expect("Failed to write to service");

            // Buffer to store the service response
            let mut service_buffer = Vec::new();

            // Read the response from the service
            service_stream.read_to_end(&mut service_buffer)
				.expect("Failed to read from service");

            // Write the service response back to the client
            stream.write_all(&service_buffer)
				.expect("Failed to write response to client");
				
            stream.flush().expect("Failed to flush client stream");

        } 
		else {
            eprintln!("Failed to connect to service on port {}", service.port);
        }
    } 
	else {
        // Serve the default index.html file if no service matches the path
        let binding = fs::read_to_string("./index.html").expect("Failed to read file");
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
            binding.len(),
            binding
        );

        stream.write_all(response.as_bytes()).expect("Failed to write response to client");
        stream.flush().expect("Failed to flush client stream");
    }
}
#[derive(Debug, Deserialize, Clone)]
struct Service {
    host: String,
    port: String,
    location: String, 
}
fn main() {
    let config_file_path = std::path::Path::new("./config/services.json");
    let config_file = fs::File::open(config_file_path)
        .expect("File not found");
    let services: Vec<Service> = serde_json::from_reader(config_file)
        .expect("Error while reading from file");
    for service in &services {
        println!("{:?}",service);
    }
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("failed to bind to address");
    println!("Server lisntening");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let services = services.clone();
                std::thread::spawn(move || handle_client(stream, &services));
            }
            Err(e) => {
                // standard error stream
                eprintln!("Failed to establish connection: {e}");
            }
        }
    }

}

