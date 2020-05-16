use cryptopals::decrypt;
use std::fs;

fn main() {
    env_logger::init();

    let input = fs::read_to_string("resources/challenge4").unwrap();

    input.split("\n").map(|s| {  decrypt::single_char_xor(&s)}).filter(|s| s.is_some()).for_each(|s| println!("{}", s.unwrap()));
}