use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file_arg = env::args().nth(1);
    match file_arg {
        Some(file) => read_syx(file),
        None => {
            println!("Usage: cargo run file.syx");
            std::process::exit(-1);
        }
    }
}

fn read_syx(file: String) {
    let mut file = File::open(file).unwrap();
    let mut buf = [0u8; 12];
    file.read(&mut buf);
    for b in buf.iter() {
        print!("{:X} ", b);
    }
}