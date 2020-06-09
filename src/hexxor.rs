use crate::hex_to_base64;
use log::debug;

/// XORs the ASCII-representation of hex string,
/// with repeated instance of the provided character-byte.
/// ```
/// assert_eq!(
///     cryptopals::hexxor::xor_string_byte("abd10792", 0x1d),
///     vec![0xab ^ 0x1d, 0xd1 ^ 0x1d, 0x07 ^ 0x1d, 0x92 ^ 0x1d]);
/// ```
/// ```should_panic
/// cryptopals::hexxor::xor_string_byte("abd1079", 0x1d);
/// ```
pub fn xor_string_byte(hex: &str, charr: u8) -> Vec<u8> {
    let bytes1 = hex_bytes(hex);
    debug!("Xoring hex {:?} ", bytes1);
    // repeat `charr` the length of `hex1` times
    let bytes2: Vec<u8> = hex.as_bytes().iter().map(|_| charr).collect();
    debug!("with hex {:?} ", bytes2);
    xor_bytes(&bytes1, &bytes2)
}

/// Converts an ASCII representation of a hex string,
/// into a Vector of corresponding bytes.
/// Input string should be a valid hex, with even number of digits.
/// ```
/// assert_eq!(
///     cryptopals::hexxor::hex_bytes("ad1f"),
///     vec![0b_1010_1101, 0b_0001_1111]);
/// ```
/// ```should_panic
/// assert_eq!(
///     cryptopals::hexxor::hex_bytes("ad1d1"),
///     vec![0b_1010_1101, 0b_0001_1111]);
/// ```
pub fn hex_bytes(hex: &str) -> Vec<u8> {
    assert_eq!(
        hex.len() & 1,
        0,
        "Invalid hex string `{}`: Even number of digits expected.",
        hex
    );
    let bytes1: Vec<u8> = hex
        .as_bytes()
        .iter()
        .map(|byte| hex_to_base64::ascii_to_hex(*byte))
        .collect();
    (0..bytes1.len())
        .step_by(2)
        .map(|index| (bytes1[index] << 4) + bytes1[index + 1])
        .collect()
}

/// XORs the input hex strings together.
/// Input strings must be ASCII representation of hexadecimal numbers.
/// Strings can contain lettrs [a-f] and numbers [0-9].
/// Strings must be of equal lengths, automatic padding is not done.
/// ```
/// assert_eq!(cryptopals::hexxor::xor_strings(
///         "0259acef",
///         "bd134678"),
///     "bf4aea97");
/// ```
/// ```should_panic
/// cryptopals::hexxor::xor_strings("02", "bd1"); // hex strings should be equal in length
/// ```
/// ```should_panic
/// cryptopals::hexxor::xor_strings("02g", "bd1"); // 'g' is not a hex digit
/// ```
pub fn xor_strings(hex1: &str, hex2: &str) -> String {
    assert!(hex1.len() == hex2.len());
    let bytes1: Vec<u8> = hex1
        .as_bytes()
        .iter()
        .map(|byte| hex_to_base64::ascii_to_hex(*byte))
        .collect();
    debug!("Xoring hex {:?} ", bytes1);
    let bytes2: Vec<u8> = hex2
        .as_bytes()
        .iter()
        .map(|byte| hex_to_base64::ascii_to_hex(*byte))
        .collect();
    debug!("with hex {:?} ", bytes2);
    let result = xor_bytes(&bytes1, &bytes2).iter().map(hex_string).collect();
    debug!("to produce {:?}", result);
    String::from_utf8(result).unwrap()
}

/// XORs the bytes in the input vectors sequentially.
/// ```
/// assert_eq!(
///     cryptopals::hexxor::xor_bytes(
///         &vec![1, 3, 7, 15],
///         &vec![31, 63, 127, 255]),
///     vec![1^31, 3^63, 7^127, 15^255]);
/// ```
pub fn xor_bytes(b1: &Vec<u8>, b2: &Vec<u8>) -> Vec<u8> {
    debug!("xoring {:?}", b1);
    debug!("with   {:?}", b2);
    b1.iter().zip(b2.iter()).map(|(b1, b2)| b1 ^ b2).collect()
}

/// Converts bytes representing hexadecimal digits to their ASCII representations.
/// Numbers 0 to 9 are converted into numbers 48 (ASCII for '0') to 57 (ASCII for '9').
/// Numbers 10 (hex a) to 15 (hex f) are converted into numbers 97 (ASCII for 'a') to
/// 102 (ASCII for 'f')
///
/// ```
/// assert_eq!(cryptopals::hexxor::hex_string(&0), 48);
/// assert_eq!(cryptopals::hexxor::hex_string(&1), 49);
/// assert_eq!(cryptopals::hexxor::hex_string(&9), 57);
/// assert_eq!(cryptopals::hexxor::hex_string(&10), 97);
/// assert_eq!(cryptopals::hexxor::hex_string(&15), 102);
/// ```
/// ```should_panic
/// cryptopals::hexxor::hex_string(&16);
///```
pub fn hex_string(h: &u8) -> u8 {
    if *h < 10 {
        // [0-9]
        *h + 48
    } else if *h < 16 {
        // [a-f]
        *h + 87
    } else {
        panic!("{} beyond the range of hexadecimal digits");
    }
}

/// Splits a byte (representing 2 hex digits) into two,
/// one for each digit, padding the additional 4 most significant bits with zeros.
/// ```
/// assert_eq!(cryptopals::hexxor::split_hex(&0b1010_0101), 0b00001010_00000101);
/// ```
pub fn split_hex(h: &u8) -> u16 {
    (((h & 0xf0) as u16) << 4) + (h & 0x0f) as u16
}
