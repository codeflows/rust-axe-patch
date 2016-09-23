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

    // http://forum.fractalaudio.com/threads/help-loading-presets-using-sysex-librarian.58581/#post732659
    if buf[6] == 0x7f {
        println!("Patch is targeting current edit buffer")
    } else {
        println!("Patch is targeting bank {} preset {}", buf[6], buf[7])
    }
}

// References:
// http://wiki.fractalaudio.com/axefx2/index.php?title=MIDI_SysEx
// http://wiki.fractalaudio.com/axefx2/index.php?title=Preset_management
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
        // ?
        buf[5] == 0x77 ||
        // MIDI_PATCH_DUMP?
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