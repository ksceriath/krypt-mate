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
    s.chars()
        .map(|c| {
            *scoring_map()
                .get(&c.to_ascii_lowercase())
                .copied()
                .get_or_insert(0.)
        })
        .fold(0., |acc, x| acc + x)
}

fn scoring_map() -> HashMap<char, f32> {
    let mut x = HashMap::new();
    // source : https://en.wikipedia.org/wiki/Letter_frequency
    x.insert('a', 8.497);
    x.insert('b', 1.492);
    x.insert('c', 2.202);
    x.insert('d', 4.253);
    x.insert('e', 11.162);
    x.insert('f', 2.228);
    x.insert('g', 2.015);
    x.insert('h', 6.094);
    x.insert('i', 7.546);
    x.insert('j', 0.153);
    x.insert('k', 1.292);
    x.insert('l', 4.025);
    x.insert('m', 2.406);
    x.insert('n', 6.749);
    x.insert('o', 7.507);
    x.insert('p', 1.929);
    x.insert('q', 0.095);
    x.insert('r', 7.587);
    x.insert('s', 6.327);
    x.insert('t', 9.356);
    x.insert('u', 2.758);
    x.insert('v', 0.978);
    x.insert('w', 2.560);
    x.insert('x', 0.150);
    x.insert('y', 1.994);
    x.insert('z', 0.077);
    x.insert(' ', 12.);
    x
}
