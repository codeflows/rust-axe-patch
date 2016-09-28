use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    if env::args().len() <= 1 {
        println!("Usage: cargo run file1.syx file2.syx ...");
        std::process::exit(-1);
    }
    for file in env::args().skip(1) {
        read_sysex_file(file)
    }
}

fn read_sysex_file(file_name: String) {
    println!("Reading {} ...", file_name);

    let mut file = File::open(file_name).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    parse_sysex_data(&buf);
}

const SYSEX_MESSAGE_START_BYTE: u8 = 0xf0;
const SYSEX_MESSAGE_END_BYTE: u8 = 0xf7;

fn parse_sysex_data(data: &[u8]) {
    let start = data[0];
    if start != SYSEX_MESSAGE_START_BYTE {
        panic!("No start byte found");
    }
    let end = find_sysex_message_end(data).unwrap();
    let message = &data[0..end+1];
    read_syx(message);
}

fn find_sysex_message_end(data: &[u8]) -> Option<usize> {
    for (index, byte) in data.iter().enumerate() {
        if *byte == SYSEX_MESSAGE_END_BYTE {
            return Some(index);
        }
    }
    return None;
}

fn read_syx(buf: &[u8]) {
    let model = axe_model_name(buf[4]);
    println!("Axe FX model: {}", model);

    if !validate_header(&buf) {
        println!("This does not look like a Axe FX patch file.");
        print_bytes(buf);
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
    )
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

fn print_bytes(buf: &[u8]) {
    for b in buf.iter() {
        print!("{:01$X} ", b, 2);
    }
    println!("");
}
