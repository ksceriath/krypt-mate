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
///     cryptopals::hexaa::hex_bytes("ad1f"),
///     vec![0b_1010_1101, 0b_0001_1111]);
/// ```
/// ```should_panic
/// assert_eq!(
///     cryptopals::hexaa::hex_bytes("ad1d1"),
///     vec![0b_1010_1101, 0b_0001_1111]);
/// ```
pub fn hex_bytes(hex: &str) -> Vec<u8> {
    assert_eq!(
        hex.len() & 1,
        0,
        "Invalid hex string `{}`: Even number of digits expected.",
        hex
    );
    hex
        .as_bytes()
        .chunks(2)
        .map(|byte| ascii_to_hex(byte[0], byte[1]))
        .collect()
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
    let result = xor_bytes(&bytes1, &bytes2).iter().map(hex_string).collect();
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

/// Converts bytes representing hexadecimal digits to their ASCII representations.
/// Numbers 0 to 9 are converted into numbers 48 (ASCII for '0') to 57 (ASCII for '9').
/// Numbers 10 (hex a) to 15 (hex f) are converted into numbers 97 (ASCII for 'a') to
/// 102 (ASCII for 'f')
///
/// ```
/// assert_eq!(cryptopals::hexaa::hex_string(&0), 48);
/// assert_eq!(cryptopals::hexaa::hex_string(&1), 49);
/// assert_eq!(cryptopals::hexaa::hex_string(&9), 57);
/// assert_eq!(cryptopals::hexaa::hex_string(&10), 97);
/// assert_eq!(cryptopals::hexaa::hex_string(&15), 102);
/// ```
/// ```should_panic
/// cryptopals::hexaa::hex_string(&16);
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
/// assert_eq!(cryptopals::hexaa::split_hex(&0b1010_0101), 0b00001010_00000101);
/// ```
pub fn split_hex(h: &u8) -> u16 {
    (((h & 0xf0) as u16) << 4) + (h & 0x0f) as u16
}

/// Converts a string representation (ASCII) of a hexadecimal number
/// into a string representation of its base64 representation
///
/// A hexadecimal digit is represented by numbers [0-9] and letters [a-f] (in order)
/// in string form, and corresponds to four bits.
/// A base64 digit is represented by letters [A-Za-z], numbers [0-9] and symbols [+, /]
/// (in order) in string form, and corresponds to six bits.
///
/// Correspondingly, every 3 hexadecimal digits are eligible to be converted into
/// 2 base64 digits. However, 3 hexadecimal digits do not stand for full bytes,
/// and hence we convert 6 hexadecimal digits (= 3 bytes) into 4 base64 digits.
/// Following implementation pads the hexadecimal number with additional zeroes
/// to get 3*n bytes, however an even number of digits are still expected.
/// The 0s are padded to the right and appear as '=' in final representation.
/// ```
/// let h = "a1"; // 1 byte provided; pads by 2 '=' char
/// assert_eq!(cryptopals::hexaa::hex_to_base64(h), "oQ==".to_owned());
/// ```
/// ```
/// let h = "a110"; // 2 bytes provided; pads by 1 '=' char
/// assert_eq!(cryptopals::hexaa::hex_to_base64(h), "oRA=".to_owned());
/// ```
/// ```
/// let h = "a11012"; // 3 bytes provided; no padding needed
/// assert_eq!(cryptopals::hexaa::hex_to_base64(h), "oRAS".to_owned());
/// ```
/// ```should_panic
/// let h = "a11"; // incomplete number of bytes
/// cryptopals::hexaa::hex_to_base64(h);
/// ```
pub fn hex_to_base64(hex: &str) -> String {
    assert!(hex.len()&1 == 0);
    let new_bytes: Vec<u8> = hex
        .as_bytes()
        .chunks(2)
        .map(|v| ascii_to_hex(v[0], v[1]))
        .collect::<Vec<u8>>()
        .chunks(3)
        .flat_map(|v| match v.len() {
            1 => hex_triad_to_base64_quad(v[0], 0, 0).pad_b64(2),
            2 => hex_triad_to_base64_quad(v[0], v[1], 0).pad_b64(1),
            3 => hex_triad_to_base64_quad(v[0], v[1], v[2]),
            _ => panic!("Unexpected: Chunk should have a maximum length of 3."),
        })
        .collect();
    String::from_utf8(new_bytes).unwrap()
}

trait Padder {
    fn pad_b64(self, count: usize) -> Self;
 }

impl Padder for Vec<u8> {
    fn pad_b64(mut self, count: usize) -> Self {
        let pad = '=' as u8;
        let len = self.len();
        for i in (len - count)..len {
            self[i] = pad;
        }
        self
    }
}

/// Converts an ASCII representation of a hexadecimal digit
/// into its integer equivalent.
/// Allowed hexadecimal digits : [0-9a-f]
/// Panics if input does not correspond to the ASCII values of above characters
pub fn ascii_to_hex(s: u8, t: u8) -> u8 {
    (if s >= 48 && s <= 57 {
        // [0-9]
        s - 48
    } else if s >= 97 && s <= 102 {
        // [a-f]
        s - 97 + 10
    } else {
        panic!("ASCII for hex should be a letter [a-f], or a digit [0-9].")
    } << 4) + (if t >= 48 && t <= 57 {
        // [0-9]
        t - 48
    } else if t >= 97 && t <= 102 {
        // [a-f]
        t - 97 + 10
    } else {
        panic!("ASCII for hex should be a letter [a-f], or a digit [0-9].")
    })
}

/// Converts a base64 digit into corresponding string representation (ASCII)
/// ASCII representation digits consist of [A-Za-z0-9+/]
/// Panics if the input does not make a base64 digit, i.e. input > 63
fn base64_table(i: u8) -> u8 {
    if i < 26 {
        // [A-Z]
        i + 65
    } else if i < 52 {
        // [a-z]
        i + 71
    } else if i < 62 {
        // [0-9]
        i - 52 + 48
    } else if i == 62 {
        // (+)
        43
    } else if i == 63 {
        // (/)
        47
    } else {
        // Not a base64 digit : throw error ?
        panic!("Base64 digits should be numbers less than 64.");
    }
}

/// Converts three hexadecimal digits into two base64 digits
/// Inputs are three hexadecimal bytes
/// Output is a Vector of two ASCII-bytes of base64 digits
fn hex_triad_to_base64_quad(a: u8, b: u8, c: u8) -> Vec<u8> {
    let first_digit = a >> 2;
    let second_digit = ((a << 4) + (b >> 4)) & 0x3f;
    let third_digit = ((b << 2) + (c >> 6)) & 0x3f;
    let fourth_digit = c & 0x3f;
    debug!("First 6 bits of {} : {}", a, first_digit);
    debug!("2 bits of {} and 4 bits of {} : {}", a, b, second_digit);
    debug!("4 bits of {} and 2 bits of {} : {}", b, c, third_digit);
    debug!("Last 6 bits of {} : {}", c, fourth_digit);
    vec![base64_table(first_digit), base64_table(second_digit), base64_table(third_digit), base64_table(fourth_digit)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_base64_should_work() {
        let h = "a11012";
        assert_eq!(hex_to_base64(h), "oRAS");
    }

    #[test]
    fn hex_to_base64_should_pad_with_one_additional_zero() {
        let h = "a110";
        assert_eq!(hex_to_base64(h), "oRA=");
    }

    #[test]
    fn hex_to_base64_should_pad_with_one_additional_zero_for_longer_hex() {
        let h = "f10a11bdef";
        assert_eq!(hex_to_base64(h), "8QoRve8=");
    }

    #[test]
    fn hex_to_base64_should_pad_with_two_additional_zeroes() {
        let h = "1f";
        assert_eq!(hex_to_base64(h), "Hw==");
    }

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
