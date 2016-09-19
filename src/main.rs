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

    if !validate_header(&buf) {
        println!("This does not look like a Axe FX patch file.");
        std::process::exit(-1);
    }

    for b in buf.iter() {
        print!("{:01$X} ", b, 2);
    }
    println!("");

    println!("Axe FX model: {} ({:02$X})", axe_model_name(buf[4]), buf[4], 2);

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
        0x06 => "Axe-Fx II XL",
        0x07 => "Axe-Fx II XL+",
        _    => "Unknown"
    }
}