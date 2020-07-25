use cryptopals::strings;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", strings::xor_hexes(&args[1], &args[2]));
}
