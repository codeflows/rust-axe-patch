mod sysex;
mod util;

use std::str;
use util::format_hex;

#[derive(Debug)]
pub struct Preset {
    model: String,
    name: Option<String>,
    target: Target
}

#[derive(Debug)]
enum Target {
    CurrentEditBuffer,
    BankAndPreset { bank: u8, preset: u8 }
}

type ParseError = String;

pub fn parse_preset(data: &[u8]) -> Result<Preset, ParseError> {
    let messages = sysex::parse_sysex_messages(data);
    for (index, message) in messages.iter().enumerate() {
        if let Some(error) = validate_message(message) {
            return Err(format!("Error parsing sysex message #{}: {}", index, error));
        }
    }
    let model = get_model_name(messages[0]).to_string();
    let name = messages.get(1).map(|message| get_name(message));
    let target = get_target(messages[0]);
    return Ok(
        Preset {
            model: model,
            name: name,
            target: target
        }
    )
}

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

fn get_model_name(message: &[u8]) -> &'static str {
    match message[4] {
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

const NAME_OFFSET: usize = 14;

fn get_name(message: &[u8]) -> String {
    let mut result: Vec<u8> = Vec::new();
    for (index, &byte) in message[NAME_OFFSET..].iter().enumerate() {
        if index % 3 == 0 {
            if byte == 0x00 {
                break;
            } else {
                result.push(byte);
            }
        }
    }
    return str::from_utf8(&result).unwrap().trim().to_string();
}

fn get_target(message: &[u8]) -> Target {
    if message[6] == 0x7f {
        return Target::CurrentEditBuffer;
    } else {
        return Target::BankAndPreset { bank: message[6], preset: message[7] }
    }
}
