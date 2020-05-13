use cryptopals::decrypt;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", decrypt::single_char_xor(&args[1]));
}