use log::debug;

/// Converts a string representation (ASCII) of a hexadecimal number
/// into a string representation of its base64 representation
///
/// A hexadecimal digit is represented by numbers [0-9] and letters [a-f] (in order)
/// in string form, and corresponds to four bits.
/// A base64 digit is represented by letters [A-Za-z], numbers [0-9] and symbols [+, /]
/// (in order) in string form, and corresponds to six bits.
///
/// Correspondingly, every 3 hexadecimal digits are eligible to be converted into
/// 2 base64 digits.
/// Following implementation pads the hexadecimal number with additional zeroes
/// to get 3*n digits.
///
/// ```
/// let h = "a11";
/// assert_eq!(cryptopals::hex_to_base64::convert(h), "oR".to_owned());
/// ```
pub fn convert(hex: &str) -> String {
    // debug!("Converting string: {}", hex);
    // let bytes: Vec<u8> = hex.as_bytes().iter().map(|v| ascii_to_hex(*v)).collect();
    // debug!("Converting bytes: {:?}", bytes);
    let new_bytes: Vec<u8> = hex
        .as_bytes()
        .iter()
        .map(|v| ascii_to_hex(*v))
        .collect::<Vec<u8>>()
        .rchunks(3) // rchunks to pad on the left
        .rev() // rev to restore the reversed rchunks order
        .flat_map(|v| match v.len() {
            1 => hex_triad_to_base64_diad(0, 0, v[0]),
            2 => hex_triad_to_base64_diad(0, v[0], v[1]),
            3 => hex_triad_to_base64_diad(v[0], v[1], v[2]),
            _ => panic!("Unexpected: Chunk should have a maximum length of 3."),
        })
        .collect();
    String::from_utf8(new_bytes).unwrap()
}

/// Converts an ASCII representation of a hexadecimal digit
/// into its integer equivalent.
/// Allowed hexadecimal digits : [0-9a-f]
/// Panics if input does not correspond to the ASCII values of above characters
pub fn ascii_to_hex(s: u8) -> u8 {
    if s >= 48 && s <= 57 {
        // [0-9]
        s - 48
    } else if s >= 97 && s <= 102 {
        // [a-f]
        s - 97 + 10
    } else {
        panic!("ASCII for hex should be a letter [a-f], or a digit [0-9].")
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
fn hex_triad_to_base64_diad(a: u8, b: u8, c: u8) -> Vec<u8> {
    let a_s = a << 2;
    let b_s = b >> 2;
    let b_s2 = (b & 0b11) << 4;
    debug!("Left shift {} by 2 to get {}", a, a_s);
    debug!("Right shift {} by 2 to get {}", b, b_s);
    debug!("Left shift lower two bits of {} by 4 to get {}", b, b_s2);
    let first_byte = a_s + b_s;
    let second_byte = b_s2 + c;
    debug!("First byte = {}", first_byte);
    debug!("Second byte = {}", second_byte);
    vec![base64_table(first_byte), base64_table(second_byte)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_should_work() {
        let h = "a11";
        assert_eq!(convert(h), "oR");
    }

    #[test]
    fn convert_should_pad_with_one_additional_zero() {
        let h = "a1";
        assert_eq!(convert(h), "Ch");
    }

    #[test]
    fn convert_should_pad_with_one_additional_zero_for_longer_hex() {
        let h = "f10a1";
        assert_eq!(convert(h), "DxCh");
    }

    #[test]
    fn convert_should_pad_with_two_additional_zeroes() {
        let h = "1f0d";
        assert_eq!(convert(h), "AB8N");
    }

    #[test]
    fn ascii_to_hex_happy_path() {
        assert_eq!(ascii_to_hex('a' as u8), 0b1010);
        assert_eq!(ascii_to_hex('f' as u8), 0b1111);
        assert_eq!(ascii_to_hex('0' as u8), 0b0000);
        assert_eq!(ascii_to_hex('9' as u8), 0b1001);
    }

    #[test]
    #[should_panic]
    fn ascii_to_hex_should_panic_for_uppercase_letters() {
        ascii_to_hex('A' as u8);
    }

    #[test]
    #[should_panic]
    fn ascii_to_hex_should_panic_for_out_of_bounds_letters() {
        ascii_to_hex('z' as u8);
    }
}
