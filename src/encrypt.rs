use crate::encodings;
use crate::hexaa;

pub fn repeated_key_xor(hex: &str, key: &str) -> String {
    let bytes: Vec<u8> = hex.as_bytes().to_vec();
    let key: Vec<u8> = key.as_bytes().to_vec();
    let repeated_key = bytes
        .chunks(key.len())
        .flat_map(|chunk| chunk.iter().zip(key.clone()).map(|(_, key_byte)| key_byte))
        .collect();
    let encrypted = hexaa::xor_bytes(&bytes, &repeated_key);
    encodings::bytes_to_hex(&encrypted)
}
