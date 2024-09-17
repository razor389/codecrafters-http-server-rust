use std::io::Write;

pub fn handle_user_agent(stream: &mut std::net::TcpStream, request: &str) {
    let mut user_agent = "Unknown";
    for line in request.lines() {
        if line.to_lowercase().starts_with("user-agent:") {
            user_agent = line.split_at("User-Agent: ".len()).1.trim();
            break;
        }
    }

    let response_body = format!("{}", user_agent);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
