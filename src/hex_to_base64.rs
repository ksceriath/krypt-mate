use log::debug;

pub fn convert(hex: &str) -> String {
    debug!("Converting string: {}", hex);
    let mut bytes: Vec<u8> = hex.as_bytes().iter().map(|v| ascii_to_hex(*v)).collect();
    match bytes.len() % 3 {
        1 => {
            bytes.push(0);
            bytes.push(0);
        }
        2 => {
            bytes.push(0);
        }
        _ => (),
    }
    debug!("Converting bytes: {:?}", bytes);
    let new_bytes: Vec<u8> = bytes
        .chunks(3)
        .flat_map(|v| {
            debug!("Convert bytes: {:?}", v);
            let b = hex_triad_to_base64_diad(v[0], v[1], v[2]);
            debug!("Converted bytes: {:?}", b);
            b
        })
        .collect();
    String::from_utf8(new_bytes).unwrap()
}

pub fn ascii_to_hex(s: u8) -> u8 {
    if s >= 48 && s <= 57 {
        // [0-9]
        s - 48
    } else if s >= 97 && s <= 102 {
        // [a-f]
        s - 97 + 10
    } else {
        s
    }
}

// converts base64 hex bytes to ascii bytes
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
        // Not a hexadecimal byte : throw error ?
        0
    }
}

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