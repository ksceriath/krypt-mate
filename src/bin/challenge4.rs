use cryptopals::decrypt;
use std::fs;

fn main() {
    env_logger::init();

    let input = fs::read_to_string("resources/challenge4").unwrap();

    let i: &Vec<&str> = &input.split("\n").collect();
    let result = decrypt::single_char_xor(i);
    println!("Result: {:?}", result);
}
