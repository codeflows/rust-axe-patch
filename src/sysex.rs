const SYSEX_MESSAGE_START_BYTE: u8 = 0xf0;
const SYSEX_MESSAGE_END_BYTE: u8 = 0xf7;

pub fn parse_sysex_messages(data: &[u8]) -> Vec<&[u8]> {
    let mut messages: Vec<&[u8]> = Vec::new();
    let mut remainder = data;

    while remainder.len() > 0 {
        let start = find_sysex_message_start(remainder).unwrap();
        let end = find_sysex_message_end(remainder).unwrap();
        let boundary = end + 1;
        let message = &remainder[start..boundary];
        messages.push(message);
        remainder = &remainder[boundary..];
    }

    return messages;
}

fn find_sysex_message_start(data: &[u8]) -> Option<usize> {
    data.get(0).and_then(|&byte| {
        if byte == SYSEX_MESSAGE_START_BYTE {
            return Some(0);
        } else {
            return None;
        }
    })
}

fn find_sysex_message_end(data: &[u8]) -> Option<usize> {
    for (index, &byte) in data.iter().enumerate() {
        if byte == SYSEX_MESSAGE_END_BYTE {
            return Some(index);
        }
    }
    return None;
}