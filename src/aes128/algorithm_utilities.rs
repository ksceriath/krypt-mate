use super::mds;
use super::s_box::SBox;

pub fn add_round_key(state: u128, keys: &[u32; 4]) -> u128 {
    state
        ^ (((keys[0] as u128) << 96)
            | ((keys[1] as u128) << 64)
            | ((keys[2] as u128) << 32)
            | (keys[3] as u128))
}

pub fn sub_bytes(state: u128) -> u128 {
    SBox::new().byte_wise_s_word_128(state)
}

pub fn inverse_sub_bytes(state: u128) -> u128 {
    SBox::inverse_new().byte_wise_s_word_128(state)
}

pub fn shift_rows(state: u128) -> u128 {
    first_row_left_shift(second_row_shift(third_row_right_shift(state)))
}

pub fn inverse_shift_rows(state: u128) -> u128 {
    first_row_right_shift(second_row_shift(third_row_left_shift(state)))
}

pub fn mix_columns(state: u128) -> u128 {
    mix_column_processor(state, mds::multiply_circulant)
}

pub fn inverse_mix_columns(state: u128) -> u128 {
    mix_column_processor(state, mds::multiply_circulant_inverse)
}

fn mix_column_processor(state: u128, f: fn(vector: &[u8]) -> Vec<u8>) -> u128 {
    let bytes = split_into_bytes(state);
    let mixed_bytes: Vec<u8> = bytes.chunks_exact(4).flat_map(|chunk| f(chunk)).collect();
    merge_bytes(mixed_bytes)
}

fn split_into_bytes(state: u128) -> Vec<u8> {
    (0..16).map(|position| nth_byte(state, position)).collect()
}

fn merge_bytes(bytes: Vec<u8>) -> u128 {
    bytes
        .into_iter()
        .fold(0, |state, byte| (state << 8) | (byte as u128))
}

fn first_row_left_shift(state: u128) -> u128 {
    shift_bytes(state, vec![1, 5, 9, 13])
}

fn first_row_right_shift(state: u128) -> u128 {
    shift_bytes(state, vec![13, 9, 5, 1])
}

fn second_row_shift(state: u128) -> u128 {
    shift_bytes(shift_bytes(state, vec![2, 10]), vec![6, 14])
}

fn third_row_right_shift(state: u128) -> u128 {
    shift_bytes(state, vec![15, 11, 7, 3])
}

fn third_row_left_shift(state: u128) -> u128 {
    shift_bytes(state, vec![3, 7, 11, 15])
}

fn shift_bytes(state: u128, positions: Vec<u32>) -> u128 {
    let mut a = positions.clone();
    a.rotate_right(1);
    a.iter()
        .zip(positions.iter())
        .map(|(p1, p2)| (*p1, nth_byte(state, *p2)))
        .fold(state, |state, positions| {
            replace_byte(state, positions.1, positions.0)
        })
}

fn replace_byte(state: u128, byte: u8, position: u32) -> u128 {
    // set `position` byte as 0, and OR it with `byte`
    (state & (!((0xff as u128) << ((15 - position) * 8))))
        | ((byte as u128) << ((15 - position) * 8))
}

fn nth_byte(word: u128, position: u32) -> u8 {
    ((word >> ((15 - position) * 8)) & 0xff) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth_byte_should_return_last_byte() {
        let actual = nth_byte(0xabcdef, 15);
        assert_eq!(actual, 0xef);
    }

    #[test]
    fn nth_byte_should_return_third_byte() {
        let actual = nth_byte(0xabcdef, 13);
        assert_eq!(actual, 0xab);
    }

    #[test]
    #[should_panic]
    fn nth_byte_should_panic_for_out_of_limit_position() {
        nth_byte(0xabcdef, 16);
    }

    #[test]
    fn replace_byte_should_replace_the_required_byte() {
        let actual = replace_byte(0xabcde10234567789, 0xff, 13);
        let expected = 0xabcde10234ff7789;
        assert_eq!(actual, expected);
    }

    #[test]
    fn shift_rows_should_work_as_expected() {
        let state: u128 = 0x12_34_56_78_90_ab_cd_ef_11_22_33_44_55_66_77_88;
        let actual = shift_rows(state);
        let expected = 0x12_ab_33_88_90_22_77_78_11_66_56_ef_55_34_cd_44;
        assert_eq!(actual, expected);
    }

    #[test]
    fn inverse_shift_rows_should_work_as_expected() {
        let state = 0x12_ab_33_88_90_22_77_78_11_66_56_ef_55_34_cd_44;
        let actual = inverse_shift_rows(state);
        let expected = 0x12_34_56_78_90_ab_cd_ef_11_22_33_44_55_66_77_88;
        assert_eq!(actual, expected);
    }

    #[test]
    fn split_into_bytes_should_split_as_expected() {
        let actual = split_into_bytes(0x12_34_56_78_90_ab_cd_ef_11_22_33_44_55_66_77_88);
        let expected = vec![
            0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
            0x77, 0x88,
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn merge_bytes_should_merge_as_expected() {
        let actual = merge_bytes(vec![
            0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
            0x77, 0x88,
        ]);
        let expected = 0x12_34_56_78_90_ab_cd_ef_11_22_33_44_55_66_77_88;
        assert_eq!(actual, expected);
    }

    #[test]
    fn mix_columns_should_work_as_expected() {
        let state = 0xdb135345_f20a225c_01010101_c6c6c6c6;
        let expected: u128 = 0x8e4da1bc_9fdc589d_01010101_c6c6c6c6;
        let actual = mix_columns(state);
        assert_eq!(actual, expected);
    }

    #[test]
    fn inverse_mix_columns_should_work_as_expected() {
        let expected = 0xdb135345_f20a225c_01010101_c6c6c6c6;
        let state = 0x8e4da1bc_9fdc589d_01010101_c6c6c6c6;
        let actual = inverse_mix_columns(state);
        assert_eq!(actual, expected);
    }

    #[test]
    fn add_round_key_should_return_0_for_same_keys_as_state() {
        let state = 0xdb135345_f20a225c_01010101_c6c6c6c6;
        let round_keys = [0xdb135345, 0xf20a225c, 0x01010101, 0xc6c6c6c6];
        let actual = add_round_key(state, &round_keys);
        assert_eq!(actual, 0);
    }
}
