use cryptopals::encodings;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", encodings::xor_hexes(&args[1], &args[2]));
}
