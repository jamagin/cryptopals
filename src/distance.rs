use std::{cmp::min, iter::zip};

// Wagner-Fischer algorithm
#[allow(clippy::needless_range_loop)] // maintaining the style of the pseudocode
pub fn levenshtein_distance(a: &[u8], b: &[u8]) -> usize {
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }
    let mut d = vec![vec![0usize; b.len()]; a.len()];

    for i in 0..a.len() {
        d[i][0] = i;
    }

    for j in 0..b.len() {
        d[0][j] = j;
    }

    for j in 1..b.len() {
        for i in 1..a.len() {
            let mut substitution_cost = 0usize;
            if a[i] != b[j] {
                substitution_cost = 1;
            }
            d[i][j] = min(
                min(d[i - 1][j] + 1, d[i][j - 1] + 1),
                d[i - 1][j - 1] + substitution_cost,
            );
        }
    }
    d[a.len() - 1][b.len() - 1]
}

pub fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    zip(a, b).map(|(a, b)| (a ^ b).count_ones()).sum::<u32>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance(b"abcdef", b"abcdef"), 0);
        assert_eq!(levenshtein_distance(b"", b""), 0);
        assert_eq!(levenshtein_distance(b"abcdef", b""), 6);
        assert_eq!(levenshtein_distance(b"abcdef", b"abzdef"), 1);
        assert_eq!(levenshtein_distance(b"abcdef", b"abcef"), 1);
        assert_eq!(levenshtein_distance(b"abcdef", b"abccdef"), 1);
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37);
    }

    // Exercise 1-5
    #[test]
    fn decrypt_repeating_xor() {}
}
