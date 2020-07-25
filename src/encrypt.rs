use crate::hexaa;
use crate::strings;

pub fn repeated_key_xor(hex: &str, key: &str) -> String {
    let bytes: Vec<u8> = hex.as_bytes().to_vec();
    let key: Vec<u8> = key.as_bytes().to_vec();
    let mut repeated_key = Vec::new();
    for i in 0..bytes.len() {
        repeated_key.push(key[i % key.len()]);
    }
    let encrypted = hexaa::xor_bytes(&bytes, &repeated_key);
    strings::bytes_to_hex(&encrypted)
}
