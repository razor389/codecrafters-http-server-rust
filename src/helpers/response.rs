use std::io::Write;

pub fn respond_with_ok(stream: &mut std::net::TcpStream) {
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn respond_with_status(stream: &mut std::net::TcpStream, status_code: u16) {
    let response = format!("HTTP/1.1 {} Created\r\nContent-Length: 0\r\n\r\n", status_code);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn respond_with_error(stream: &mut std::net::TcpStream, status_code: u16) {
    let response = format!("HTTP/1.1 {} Error\r\nContent-Length: 0\r\n\r\n", status_code);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
