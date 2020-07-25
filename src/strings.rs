use crate::hexaa;
use log::debug;

/// For the purposes of readability, note the following definitions.
/// hex : means the string form of a hexadecimal number. E.g. "0259acef". (It does not represent the underlying hexadecimal bytes, nor the ASCII bytes)
/// b64 : means the string form of a base-64 number. E.g. "8QoRve8=". (Again, it does not represent the underlying binary format, nor the ASCII bytes)
/// hex_ascii : means the ASCII bytes corresponding to a hex string.
/// b64_ascii : means the ASCII bytes corresponding to a b84 string.
/// byte : anywhere refers to binary. Its represented as a u8, or a Vec<u8>.
/// So, a typical conversion is like: hex/b64 (string) ===> hex_ascii/b64_ascii (Vec<u8>) ===> bytes (Vec<u8>)

/// Padder provides an implementation of pad_b64 function for Vec<u8>
/// pad_b64 provides a fluent way of padding a Vec representing a base64 string
/// with sufficient number of pad characters ('=')
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

/// XORs the input hex strings together.
/// Input strings must be ASCII representation of hexadecimal numbers.
/// Strings can contain letters [a-f] and numbers [0-9].
/// Strings must be of equal lengths, automatic padding is not done.
/// ```
/// assert_eq!(cryptopals::strings::xor_hexes(
///         "0259acef",
///         "bd134678"),
///     "bf4aea97");
/// ```
/// ```should_panic
/// cryptopals::strings::xor_hexes("02", "bd1"); // length constraint
/// ```
/// ```should_panic
/// cryptopals::strings::xor_hexes("02g", "bd1"); // invalid hex digit
/// ```
pub fn xor_hexes(hex1: &str, hex2: &str) -> String {
    assert!(hex1.len() == hex2.len());
    let bytes1: Vec<u8> = hex_as_bytes(hex1);
    debug!("Xoring hex {:?} ", bytes1);
    let bytes2: Vec<u8> = hex_as_bytes(hex2);
    debug!("with hex {:?} ", bytes2);
    let result = hexaa::xor_bytes(&bytes1, &bytes2);
    bytes_to_hex(&result)
}

