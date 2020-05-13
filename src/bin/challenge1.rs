use cryptopals::hex_to_base64;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!(
        "{:?}",
        args[1..]
            .iter()
            .map(|arg| hex_to_base64::convert(arg))
            .collect::<Vec<String>>()
    );
}
