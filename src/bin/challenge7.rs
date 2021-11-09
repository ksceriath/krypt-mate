use cryptopals::aes128::decrypt::decrypt;
use cryptopals::encodings;
use std::fs;

fn main() {
    let b64_decoded: Vec<u8> = fs::read_to_string("resources/challenge7")
        .unwrap()
        .split("\n")
        .flat_map(encodings::b64_as_bytes)
        .collect();

    let key = "YELLOW SUBMARINE";
    let key = bytes_to_u128(key.as_bytes());

    println!(
        "{}",
        b64_decoded
            .chunks_exact(16)
            .map(bytes_to_u128)
            .map(|block| decrypt(block, key))
            .map(|s| String::from_utf8(u128_to_bytes(s)).unwrap())
            .collect::<String>()
    );
}

fn bytes_to_u128(bytes: &[u8]) -> u128 {
    bytes
        .into_iter()
        .fold(0, |acc, byte| (acc << 8) | (*byte as u128))
}

fn u128_to_bytes(block: u128) -> Vec<u8> {
    (0..16)
        .rev()
        .map(|position| (block >> (8 * position)) as u8)
        .collect()
}
