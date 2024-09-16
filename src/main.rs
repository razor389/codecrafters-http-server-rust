#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::{Read, Write};
use std::fs::File;
use std::env;
use std::path::Path;
use std::thread;

fn handle_connection(mut stream: std::net::TcpStream, directory: Option<String>) {
    let mut buffer = [0; 512];  // A buffer to read incoming data
    let bytes_read = stream.read(&mut buffer).unwrap();

    // Convert buffer to a string to examine the request headers
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    // Log the incoming request for debugging
    println!("Received request: {}", request);

    // Parse the request line (first line of the request)
    let request_line = request.lines().next().unwrap_or("");

    // Split the request line into components: method, path, version
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() >= 2 {
        let method = parts[0];
        let path = parts[1];
        
        // Handle POST /files/{filename} requests
        if method == "POST" && path.starts_with("/files/") {
            if let Some(directory) = &directory {
                // Extract the filename from the path
                let filename = &path[7..]; // The part after "/files/"
                let file_path = Path::new(&directory).join(filename);

                // Get the Content-Length header to determine the size of the body
                let mut content_length = 0;
                for line in request.lines() {
                    if line.starts_with("Content-Length:") {
                        let len_str = line.split(":").nth(1).unwrap().trim();
                        content_length = len_str.parse::<usize>().unwrap_or(0);
                    }
                }

                // Initialize a vector to hold the entire body
                let mut body = Vec::with_capacity(content_length);

                // Get the part of the body that is already in the buffer after the headers
                if let Some(index) = request.find("\r\n\r\n") {
                    let body_start = index + 4; // Skip the `\r\n\r\n`
                    let remaining_in_buffer = bytes_read - body_start;
                    if remaining_in_buffer > 0 {
                        body.extend_from_slice(&buffer[body_start..bytes_read]);
                    }
                }

                // Read the remaining body from the stream if necessary
                while body.len() < content_length {
                    let mut chunk = vec![0; content_length - body.len()];
                    let bytes_read = stream.read(&mut chunk).unwrap();
                    body.extend_from_slice(&chunk[..bytes_read]);
                }

                // Write the body content to the specified file
                let mut file = File::create(file_path).unwrap();
                file.write_all(&body).unwrap();

                // Respond with 201 Created
                let response = "HTTP/1.1 201 Created\r\nContent-Length: 0\r\n\r\n";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            } else {
                // Directory not provided, return 400 Bad Request
                let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        } 
        // Handle GET /files/{filename} requests 
        else if method == "GET" && path.starts_with("/files/") {
            if let Some(directory) = &directory {
                // Extract the filename from the path
                let filename = &path[7..]; // The part after "/files/"
                let file_path = Path::new(&directory).join(filename);

                if file_path.exists() && file_path.is_file() {
                    // Open the file and read its contents
                    let mut file = File::open(file_path).unwrap();
                    let mut file_contents = Vec::new();
                    file.read_to_end(&mut file_contents).unwrap();
                    
                    // Create the HTTP response with file contents
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                        file_contents.len()
                    );
                    stream.write(response.as_bytes()).unwrap();
                    stream.write(&file_contents).unwrap();
                    stream.flush().unwrap();
                } else {
                    // File not found, return 404
                    let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            } else {
                // Directory not provided, return 400 Bad Request
                let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        } else if method == "GET" && path == "/user-agent" {
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
            let response_body = format!("{}", user_agent);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
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
            let response_body = format!("{}", echo_str);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
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

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let mut directory: Option<String> = None;

    // Check if the --directory flag is provided
    if args.len() > 2 && args[1] == "--directory" {
        directory = Some(args[2].clone());
    }

    if let Some(dir) = &directory {
        println!("Serving files from directory: {}", dir);
    } else {
        println!("No directory specified. /files requests will return a 400 Bad Request.");
    }

    println!("Server running on 127.0.0.1:4221...");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    // Loop to accept incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let directory = directory.clone();
                // Spawn a new thread for each connection
                thread::spawn(move || {
                    handle_connection(stream, directory);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}