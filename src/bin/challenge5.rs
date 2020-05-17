use cryptopals::encrypt;

fn main() {
    env_logger::init();
    let text = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";
    println!("Encrypted : {}", encrypt::repeated_key_xor(text, key));
}
