use crate::hexxor;
use std::collections::HashMap;

pub fn single_char_xor(hex: &str) -> String {
    let mut res: String = String::new();
    let mut max_score = -1.;
    (65..91).for_each(|c| match hexxor::xor_string_byte(hex, c) {
        Ok(s) => {
            let score = score(&s);
            if score > max_score {
                res = s.clone();
                max_score = score;
            }
            println!("Decrypting with {} gives {} with score {}", c, s, score);
        }
        Err(e) => println!("Error with {} : {}", c, e),
    });
    res
}

fn score(s: &str) -> f32 {
    let x = scoring_map();

    let mut score = 0.;
    s.chars().for_each(|c| {
        if x.contains_key(&c.to_ascii_uppercase()) {
            score += x.get(&c.to_ascii_uppercase()).unwrap();
        }
    });
    score
}

fn scoring_map() -> HashMap<char, f32> {
    let mut x = HashMap::new();
    x.insert('E', 12.02);
    x.insert('T', 9.10);
    x.insert('A', 8.12);
    x.insert('O', 7.68);
    x.insert('I', 7.31);
    x.insert('N', 6.95);
    x.insert('S', 6.28);
    x.insert('R', 6.02);
    x.insert('H', 5.92);
    x.insert('D', 4.32);
    x.insert('L', 3.98);
    x.insert('U', 2.88);
    x.insert('C', 2.71);
    x.insert('M', 2.61);
    x.insert('F', 2.30);
    x.insert('Y', 2.11);
    x.insert('W', 2.09);
    x.insert('G', 2.03);
    x.insert('P', 1.82);
    x.insert('B', 1.49);
    x.insert('V', 1.11);
    x.insert('K', 0.69);
    x.insert('X', 0.17);
    x.insert('Q', 0.11);
    x.insert('J', 0.10);
    x.insert('Z', 0.07);
    x
}
