use crate::hex_to_base64;
use log::debug;
use std::string::FromUtf8Error;

pub fn xor_string_byte(hex1: &str, charr: u8) -> Result<String, FromUtf8Error> {
    let bytes1: Vec<u8> = hex1
        .as_bytes()
        .iter()
        .map(|byte| hex_to_base64::ascii_to_hex(*byte))
        .collect();
    let bytes1 = (0..bytes1.len())
        .step_by(2)
        .map(|index| (bytes1[index] << 4) + bytes1[index + 1])
        .collect();
    debug!("Xoring hex {:?} ", bytes1);
    // repeat `charr` the length of `hex1` times
    let bytes2: Vec<u8> = hex1.as_bytes().iter().map(|_| charr).collect();
    debug!("with hex {:?} ", bytes2);
    let result = xor_bytes(bytes1, bytes2);
    debug!("to produce {:?}", result);
    String::from_utf8(result)
}

pub fn xor_strings(hex1: &str, hex2: &str) -> String {
    assert!(hex1.len() == hex2.len());
    let bytes1: Vec<u8> = hex1
        .as_bytes()
        .iter()
        .map(|byte| hex_to_base64::ascii_to_hex(*byte))
        .collect();
    debug!("Xoring hex {:?} ", bytes1);
    let bytes2: Vec<u8> = hex2
        .as_bytes()
        .iter()
        .map(|byte| hex_to_base64::ascii_to_hex(*byte))
        .collect();
    debug!("with hex {:?} ", bytes2);
    let result = xor_bytes(bytes1, bytes2).iter().map(hex_to_ascii).collect();
    debug!("to produce {:?}", result);
    String::from_utf8(result).unwrap()
}

fn xor_bytes(b1: Vec<u8>, b2: Vec<u8>) -> Vec<u8> {
    b1.iter().zip(b2.iter()).map(|(b1, b2)| b1 ^ b2).collect()
}

fn hex_to_ascii(h: &u8) -> u8 {
    if *h < 10 {
        // [0-9]
        *h + 48
    } else {
        // [a-f]
        *h + 87
    }
}
