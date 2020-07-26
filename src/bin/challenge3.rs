use cryptopals::decrypt;
use std::fs;

fn main() {
    let input = fs::read_to_string("resources/challenge3").unwrap();

    println!("{:?}", decrypt::single_char_xor(&[&input]));
}
