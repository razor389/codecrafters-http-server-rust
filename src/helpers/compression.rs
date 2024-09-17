use std::io::Write;

use flate2::write::GzEncoder;
use flate2::Compression;

pub fn supports_gzip(request: &str) -> bool {
    for line in request.lines() {
        if line.to_lowercase().starts_with("accept-encoding:") {
            let encodings: Vec<&str> = line.split(":").nth(1).unwrap().split(',').map(|s| s.trim()).collect();
            return encodings.contains(&"gzip");
        }
    }
    false
}

pub fn compress_gzip(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}
