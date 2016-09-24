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
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let model = axe_model_name(buf[4]);
    println!("Axe FX model: {}", model);

    if model == "Axe-Fx Standard" || model == "Axe-Fx Ultra" {
        println!("Skip validation for now.");
        return;
    }

    if !validate_header(&buf) {
        println!("This does not look like a Axe FX patch file.");
        for b in buf.iter() {
            print!("{:01$X} ", b, 2);
        }
        println!("");
        std::process::exit(-1);
    }

    // http://forum.fractalaudio.com/threads/help-loading-presets-using-sysex-librarian.58581/#post732659
    if buf[6] == 0x7f {
        println!("Patch is targeting current edit buffer")
    } else {
        println!("Patch is targeting bank {} preset {}", buf[6], buf[7])
    }
}

fn validate_header(buf: &[u8]) -> bool {
    // sysex start
    buf[0] == 0xF0 &&
    // "Manufacturer sysex ID byte 0. As of firmware 8.02 this is always 00."
    buf[1] == 0x00 &&
    // "Manufacturer sysex ID byte 1. As of firmware 10.02, this is always 01 (in previous firmware versions this was 00).""
    buf[2] == 0x01 &&
    // "Manufacture sysex ID byte 2. As of firmware 10.02, this is 74 (in previous firmware versions this was 7D).""
    buf[3] == 0x74 &&
    (
        // this seems to be the default
        buf[5] == 0x77 ||
        // seen this in at least one (2231)
        buf[5] == 0x7a ||
        // MIDI_PATCH_DUMP? standard and ultra patches?
        buf[5] == 0x04
    ) &&
    // TODO assuming header length is 12 bytes here, should search for end instead
    buf[11] == 0xf7
}

fn axe_model_name(code: u8) -> &'static str {
    match code {
        0x00 => "Axe-Fx Standard",
        0x01 => "Axe-Fx Ultra",
        0x03 => "Axe-Fx II",
        0x05 => "FX8",
        0x06 => "Axe-Fx II XL",
        0x07 => "Axe-Fx II XL+",
        0x08 => "AX8",
        _    => "Unknown"
    }
}