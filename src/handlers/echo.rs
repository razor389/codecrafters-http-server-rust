use std::io::Write;
use crate::helpers::compression::compress_gzip;

pub fn handle_echo(stream: &mut std::net::TcpStream, path: &str, supports_gzip: bool) {
    let echo_str = &path[6..];
    let response_body = echo_str.as_bytes();

    if supports_gzip {
        let compressed_body = compress_gzip(response_body);
        let headers = format!(
            "HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n",
            compressed_body.len()
        );
        stream.write(headers.as_bytes()).unwrap();
        stream.write(&compressed_body).unwrap();
    } else {
        let headers = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n",
            response_body.len()
        );
        stream.write(headers.as_bytes()).unwrap();
        stream.write(response_body).unwrap();
    }
    stream.flush().unwrap();
}
