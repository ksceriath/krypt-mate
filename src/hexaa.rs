use log::debug;

/// XORs a vector of bytes with repeated instances of another byte.
/// ```
/// assert_eq!(
///     cryptopals::hexaa::repeated_byte_xor(
///      &vec![0xab, 0xd1, 0x07, 0x92],
///       0x1d),
///     vec![0xab ^ 0x1d, 0xd1 ^ 0x1d, 0x07 ^ 0x1d, 0x92 ^ 0x1d]);
/// ```
pub fn repeated_byte_xor(bytes1: &Vec<u8>, charr: u8) -> Vec<u8> {
    debug!("Xoring hex {:?} ", bytes1);
    // repeat `charr` the length of `hex1` times
    let bytes2: Vec<u8> = bytes1.iter().map(|_| charr).collect();
    debug!("with hex {:?} ", bytes2);
    xor_bytes(&bytes1, &bytes2)
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

pub fn count_set_bits_in_bytes(bytes: Vec<u8>) -> u32 {
    bytes.iter().map(|byte| count_set_bits(*byte)).sum()
}

pub fn count_set_bits(byte: u8) -> u32 {
    if byte == 0 {
        0
    } else {
        1 + count_set_bits(byte & (byte - 1))
    }
}
