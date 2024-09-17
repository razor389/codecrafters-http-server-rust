use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use crate::helpers::response::{respond_with_error, respond_with_status};
use crate::helpers::utils::{get_content_length, extract_body};

pub fn handle_post_files(
    stream: &mut std::net::TcpStream,
    request: &str,
    buffer: &[u8],
    bytes_read: usize,
    directory: Option<String>,
) {
    if let Some(directory) = directory {
        let filename = &request.split_whitespace().nth(1).unwrap()[7..];
        let file_path = Path::new(&directory).join(filename);

        let content_length = get_content_length(request);
        let mut body = extract_body(request, buffer, bytes_read);

        while body.len() < content_length {
            let mut chunk = vec![0; content_length - body.len()];
            let bytes_read = stream.read(&mut chunk).unwrap();
            body.extend_from_slice(&chunk[..bytes_read]);
        }

        let mut file = File::create(file_path).unwrap();
        file.write_all(&body).unwrap();
        respond_with_status(stream, 201);
    } else {
        respond_with_error(stream, 400);
    }
}

pub fn handle_get_files(stream: &mut std::net::TcpStream, path: &str, directory: Option<String>) {
    if let Some(directory) = directory {
        let filename = &path[7..];
        let file_path = Path::new(&directory).join(filename);

        if file_path.exists() && file_path.is_file() {
            let mut file = File::open(file_path).unwrap();
            let mut file_contents = Vec::new();
            file.read_to_end(&mut file_contents).unwrap();

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                file_contents.len()
            );
            stream.write(response.as_bytes()).unwrap();
            stream.write(&file_contents).unwrap();
            stream.flush().unwrap();
        } else {
            respond_with_error(stream, 404);
        }
    } else {
        println!("responding with error");
        respond_with_error(stream, 400);
    }
}
