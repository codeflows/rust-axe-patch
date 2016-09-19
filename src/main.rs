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

fn read_syx(file_name: String) {
    println!("Reading {} ...", file_name);

    let mut file = File::open(file_name).unwrap();
    let mut buf = [0u8; 12];
    file.read(&mut buf).unwrap();

    if !validate(&buf) {
        println!("This does not look like a Axe FX patch file.");
        std::process::exit(-1);
    }

    for b in buf.iter() {
        print!("{:01$X} ", b, 2);
    }
}

fn validate(buf: &[u8]) -> bool {
    buf.len() > 4 && buf[0] == 0xF0 && buf[1] == 0x00 && buf[2] == 0x01 && buf[3] == 0x74
}