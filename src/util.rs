use std::fmt::Write;

pub fn format_hex(buf: &[u8]) -> String {
    let mut s = String::new();
    for &byte in buf {
        write!(&mut s, "{:02X} ", byte).unwrap();
    }
    return s.trim().to_string();
}
