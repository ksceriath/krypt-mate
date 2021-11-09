use super::algorithm_utilities::*;
use super::round_key::RoundKey;
use std::convert::TryInto;

pub fn encrypt_block(bytes: u128, round_key: RoundKey) -> u128 {
    let init_keys = round_key.get(0);

    let initial_state = add_round_key(bytes, init_keys.try_into().unwrap());
    let state = (1..=9).fold(initial_state, |state, round| {
        add_round_key(
            mix_columns(shift_rows(sub_bytes(state))),
            round_key.get(round).try_into().unwrap(),
        )
    });

    add_round_key(
        shift_rows(sub_bytes(state)),
        round_key.get(10).try_into().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_block_should_work_as_expected() {
        let bytes = 0xdb135345_f20a225c_01010101_c6c6c6c6;
        let key = 0;
        let round_key = RoundKey::new(key, 10);
        let actual = encrypt_block(bytes, round_key);
        let expected = 0x370da167_1a3de46b_a6307c65_fbb14597;
        assert_eq!(actual, expected);
    }
}
