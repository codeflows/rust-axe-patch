extern crate rusty_axe;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    if env::args().len() <= 1 {
        println!("Usage: cargo run file1.syx file2.syx ...");
        std::process::exit(-1);
    }
    for file in env::args().skip(1) {
        let data = read_file(&file);
        let preset = rusty_axe::parse_preset(&data);
        println!("{}: {:?}", file, preset);
    }
}

fn read_file(file_name: &str) -> Vec<u8> {
    let mut file = File::open(file_name).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    return buf;
}