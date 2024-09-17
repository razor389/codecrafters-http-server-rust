mod handlers;
mod helpers;

use std::io::Read;
use std::net::TcpListener;
use std::env;
use std::thread;
use handlers::files::{handle_get_files, handle_post_files};
use handlers::echo::handle_echo;
use handlers::user_agent::handle_user_agent;
use helpers::compression::supports_gzip;
use helpers::response::{respond_with_ok, respond_with_error};

fn handle_connection(mut stream: std::net::TcpStream, directory: Option<String>) {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received request: {}", request);

    let request_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    if parts.len() < 2 {
        respond_with_error(&mut stream, 404);
        return;
    }

    let method = parts[0];
    let path = parts[1];

    if method == "POST" && path.starts_with("/files/") {
        handle_post_files(&mut stream, &request, &buffer, bytes_read, directory);
    } else if method == "GET" && path.starts_with("/files/") {
        handle_get_files(&mut stream, path, directory);
    } else if method == "GET" && path == "/user-agent" {
        handle_user_agent(&mut stream, &request);
    } else if method == "GET" && path == "/" {
        respond_with_ok(&mut stream);
    } else if method == "GET" && path.starts_with("/echo/") {
        handle_echo(&mut stream, path, supports_gzip(&request));
    } else {
        respond_with_error(&mut stream, 404);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut directory: Option<String> = None;

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

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let directory = directory.clone();
                thread::spawn(move || handle_connection(stream, directory));
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
