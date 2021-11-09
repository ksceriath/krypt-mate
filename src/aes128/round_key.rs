use super::s_box::SBox;

pub struct RoundKey {
    round_keys: Vec<u32>,
}

impl RoundKey {
    pub fn new(key: u128, round_count: u32) -> Self {
        let sbox = SBox::new();
        let mut prev: [u32; 4] = Self::split_words(key);
        let mut round_keys = Vec::with_capacity(44);
        round_keys.extend_from_slice(&prev);
        (0..round_count).for_each(|round| {
            prev[0] = prev[0]
                ^ sbox.byte_wise_s_word_32(Self::left_cyclic_byte_rotate(prev[3]))
                ^ Self::r_con(round);
            prev[1] = prev[1] ^ prev[0];
            prev[2] = prev[2] ^ prev[1];
            prev[3] = prev[3] ^ prev[2];

            round_keys.push(prev[0]);
            round_keys.push(prev[1]);
            round_keys.push(prev[2]);
            round_keys.push(prev[3]);
        });
        RoundKey { round_keys }
    }

    pub fn get(&self, round: usize) -> &[u32] {
        let index = round * 4;
        &self.round_keys[index..index + 4]
    }

    fn split_words(bits: u128) -> [u32; 4] {
        let first_word = (bits >> 96) as u32;
        let second_word = (bits >> 64) as u32;
        let third_word = (bits >> 32) as u32;
        let fourth_word = bits as u32;
        [first_word, second_word, third_word, fourth_word]
    }

    fn left_cyclic_byte_rotate(bytes: u32) -> u32 {
        (bytes << 8) | ((bytes & 0xff000000) >> 24)
    }

    fn r_con(round: u32) -> u32 {
        (Self::RC[round as usize] as u32) << 24
    }

    const RC: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_keys_should_be_correct() {
        let key = 0x54_68_61_74_73_20_6D_79_20_4B_75_6E_67_20_46_75;
        let expected: Vec<u32> = vec![
            0x54_68_61_74,
            0x73_20_6D_79,
            0x20_4B_75_6E,
            0x67_20_46_75,
            0xE2_32_FC_F1,
            0x91_12_91_88,
            0xB1_59_E4_E6,
            0xD6_79_A2_93,
            0x56_08_20_07,
            0xC7_1A_B1_8F,
            0x76_43_55_69,
            0xA0_3A_F7_FA,
            0xD2_60_0D_E7,
            0x15_7A_BC_68,
            0x63_39_E9_01,
            0xC3_03_1E_FB,
            0xA1_12_02_C9,
            0xB4_68_BE_A1,
            0xD7_51_57_A0,
            0x14_52_49_5B,
            0xB1_29_3B_33,
            0x05_41_85_92,
            0xD2_10_D2_32,
            0xC6_42_9B_69,
            0xBD_3D_C2_87,
            0xB8_7C_47_15,
            0x6A_6C_95_27,
            0xAC_2E_0E_4E,
            0xCC_96_ED_16,
            0x74_EA_AA_03,
            0x1E_86_3F_24,
            0xB2_A8_31_6A,
            0x8E_51_EF_21,
            0xFA_BB_45_22,
            0xE4_3D_7A_06,
            0x56_95_4B_6C,
            0xBF_E2_BF_90,
            0x45_59_FA_B2,
            0xA1_64_80_B4,
            0xF7_F1_CB_D8,
            0x28_FD_DE_F8,
            0x6D_A4_24_4A,
            0xCC_C0_A4_FE,
            0x3B_31_6F_26,
        ];
        let actual = RoundKey::new(key, 10).round_keys;
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_should_return_correct_elements() {
        let keys = RoundKey {
            round_keys: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        };
        let actual = keys.get(2);
        let expected = [8, 9, 10, 11];
        assert_eq!(actual, expected)
    }

    #[test]
    fn left_cyclic_byte_rotate_test() {
        let actual = RoundKey::left_cyclic_byte_rotate(0x13579bdf);
        let expected = 0x579bdf13;
        assert_eq!(actual, expected);
    }

    #[test]
    fn split_words_test() {
        let actual = RoundKey::split_words(0x13579bdf_23579adf_33579cdf_43579edf);
        let expected: [u32; 4] = [0x13579bdf, 0x23579adf, 0x33579cdf, 0x43579edf];
        assert_eq!(actual, expected);
    }
}
