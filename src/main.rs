use cryptopals::hexxor;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", hexxor::xor(&args[1], &args[2]));
}
