use log::debug;

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
/// assert_eq!(cryptopals::strings::hex_string_to_base64_string(h), "oQ==".to_owned());
/// ```
/// ```
/// let h = "a110"; // 2 bytes provided; pads by 1 '=' char
/// assert_eq!(cryptopals::strings::hex_string_to_base64_string(h), "oRA=".to_owned());
/// ```
/// ```
/// let h = "a11012"; // 3 bytes provided; no padding needed
/// assert_eq!(cryptopals::strings::hex_string_to_base64_string(h), "oRAS".to_owned());
/// ```
/// ```should_panic
/// let h = "a11"; // incomplete number of bytes
/// cryptopals::strings::hex_string_to_base64_string(h);
/// ```
pub fn hex_string_to_base64_string(hex: &str) -> String {
    assert!(hex.len() & 1 == 0);
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

/// Converts an ASCII representation of a hex string,
/// into a Vector of corresponding bytes.
/// Input string should be a valid hex, with even number of digits.
/// ```
/// assert_eq!(
///     cryptopals::strings::hex_string_as_bytes("ad1f"),
///     vec![0b_1010_1101, 0b_0001_1111]);
/// ```
/// ```should_panic
/// assert_eq!(
///     cryptopals::strings::hex_string_as_bytes("ad1d1"),
///     vec![0b_1010_1101, 0b_0001_1111]);
/// ```
pub fn hex_string_as_bytes(hex: &str) -> Vec<u8> {
    assert_eq!(
        hex.len() & 1,
        0,
        "Invalid hex string `{}`: Even number of digits expected.",
        hex
    );
    hex.as_bytes()
        .chunks(2)
        .map(|byte| ascii_to_hex(byte[0], byte[1]))
        .collect()
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

/// Converts bytes representing hexadecimal digits to their ASCII representations.
/// Numbers 0 to 9 are converted into numbers 48 (ASCII for '0') to 57 (ASCII for '9').
/// Numbers 10 (hex a) to 15 (hex f) are converted into numbers 97 (ASCII for 'a') to
/// 102 (ASCII for 'f')
///
/// ```
/// assert_eq!(cryptopals::strings::hex_as_ascii(&0), 48);
/// assert_eq!(cryptopals::strings::hex_as_ascii(&1), 49);
/// assert_eq!(cryptopals::strings::hex_as_ascii(&9), 57);
/// assert_eq!(cryptopals::strings::hex_as_ascii(&10), 97);
/// assert_eq!(cryptopals::strings::hex_as_ascii(&15), 102);
/// ```
/// ```should_panic
/// cryptopals::strings::hex_as_ascii(&16);
///```
pub fn hex_as_ascii(h: &u8) -> u8 {
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
    vec![
        base64_table(first_digit),
        base64_table(second_digit),
        base64_table(third_digit),
        base64_table(fourth_digit),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_base64_should_work() {
        let h = "a11012";
        assert_eq!(hex_string_to_base64_string(h), "oRAS");
    }

    #[test]
    fn hex_to_base64_should_pad_with_one_additional_zero() {
        let h = "a110";
        assert_eq!(hex_string_to_base64_string(h), "oRA=");
    }

    #[test]
    fn hex_to_base64_should_pad_with_one_additional_zero_for_longer_hex() {
        let h = "f10a11bdef";
        assert_eq!(hex_string_to_base64_string(h), "8QoRve8=");
    }

    #[test]
    fn hex_to_base64_should_pad_with_two_additional_zeroes() {
        let h = "1f";
        assert_eq!(hex_string_to_base64_string(h), "Hw==");
    }
}