/// Convert a vector of bytes to a hexadecimal ASCII string representation
pub fn bytes_to_hex(bytes: &Vec<u8>) -> String {
    let ascii = bytes
        .iter()
        .flat_map(|item| byte_as_hex_ascii(item))
        .collect();
    String::from_utf8(ascii).unwrap()
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
/// assert_eq!(cryptopals::strings::hex_to_b64(h), "oQ==".to_owned());
/// ```
/// ```
/// let h = "a110"; // 2 bytes provided; pads by 1 '=' char
/// assert_eq!(cryptopals::strings::hex_to_b64(h), "oRA=".to_owned());
/// ```
/// ```
/// let h = "a11012"; // 3 bytes provided; no padding needed
/// assert_eq!(cryptopals::strings::hex_to_b64(h), "oRAS".to_owned());
/// ```
/// ```should_panic
/// let h = "a11"; // incomplete number of bytes
/// cryptopals::strings::hex_to_b64(h);
/// ```
pub fn hex_to_b64(hex: &str) -> String {
    assert!(hex.len() & 1 == 0);

    bytes_to_b64(hex_as_bytes(hex))
}

/// Converts an ASCII representation of a hex string,
/// into a Vector of corresponding bytes.
/// Input string should be a valid hex, with even number of digits.
/// ```
/// assert_eq!(
///     cryptopals::strings::hex_as_bytes("ad1f"),
///     vec![0b_1010_1101, 0b_0001_1111]);
/// ```
/// ```should_panic
/// assert_eq!(
///     cryptopals::strings::hex_as_bytes("ad1d1"),
///     vec![0b_1010_1101, 0b_0001_1111]);
/// ```
pub fn hex_as_bytes(hex: &str) -> Vec<u8> {
    assert_eq!(
        hex.len() & 1,
        0,
        "Invalid hex string `{}`: Even number of digits expected.",
        hex
    );
    hex.as_bytes()
        .chunks(2)
        .map(|byte| hex_ascii_to_byte(byte[0], byte[1]))
        .collect()
}

/// Converts an ASCII representation of 2 hexadecimal digits
/// into binary equivalent (1 byte).
/// Allowed hexadecimal digits : [0-9a-f]
/// Panics if input does not correspond to the ASCII values of above characters
fn hex_ascii_to_byte(s: u8, t: u8) -> u8 {
    (if s >= 48 && s <= 57 {
        // [0-9]
        s - 48
    } else if s >= 97 && s <= 102 {
        // [a-f]
        s - 97 + 10
    } else {
        panic!("ASCII for hex should be a letter [a-f], or a digit [0-9].")
    } << 4)
        + (if t >= 48 && t <= 57 {
            // [0-9]
            t - 48
        } else if t >= 97 && t <= 102 {
            // [a-f]
            t - 97 + 10
        } else {
            panic!("ASCII for hex should be a letter [a-f], or a digit [0-9].")
        })
}

fn byte_as_hex_ascii(h: &u8) -> Vec<u8> {
    vec![
        byte_as_partial_hex_ascii(&(h >> 4)),
        byte_as_partial_hex_ascii(&(h & 0x0f)),
    ]
}

/// Converts bytes representing hexadecimal digits to their ASCII representations.
/// Numbers 0 to 9 are converted into numbers 48 (ASCII for '0') to 57 (ASCII for '9').
/// Numbers 10 (hex a) to 15 (hex f) are converted into numbers 97 (ASCII for 'a') to
/// 102 (ASCII for 'f')
fn byte_as_partial_hex_ascii(h: &u8) -> u8 {
    if *h < 10 {
        // [0-9]
        *h + 48
    } else if *h < 16 {
        // [a-f]
        *h + 87
    } else {
        panic!("{} beyond the range of hexadecimal digits", h);
    }
}

fn bytes_to_b64(bytes: Vec<u8>) -> String {
    let new_bytes = bytes
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

/// Converts a base64 digit into corresponding string representation (ASCII)
/// ASCII representation digits consist of [A-Za-z0-9+/]
/// Panics if the input does not make a base64 digit, i.e. input > 63
fn byte_to_b64_ascii(i: u8) -> u8 {
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
    vec![
        byte_to_b64_ascii(first_digit),
        byte_to_b64_ascii(second_digit),
        byte_to_b64_ascii(third_digit),
        byte_to_b64_ascii(fourth_digit),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_base64_should_work() {
        let h = "a11012";
        assert_eq!(hex_to_b64(h), "oRAS");
    }

    #[test]
    fn hex_to_base64_should_pad_with_one_additional_zero() {
        let h = "a110";
        assert_eq!(hex_to_b64(h), "oRA=");
    }

    #[test]
    fn hex_to_base64_should_pad_with_one_additional_zero_for_longer_hex() {
        let h = "f10a11bdef";
        assert_eq!(hex_to_b64(h), "8QoRve8=");
    }

    #[test]
    fn hex_to_base64_should_pad_with_two_additional_zeroes() {
        let h = "1f";
        assert_eq!(hex_to_b64(h), "Hw==");
    }

    #[test]
    fn ascii_to_hex_happy_path() {
        assert_eq!(hex_ascii_to_byte('9' as u8, 'a' as u8), 0b10011010);
        assert_eq!(hex_ascii_to_byte('a' as u8, 'f' as u8), 0b10101111);
        assert_eq!(hex_ascii_to_byte('f' as u8, '0' as u8), 0b11110000);
        assert_eq!(hex_ascii_to_byte('0' as u8, '9' as u8), 0b00001001);
    }

    #[test]
    #[should_panic]
    fn ascii_to_hex_should_panic_for_uppercase_letters() {
        hex_ascii_to_byte('0' as u8, 'A' as u8);
    }

    #[test]
    #[should_panic]
    fn ascii_to_hex_should_panic_for_out_of_bounds_letters() {
        hex_ascii_to_byte('0' as u8, 'z' as u8);
    }

    #[test]
    fn byte_as_partial_hex_ascii_should_convert_as_expected() {
        assert_eq!(byte_as_partial_hex_ascii(&0), 48);
        assert_eq!(byte_as_partial_hex_ascii(&1), 49);
        assert_eq!(byte_as_partial_hex_ascii(&9), 57);
        assert_eq!(byte_as_partial_hex_ascii(&10), 97);
        assert_eq!(byte_as_partial_hex_ascii(&15), 102);
    }

    #[test]
    #[should_panic]
    fn byte_as_partial_hex_ascii_should_panic_for_out_of_bounds_bytes() {
        byte_as_partial_hex_ascii(&16);
    }
}
