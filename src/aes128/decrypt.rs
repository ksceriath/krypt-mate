use super::algorithm_utilities::*;
use super::round_key::RoundKey;
use std::convert::TryInto;

pub fn decrypt(bytes: u128, key: u128) -> u128 {
    decrypt_block(bytes, RoundKey::new(key, 10))
}

fn decrypt_block(bytes: u128, round_key: RoundKey) -> u128 {
    let initial_state = inverse_sub_bytes(inverse_shift_rows(add_round_key(
        bytes,
        round_key.get(10).try_into().unwrap(),
    )));

    let state = (1..=9).rev().fold(initial_state, |state, round| {
        inverse_sub_bytes(inverse_shift_rows(inverse_mix_columns(add_round_key(
            state,
            round_key.get(round).try_into().unwrap(),
        ))))
    });

    add_round_key(state, round_key.get(0).try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decrypt_block_should_work_as_expected() {
        let bytes = 0x370da167_1a3de46b_a6307c65_fbb14597;
        let key = 0;
        let round_key = RoundKey::new(key, 10);
        let actual = decrypt_block(bytes, round_key);
        let expected = 0xdb135345_f20a225c_01010101_c6c6c6c6;
        assert_eq!(actual, expected);
    }
}
