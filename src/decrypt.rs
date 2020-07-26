use crate::hexaa;
use crate::encodings;
use log::debug;
use std::collections::HashMap;

/// Runs the input strings through single-char XORs
/// with all the possible character bytes (0x00 to 0xff)
/// and returns the string with highest score
/// calculated as weighted sum of letter frequencies of english alphabet.
/// Letters [a-z] and space [' '] are included in the calculated frequency score.
pub fn single_char_xor(ss: &[&str]) -> Option<String> {
    let mut max_score = 0.;
    let mut result = None;
    for ref s in ss.iter() {
        let (out, score) = helper(s);
        if score > max_score {
            max_score = score;
            result = out;
        }
    }
    result
}

fn helper(hex: &str) -> (Option<String>, f32) {
    let hex = encodings::hex_as_bytes(hex);
    let mut res = None;
    let mut max_score = -1.;
    let scorer = LetterFrequency::new();
    (0..0xff).for_each(
        |c| match String::from_utf8(hexaa::repeated_byte_xor(&hex, c)) {
            Ok(s) => {
                let score = scorer.score(&s);
                if score > max_score {
                    res = Some(s.clone());
                    max_score = score;
                }
                debug!("Decrypting with {} gives {} with score {}", c, s, score);
            }
            Err(e) => debug!(
                "Decrypting with {} did not give a UTF6 encoded string : {}",
                c, e
            ),
        },
    );
    (res, max_score)
}

struct LetterFrequency {
    data: HashMap<char, f32>,
}

impl LetterFrequency {
    fn new() -> Self {
        // source : https://en.wikipedia.org/wiki/Letter_frequency
        LetterFrequency {
            data: vec![
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
            .collect(),
        }
    }

    fn score(&self, s: &str) -> f32 {
        s.chars()
            .map(|c| {
                *self
                    .data
                    .get(&c.to_ascii_lowercase())
                    .copied()
                    .get_or_insert(0.)
            })
            .fold(0., |acc, x| acc + x)
    }
}
