use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    if env::args().len() <= 1 {
        println!("Usage: cargo run file1.syx file2.syx ...");
        std::process::exit(-1);
    }
    for file in env::args().skip(1) {
        read_syx(file)
    }
}

fn read_syx(file_name: String) {
    println!("Reading {} ...", file_name);

    let mut file = File::open(file_name).unwrap();
    let mut buf = [0u8; 12];
    file.read(&mut buf).unwrap();

    for b in buf.iter() {
        print!("{:01$X} ", b, 2);
    }
    println!("");

    if !validate_header(&buf) {
        println!("This does not look like a Axe FX patch file.");
        std::process::exit(-1);
    }

    println!("Axe FX model: {}", axe_model_name(buf[4]));

    if buf[6] == 0x7f {
        println!("Patch is targeting current edit buffer")
    } else {
        println!("Patch is targeting bank {} preset {}", buf[6], buf[7])
    }
}

fn validate_header(buf: &[u8]) -> bool {
    buf[0] == 0xF0 && buf[1] == 0x00 && buf[2] == 0x01 && buf[3] == 0x74 &&
    buf[5] == 0x77 &&
    buf[11] == 0xf7
}

fn axe_model_name(code: u8) -> &'static str {
    match code {
        0x03 => "Axe-Fx II",
        0x05 => "FX8",
        0x06 => "Axe-Fx II XL",
        0x07 => "Axe-Fx II XL+",
        0x08 => "AX8",
        _    => "Unknown"
    }
}