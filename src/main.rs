#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Bind the listener to the address and port
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    // Handle incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");

                // Create a buffer to read the incoming request
                let mut buffer = [0; 512];
                stream.read(&mut buffer).unwrap();

                // Prepare a basic HTTP 200 OK response
                let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
                
                // Write the response to the stream
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
