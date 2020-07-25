use cryptopals::strings;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!(
        "{:?}",
        args[1..]
            .iter()
            .map(|arg| strings::hex_to_b64(arg))
            .collect::<Vec<String>>()
    );
}
