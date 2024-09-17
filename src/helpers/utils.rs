pub fn get_content_length(request: &str) -> usize {
    for line in request.lines() {
        if line.starts_with("Content-Length:") {
            return line.split(":").nth(1).unwrap().trim().parse::<usize>().unwrap_or(0);
        }
    }
    0
}

pub fn extract_body(request: &str, buffer: &[u8], bytes_read: usize) -> Vec<u8> {
    let mut body = Vec::new();
    if let Some(index) = request.find("\r\n\r\n") {
        let body_start = index + 4; // Skip the `\r\n\r\n`
        let remaining_in_buffer = bytes_read - body_start;
        if remaining_in_buffer > 0 {
            body.extend_from_slice(&buffer[body_start..bytes_read]);
        }
    }
    body
}
