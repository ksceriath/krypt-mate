use crate::strings::ascii_to_hex;
use crate::strings::hex_as_ascii;
use crate::strings::hex_string_as_bytes;
use log::debug;

/// XORs the ASCII-representation of hex string,
/// with repeated instance of the provided character-byte.
/// ```
/// assert_eq!(
///     cryptopals::hexaa::xor_string_byte("abd10792", 0x1d),
///     vec![0xab ^ 0x1d, 0xd1 ^ 0x1d, 0x07 ^ 0x1d, 0x92 ^ 0x1d]);
/// ```
/// ```should_panic
/// cryptopals::hexaa::xor_string_byte("abd1079", 0x1d);
/// ```
// TODO This can work with just an array of bytes &[u8], and doesn't need a &str.
pub fn xor_string_byte(hex: &str, charr: u8) -> Vec<u8> {
    let bytes1 = hex_string_as_bytes(hex);
    debug!("Xoring hex {:?} ", bytes1);
    // repeat `charr` the length of `hex1` times
    let bytes2: Vec<u8> = hex.as_bytes().iter().map(|_| charr).collect();
    debug!("with hex {:?} ", bytes2);
    xor_bytes(&bytes1, &bytes2)
}

/// XORs the input hex strings together.
/// Input strings must be ASCII representation of hexadecimal numbers.
/// Strings can contain letters [a-f] and numbers [0-9].
/// Strings must be of equal lengths, automatic padding is not done.
/// ```
/// assert_eq!(cryptopals::hexaa::xor_strings(
///         "0259acef",
///         "bd134678"),
///     "bf4aea97");
/// ```
/// ```should_panic
/// cryptopals::hexaa::xor_strings("02", "bd1"); // length constraint
/// ```
/// ```should_panic
/// cryptopals::hexaa::xor_strings("02g", "bd1"); // invalid hex digit
/// ```
pub fn xor_strings(hex1: &str, hex2: &str) -> String {
    assert!(hex1.len() == hex2.len());
    let bytes1: Vec<u8> = hex1
        .as_bytes()
        .iter()
        .map(|byte| ascii_to_hex('0' as u8, *byte))
        .collect();
    debug!("Xoring hex {:?} ", bytes1);
    let bytes2: Vec<u8> = hex2
        .as_bytes()
        .iter()
        .map(|byte| ascii_to_hex('0' as u8, *byte))
        .collect();
    debug!("with hex {:?} ", bytes2);
    let result = xor_bytes(&bytes1, &bytes2)
        .iter()
        .map(hex_as_ascii)
        .collect();
    debug!("to produce {:?}", result);
    String::from_utf8(result).unwrap()
}

/// XORs the bytes in the input vectors sequentially.
/// ```
/// assert_eq!(
///     cryptopals::hexaa::xor_bytes(
///         &vec![1, 3, 7, 15],
///         &vec![31, 63, 127, 255]),
///     vec![1^31, 3^63, 7^127, 15^255]);
/// ```
pub fn xor_bytes(b1: &Vec<u8>, b2: &Vec<u8>) -> Vec<u8> {
    debug!("xoring {:?}", b1);
    debug!("with   {:?}", b2);
    b1.iter().zip(b2.iter()).map(|(b1, b2)| b1 ^ b2).collect()
}

/// Splits a byte (representing 2 hex digits) into two,
/// one for each digit, padding the additional 4 most significant bits with zeros.
/// ```
/// assert_eq!(cryptopals::hexaa::split_hex(&0b1010_0101), 0b00001010_00000101);
/// ```
pub fn split_hex(h: &u8) -> u16 {
    (((h & 0xf0) as u16) << 4) + (h & 0x0f) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_to_hex_happy_path() {
        assert_eq!(ascii_to_hex('9' as u8, 'a' as u8), 0b10011010);
        assert_eq!(ascii_to_hex('a' as u8, 'f' as u8), 0b10101111);
        assert_eq!(ascii_to_hex('f' as u8, '0' as u8), 0b11110000);
        assert_eq!(ascii_to_hex('0' as u8, '9' as u8), 0b00001001);
    }

    #[test]
    #[should_panic]
    fn ascii_to_hex_should_panic_for_uppercase_letters() {
        ascii_to_hex('0' as u8, 'A' as u8);
    }

    #[test]
    #[should_panic]
    fn ascii_to_hex_should_panic_for_out_of_bounds_letters() {
        ascii_to_hex('0' as u8, 'z' as u8);
    }
}
