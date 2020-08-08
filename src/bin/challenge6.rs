use cryptopals::decrypt;
use cryptopals::encodings;
use cryptopals::encrypt;
use std::fs;

fn main() {
    let input = fs::read_to_string("resources/challenge6").unwrap();
    let input = input.split("\n");

    let b64_decoded: Vec<u8> = input.flat_map(encodings::b64_as_bytes).collect();
    let key = decrypt::find_vignere_key(&b64_decoded);
    let decrypted_bytes = encrypt::repeated_key_xor(&b64_decoded, &key);

    println!("key = {:?}", String::from_utf8(key).unwrap());

    String::from_utf8(decrypted_bytes)
        .unwrap()
        .split("\n")
        .for_each(|s| println!("{}", s));
}
