mod sysex;

#[derive(Debug)]
pub struct Preset {
    model: &'static str,
    target: Target
}

#[derive(Debug)]
enum Target {
    CurrentEditBuffer,
    BankAndPreset { bank: u8, preset: u8 }
}

pub fn parse_preset(data: &[u8]) -> Option<Preset> {
    let messages = sysex::parse_sysex_messages(data);
    for (index, message) in messages.iter().enumerate() {
        match validate_message(message) {
            Some(error) => {
                println!("Error parsing sysex message #{}: {}", index, error);
                return None;
            }
            None => () 
        }
    }
    return None;//get_target(messages[0]);
}

type ParseError = String;

fn validate_message(message: &[u8]) -> Option<ParseError> {
    let result = validate_header(message);
    if result.is_some() {
        return result;
    }

    let (file_checksum, expected_checksum) = get_checksums(&message);
    if file_checksum != expected_checksum {
        return Some(format!("Invalid checksum: expected {:02$X} but got {:02$X}", expected_checksum, file_checksum, 2));
    }

    return None;
}

fn validate_header(message: &[u8]) -> Option<ParseError> {
    let file_header = &message[1..4];
    let expected_header = [
        // "Manufacturer sysex ID byte 0. As of firmware 8.02 this is always 00."
        0x00,
        // "Manufacturer sysex ID byte 1. As of firmware 10.02, this is always 01 (in previous firmware versions this was 00).""
        0x01,
        // "Manufacture sysex ID byte 2. As of firmware 10.02, this is 74 (in previous firmware versions this was 7D).""
        0x74
    ];
    if file_header != expected_header {
        return Some(format!("Invalid header bytes, expected {} but got {}", format_hex(&expected_header), format_hex(file_header)));
    }
    return None;
}

fn get_checksums(message: &[u8]) -> (u8, u8) {
    let checksum_index = message.len() - 2;
    let file_checksum = message[checksum_index];
    let xor = message[..checksum_index]
        .iter()
        .fold(0, |acc, &x| acc ^ x);
    let expected_checksum = xor & 0x7F;
    return (file_checksum, expected_checksum);
}

fn get_model_name(message: &[u8]) -> Option<&'static str> {
    message.get(4).map(|&code| {
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
    })
}

fn get_target(message: &[u8]) -> Target {
    if message[6] == 0x7f {
        return Target::CurrentEditBuffer;
    } else {
        return Target::BankAndPreset { bank: message[6], preset: message[7] }
    }
}

fn format_hex(buf: &[u8]) -> String {
    let hex: Vec<String> = buf.iter().map(|b| format!("{:01$X}", b, 2)).collect();
    return hex.join(" ");
}
