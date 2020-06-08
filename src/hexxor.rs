use crate::hex_to_base64;
use log::debug;
use std::string::FromUtf8Error;

pub fn xor_string_byte(hex: &str, charr: u8) -> Result<String, FromUtf8Error> {
    let bytes1 = hex_bytes(hex);
    debug!("Xoring hex {:?} ", bytes1);
    // repeat `charr` the length of `hex1` times
    let bytes2: Vec<u8> = hex.as_bytes().iter().map(|_| charr).collect();
    debug!("with hex {:?} ", bytes2);
    let result = xor_bytes(bytes1, bytes2);
    debug!("to produce {:?}", result);
    String::from_utf8(result)
}

pub fn hex_bytes(hex: &str) -> Vec<u8> {
    let bytes1: Vec<u8> = hex
        .as_bytes()
        .iter()
        .map(|byte| hex_to_base64::ascii_to_hex(*byte))
        .collect();
    (0..bytes1.len())
        .step_by(2)
        .map(|index| (bytes1[index] << 4) + bytes1[index + 1])
        .collect()
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
    let result = xor_bytes(bytes1, bytes2).iter().map(hex_string).collect();
    debug!("to produce {:?}", result);
    String::from_utf8(result).unwrap()
}

pub fn xor_bytes(b1: Vec<u8>, b2: Vec<u8>) -> Vec<u8> {
    debug!("xoring {:?}", b1);
    debug!("with   {:?}", b2);
    b1.iter().zip(b2.iter()).map(|(b1, b2)| b1 ^ b2).collect()
}

// Converts bytes representing hexadecimal digits to their ascii counterparts
// The bytes from
// 00000000 => 00110000 (= 48, ascii for character '0')
// 00000001 => 00110001 (= 49, ascii for character '1')
// 00001001 => 00111001 (= 57, ascii for character '9')
// 00001010 => 01100001 (= 97, ascii for character 'a')
// 00001111 => 01100110 (= 102, ascii for character 'f')
//
// Other input numbers beyond hexadecimal range are converted as well,
// without erroring, but they are not the purpose of this function.
pub fn hex_string(h: &u8) -> u8 {
    if *h < 10 {
        // [0-9]
        *h + 48
    } else {
        // [a-f]
        *h + 87
    }
}

pub fn split_hex(h: &u8) -> u16 {
    (((h & 0xf0) as u16) << 4) + (h & 0x0f) as u16
}
