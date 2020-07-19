use crate::hexaa;
use crate::strings;
use log::debug;

pub fn repeated_key_xor(hex: &str, key: &str) -> String {
    let bytes: Vec<u8> = hex.as_bytes().to_vec();
    let key: Vec<u8> = key.as_bytes().to_vec();
    let mut repeated_key = Vec::new();
    for i in 0..bytes.len() {
        repeated_key.push(key[i % key.len()]);
    }
    let encrypted = hexaa::xor_bytes(&bytes, &repeated_key);
    let x = encrypted.iter().map(|byte| hexaa::split_hex(byte));
    match String::from_utf8(
        x.flat_map(|bytes| bytes.to_be_bytes().to_vec())
            .map(|byte| strings::hex_as_ascii(&byte))
            .collect(),
    ) {
        Ok(s) => s,
        Err(e) => {
            debug!("Error encrypting {} : {:?}", hex, e);
            String::new()
        }
    }
}
