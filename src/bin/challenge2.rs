use cryptopals::hexaa;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", hexaa::xor_strings(&args[1], &args[2]));
}
