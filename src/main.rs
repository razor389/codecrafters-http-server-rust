#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    println!("Server running on 127.0.0.1:4221...");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 512];
                stream.read(&mut buffer).unwrap();
                
                // Convert buffer to a string to examine the request
                let request = String::from_utf8_lossy(&buffer[..]);
                
                // Log the incoming request for debugging
                println!("Received request here: {}", request);

                // Parse the request line (first line of the request)
                let request_line = request.lines().next().unwrap_or("");
                
                // Split the request line into components: method, path, version
                let parts: Vec<&str> = request_line.split_whitespace().collect();
                println!("parts of request line: {:?}", parts);
                if parts.len() >= 2 {
                    let method = parts[0];
                    let path = parts[1];

                    if method == "GET" && path == "/" {
                        // Respond with 200 OK for the root "/"
                        let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
                        stream.write(response.as_bytes()).unwrap();
                        stream.flush().unwrap();
                    } else if method == "GET" && path.starts_with("/echo/") {
                        // Extract the string after "/echo/"
                        let echo_str = &path[6..]; // Get the part after "/echo/"
                        
                        // Prepare an HTTP response with the echoed string
                        let response_body = format!("{}", echo_str);
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                            response_body.len(),
                            response_body
                        );
                        stream.write(response.as_bytes()).unwrap();
                        stream.flush().unwrap();
                    } else {
                        // Respond with 404 Not Found for any other path
                        let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
                        stream.write(response.as_bytes()).unwrap();
                        stream.flush().unwrap();
                    }
                } else {
                    // If the request is malformed, also respond with 404
                    let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
