use cryptopals::hexaa;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!(
        "{:?}",
        args[1..]
            .iter()
            .map(|arg| hexaa::hex_string_to_base64_string(arg))
            .collect::<Vec<String>>()
    );
}
