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
                println!("Received request: {}", request);

                // Parse the request line (first line of the request)
                let request_line = request.lines().next().unwrap_or("");
                
                // Split the request line into components: method, path, version
                let parts: Vec<&str> = request_line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let method = parts[0];
                    let path = parts[1];
                    
                    // Handle GET requests
                    if method == "GET" && path == "/user-agent" {
                        // Parse headers to extract "User-Agent"
                        let mut user_agent = "Unknown"; // Default value
                        for line in request.lines() {
                            if line.to_lowercase().starts_with("user-agent:") {
                                user_agent = line.split_at("User-Agent: ".len()).1.trim();
                                break;
                            }
                        }

                        // Log the request details
                        println!("Host: localhost:4221");
                        println!("User-Agent: {}", user_agent);

                        // Respond with the User-Agent
                        let response_body = format!("User-Agent: {}", user_agent);
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                            response_body.len(),
                            response_body
                        );
                        stream.write(response.as_bytes()).unwrap();
                        stream.flush().unwrap();
                    } else if method == "GET" && path == "/" {
                        // Handle root "/"
                        let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
                        stream.write(response.as_bytes()).unwrap();
                        stream.flush().unwrap();
                    } else if method == "GET" && path.starts_with("/echo/") {
                        // Handle /echo/{str}
                        let echo_str = &path[6..]; // Extract the part after "/echo/"
                        let response_body = format!("You said: {}", echo_str);
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                            response_body.len(),
                            response_body
                        );
                        stream.write(response.as_bytes()).unwrap();
                        stream.flush().unwrap();
                    } else {
                        // Handle 404 Not Found
                        let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
                        stream.write(response.as_bytes()).unwrap();
                        stream.flush().unwrap();
                    }
                } else {
                    // Handle malformed request (404)
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
