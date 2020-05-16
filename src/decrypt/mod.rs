use crate::hexxor;
use log::debug;
use std::collections::HashMap;

pub fn single_char_xor(hex: &str) -> Option<String> {
    let mut res = None;
    let mut max_score = -1.;
    (0..0xff).for_each(|c| match hexxor::xor_string_byte(hex, c) {
        Ok(s) => {
            let score = score(&s);
            if score > max_score {
                res = Some(s.clone());
                max_score = score;
            }
            debug!("Decrypting with {} gives {} with score {}", c, s, score);
        }
        Err(e) => debug!("Error with {} : {}", c, e),
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
    // source : https://en.wikipedia.org/wiki/Letter_frequency
    vec![
        ('a', 8.497),
        ('b', 1.492),
        ('c', 2.202),
        ('d', 4.253),
        ('e', 11.162),
        ('f', 2.228),
        ('g', 2.015),
        ('h', 6.094),
        ('i', 7.546),
        ('j', 0.153),
        ('k', 1.292),
        ('l', 4.025),
        ('m', 2.406),
        ('n', 6.749),
        ('o', 7.507),
        ('p', 1.929),
        ('q', 0.095),
        ('r', 7.587),
        ('s', 6.327),
        ('t', 9.356),
        ('u', 2.758),
        ('v', 0.978),
        ('w', 2.560),
        ('x', 0.150),
        ('y', 1.994),
        ('z', 0.077),
        (' ', 12.),
    ]
    .into_iter()
    .collect()
}
