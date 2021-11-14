use crate::encodings;
use crate::hexaa;
use std::collections::HashMap;

pub fn find_vignere_key(bytes: &Vec<u8>) -> Vec<u8> {
    let key_size = find_optimum_key_size(bytes);

    chunk_and_transpose(&bytes, key_size)
        .iter()
        .map(single_byte_xor)
        .map(|x| x.unwrap().0)
        .collect()
}

/// Groups the vector into {{chunk_size}}d chunks and
/// then transposes the vectors, to give {{chunk_size}}
/// number of vectors.
fn chunk_and_transpose<T: Copy>(v: &Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    let mut transposed_blocks = (0..chunk_size).map(|_| vec![]).collect();
    v.chunks(chunk_size).for_each(|block| {
        block
            .iter()
            .zip(&mut transposed_blocks)
            .for_each(|(x, y): (&T, &mut Vec<T>)| y.push(*x));
    });
    transposed_blocks
}

/// Iterates over key_sizes from 2 to 40, and returns the key_size for which
/// the average normalized hamming distance is minimum.
pub fn find_optimum_key_size(s: &Vec<u8>) -> usize {
    (2..41)
        .into_iter()
        .map(|k| (k, encodings::average_normalized_hamming_distance(k, s)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap()
        .0
}

/// Runs the input hex-strings through single-char XORs
/// with all the possible character bytes (0x00 to 0xff)
/// and returns the string with highest score
/// calculated as weighted sum of letter frequencies of english alphabet.
/// Letters [a-z] and space [' '] are included in the calculated frequency score.
/// TODO this is not a utility; move this to the challenge (3, 4) specific code instead
pub fn single_char_xor(ss: &[&str]) -> Option<(u8, String)> {
    ss.iter()
        .map(|s| encodings::hex_as_bytes(s))
        .map(|bytes| single_byte_xor(&bytes))
        .filter(|result| result.is_some())
        .map(|some_result| some_result.unwrap())
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .map(|x| (x.0, x.1))
}

/// Runs the input vector of bytes through repeated_byte_xor with all the possible bytes (keys)
/// returning the key, score, and the xor'ed output for the key with highest score.
/// Score is calculated based on English language letter frequencies.
fn single_byte_xor(bytes: &Vec<u8>) -> Option<(u8, String, f32)> {
    let scorer = LetterFrequency::new();
    (0..=0xff)
        .map(|key| {
            (
                key,
                String::from_utf8(hexaa::repeated_byte_xor(&bytes, key)),
            )
        })
        .filter(|(_, result)| result.is_ok())
        .map(|(key, result)| (key, result.unwrap()))
        .map(|(key, string)| (key, string.to_string(), scorer.score(&string)))
        .max_by(|(_, _, score1), (_, _, score2)| score1.partial_cmp(score2).unwrap())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_and_transpose_should_do_so() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert_eq!(
            chunk_and_transpose(&v, 3),
            vec![vec![0, 3, 6, 9], vec![1, 4, 7, 0], vec![2, 5, 8]]
        );
    }
}
