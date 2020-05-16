use cryptopals::decrypt;
use std::fs;

fn main() {
    env_logger::init();

    let input = fs::read_to_string("resources/challenge4").unwrap();

    input
        .split("\n")
        .map(|s| (s, decrypt::single_char_xor(&s)))
        .filter(|(_, target)| target.is_some())
        .for_each(|(source, target)| println!("{} => {}", source, target.unwrap()));
}
