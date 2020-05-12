use crate::hex_to_base64;

pub fn xor(hex1: &str, hex2: &str) -> String {
    assert!(hex1.len() == hex2.len());
    let bytes1: Vec<u8> = hex1
        .as_bytes()
        .iter()
        .map(|byte| hex_to_base64::ascii_to_hex(*byte))
        .collect();
    print!("Xoring hex {:?} ", bytes1);
    let bytes2: Vec<u8> = hex2
        .as_bytes()
        .iter()
        .map(|byte| hex_to_base64::ascii_to_hex(*byte))
        .collect();
    print!("with hex {:?} ", bytes2);
    let result = bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(b1, b2)| hex_to_ascii(b1 ^ b2))
        .collect();
    println!("to produce {:?}", result);
    String::from_utf8(result).unwrap()
}

fn hex_to_ascii(h: u8) -> u8 {
    if h < 10 {
        // [0-9]
        h + 48
    } else {
        // [a-f]
        h + 87
    }
}