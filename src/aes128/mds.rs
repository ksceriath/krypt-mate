use std::convert::identity;

fn modulus(operand: u8) -> u8 {
    ((operand >> 7) & 1) * 0x1B
}

fn into_3(operand: u8) -> u8 {
    (operand << 1) ^ operand ^ modulus(operand)
}

fn into_2(operand: u8) -> u8 {
    (operand << 1) ^ modulus(operand)
}

fn into_4(operand: u8) -> u8 {
    into_2(into_2(operand))
}

fn into_8(operand: u8) -> u8 {
    into_2(into_2(into_2(operand)))
}

fn into_9(operand: u8) -> u8 {
    into_8(operand) ^ operand
}

fn into_11(operand: u8) -> u8 {
    into_8(operand) ^ into_2(operand) ^ operand
}

fn into_13(operand: u8) -> u8 {
    into_8(operand) ^ into_4(operand) ^ operand
}

fn into_14(operand: u8) -> u8 {
    into_8(operand) ^ into_4(operand) ^ into_2(operand)
}

const CIRCULANT_MULTIPLIER: [[fn(u8) -> u8; 4]; 4] = [
    [into_2, into_3, identity, identity],
    [identity, into_2, into_3, identity],
    [identity, identity, into_2, into_3],
    [into_3, identity, identity, into_2],
];

const INVERSE_CIRCULANT_MULTIPLIER: [[fn(u8) -> u8; 4]; 4] = [
    [into_14, into_11, into_13, into_9],
    [into_9, into_14, into_11, into_13],
    [into_13, into_9, into_14, into_11],
    [into_11, into_13, into_9, into_14],
];

fn dot_product(a: &[u8], b: &[fn(u8) -> u8; 4]) -> u8 {
    (0..4).fold(0, |acc, index| acc ^ b[index](a[index]))
}

pub fn multiply_circulant(vector: &[u8]) -> Vec<u8> {
    let mut iter = (0..4).map(|index| dot_product(vector, &CIRCULANT_MULTIPLIER[index]));
    vec![
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

pub fn multiply_circulant_inverse(vector: &[u8]) -> Vec<u8> {
    let mut iter = (0..4).map(|index| dot_product(vector, &INVERSE_CIRCULANT_MULTIPLIER[index]));
    vec![
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modulus_should_return_0_if_leading_bit_is_unset() {
        let operand = 0x7f;
        assert_eq!(modulus(operand), 0);
    }

    #[test]
    fn modulus_should_return_1b_if_leading_bit_is_set() {
        let operand = 0x80;
        assert_eq!(modulus(operand), 0x1b);
    }

    #[test]
    fn into_2_should_work_as_expected() {
        let operand = 0xff;
        let expected = 0xe5;
        let actual = into_2(operand);
        assert_eq!(actual, expected);
    }

    #[test]
    fn into_3_should_work_as_expected() {
        let operand = 0xff;
        let expected = 0x1a;
        let actual = into_3(operand);
        assert_eq!(actual, expected);
    }

    #[test]
    fn into_4_should_work_as_expected() {
        let operand = 0xff;
        let expected = 0xd1;
        let actual = into_4(operand);
        assert_eq!(actual, expected);
    }

    #[test]
    fn into_8_should_work_as_expected() {
        let operand = 0xff;
        let expected = 0xb9;
        let actual = into_8(operand);
        assert_eq!(actual, expected);
    }

    #[test]
    fn into_9_should_work_as_expected() {
        let operand = 0xff;
        let expected = 0x46;
        let actual = into_9(operand);
        assert_eq!(actual, expected);
    }

    #[test]
    fn into_11_should_work_as_expected() {
        let operand = 0xff;
        let expected = 0xa3;
        let actual = into_11(operand);
        assert_eq!(actual, expected);
    }

    #[test]
    fn into_13_should_work_as_expected() {
        let operand = 0xff;
        let expected = 0x97;
        let actual = into_13(operand);
        assert_eq!(actual, expected);
    }

    #[test]
    fn into_14_should_work_as_expected() {
        let operand = 0xff;
        let expected = 0x8d;
        let actual = into_14(operand);
        assert_eq!(actual, expected);
    }

    #[test]
    fn dot_product_should_work_as_expected() {
        let into_11 = |operand| 11 * operand;
        let into_13 = |operand| 13 * operand;
        let into_17 = |operand| 17 * operand;
        let into_19 = |operand| 19 * operand;
        let actual = dot_product(&[2, 3, 5, 7], &[into_11, into_13, into_17, into_19]);
        let expected = 0xe1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_circulant_should_correctly_compute_1() {
        let vector = vec![219, 19, 83, 69];
        let actual = multiply_circulant(&vector);
        let expected = vec![142, 77, 161, 188];
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_circulant_should_correctly_compute_2() {
        let vector = vec![242, 10, 34, 92];
        let actual = multiply_circulant(&vector);
        let expected = vec![159, 220, 88, 157];
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_circulant_should_correctly_compute_3() {
        let vector = vec![1, 1, 1, 1];
        let actual = multiply_circulant(&vector);
        let expected = vec![1, 1, 1, 1];
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_circulant_should_correctly_compute_4() {
        let vector = vec![198, 198, 198, 198];
        let actual = multiply_circulant(&vector);
        let expected = vec![198, 198, 198, 198];
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_circulant_should_correctly_compute_5() {
        let vector = vec![212, 212, 212, 213];
        let actual = multiply_circulant(&vector);
        let expected = vec![213, 213, 215, 214];
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_circulant_should_correctly_compute_6() {
        let vector = vec![45, 38, 49, 76];
        let actual = multiply_circulant(&vector);
        let expected = vec![77, 126, 189, 248];
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_circulant_inverse_should_correctly_compute_1() {
        let vector = vec![142, 77, 161, 188];
        let actual = multiply_circulant_inverse(&vector);
        let expected = vec![219, 19, 83, 69];
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_circulant_inverse_should_correctly_compute_2() {
        let vector = vec![159, 220, 88, 157];
        let actual = multiply_circulant_inverse(&vector);
        let expected = vec![242, 10, 34, 92];
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_circulant_inverse_should_correctly_compute_3() {
        let vector = vec![1, 1, 1, 1];
        let actual = multiply_circulant_inverse(&vector);
        let expected = vec![1, 1, 1, 1];
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_circulant_inverse_should_correctly_compute_4() {
        let vector = vec![198, 198, 198, 198];
        let actual = multiply_circulant_inverse(&vector);
        let expected = vec![198, 198, 198, 198];
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_circulant_inverse_should_correctly_compute_5() {
        let vector = vec![213, 213, 215, 214];
        let actual = multiply_circulant_inverse(&vector);
        let expected = vec![212, 212, 212, 213];
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_circulant_inverse_should_correctly_compute_6() {
        let vector = vec![77, 126, 189, 248];
        let actual = multiply_circulant_inverse(&vector);
        let expected = vec![45, 38, 49, 76];
        assert_eq!(actual, expected);
    }
}
