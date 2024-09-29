use crate::bytes::xor_byte_array;
use std::{collections::HashMap, iter::zip};

#[rustfmt::skip]
const ENGLISH_LETTER_FREQ: [f32; 26] = // https://en.wikipedia.org/wiki/Letter_frequency
    [
        0.082, 0.015, 0.028, 0.043, 0.127, 0.022, 0.02,   // A-G
        0.061, 0.07, 0.0015, 0.0066, 0.04, 0.024, 0.067,  // H-N
        0.075, 0.019, 0.00095, 0.06, 0.063, 0.091, 0.028, // O-U
        0.0098, 0.024, 0.0015, 0.02, 0.00074,                // V-Z
    ];

fn count_frequencies(text: &[u8]) -> Vec<f32> {
    let frequencies = text
        .iter()
        .filter(|x| x.is_ascii_alphabetic())
        .map(|x| x.to_ascii_lowercase())
        .fold(HashMap::new(), |mut map, val| {
            map.entry(val)
                .and_modify(|frq| *frq += 1usize)
                .or_insert(1usize);
            map
        });

    let count_vec: Vec<usize> = (0u8..26u8)
        .map(|letter| frequencies.get(&(letter + b'a')).unwrap_or(&0))
        .copied()
        .collect();
    let total = count_vec.iter().sum::<usize>();

    // a missing character puts us far away
    count_vec
        .iter()
        .map(|freq| {
            if *freq == 0 {
                0f32
            } else {
                *freq as f32 / total as f32
            }
        })
        .collect()
}

// TODO: also try chisquared
fn sum_squares_distance(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    assert_eq!(a.len(), b.len());

    zip(a, b).map(|(a, b)| (a - b).powi(2)).sum()
}

pub fn distance_metric(text: &[u8]) -> f32 {
    let reference = ENGLISH_LETTER_FREQ.to_vec();
    let (letters, mut non_letters): (Vec<u8>, Vec<u8>) =
        text.iter().partition(|x| x.is_ascii_alphabetic());
    let frequencies = count_frequencies(&letters);
    // least sum of squares difference, but count of non-letters penalizes a lot
    non_letters.retain(|x| *x != b' '); // spaces are free
    let penalty = non_letters.len();
    sum_squares_distance(&frequencies, &reference) + penalty as f32
}

pub fn break_single_byte_xor(cyphertext: Vec<u8>) -> (f32, u8, Vec<u8>) {
    let mut min_distance: Option<f32> = None;
    let mut best_key = 0x00;
    let mut best_decrypt = cyphertext.clone();

    for key in 0x00..=0xff {
        let decrypt = xor_byte_array(&cyphertext, &[key]);
        let distance = distance_metric(&decrypt);

        if (min_distance.is_none()) || (distance < min_distance.unwrap()) {
            min_distance = Some(distance);
            best_key = key;
            best_decrypt = decrypt.clone();
        }
    }
    (
        min_distance.expect("did you even try"),
        best_key,
        best_decrypt,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_letter_frequency_reference() {
        const MOST_FREQUENT_ENGLISH_LETTERS: [u8; 26] = *b"etaoinshrdlcumwfgypbvkjxqz";
        let mut x: Vec<(u8, f32)> = ENGLISH_LETTER_FREQ
            .iter()
            .enumerate()
            .map(|(letter, freq)| (letter as u8 + b'a', *freq))
            .collect();
        x.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        assert_eq!(
            x.iter().map(|(letter, _)| *letter).collect::<Vec<u8>>(),
            MOST_FREQUENT_ENGLISH_LETTERS.to_vec()
        );
        assert!(
            x.iter()
                .map(|(_, freq)| *freq)
                .reduce(|a, b| a + b)
                .unwrap()
                .abs()
                - 1.0
                < 0.001
        );
    }
}
