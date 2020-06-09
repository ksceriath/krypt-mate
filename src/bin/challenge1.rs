use cryptopals::hexaa;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!(
        "{:?}",
        args[1..]
            .iter()
            .map(|arg| hexaa::convert(arg))
            .collect::<Vec<String>>()
    );
}
