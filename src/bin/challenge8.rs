use cryptopals::encodings;
use std::collections::HashSet;
use std::fs;

fn main() {
    match fs::read_to_string("resources/challenge8")
        .unwrap()
        .split("\n")
        .map(|string| (string, bytes_to_u128(encodings::hex_as_bytes(string))))
        .map(|(string, blocks)| {
            let mut s = HashSet::new();
            blocks.iter().for_each(|block| {
                s.insert(*block);
            });
            (string, blocks.len() - s.len())
        })
        .max_by(|(_, duplicates1), (_, duplicates2)| duplicates1.partial_cmp(duplicates2).unwrap())
    {
        None => {}
        Some((string, score)) => println!("{} , score = {}", string, score),
    };
}

fn bytes_to_u128(bytes: Vec<u8>) -> Vec<u128> {
    bytes
        .chunks_exact(16)
        .map(|chunk| {
            chunk
                .iter()
                .fold(0 as u128, |acc, byte| (acc << 8) | (*byte as u128))
        })
        .collect()
}
