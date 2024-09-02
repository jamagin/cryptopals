use std::cmp::min;


// Wagner-Fischer algorithm, probably the wrong approach but I wanna
fn levenshtein_distance(a: Vec<u8>, b: Vec<u8>) -> usize {

    if a.len() == 0 {
        return b.len();
    }
    if b.len() == 0 {
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
            d[i][j] = min(min(d[i-1][j] + 1, d[i][j-1] + 1), d[i-1][j-1] + substitution_cost);
        }
    }
    d[a.len() - 1][b.len() - 1]
}

const MOST_FREQUENT_ENGLISH_LETTERS: [u8; 26] = *b"etaoinshrdlcumwfgypbvkjxqz";



mod tests {
    use super::*;

    // Exercise 1-3
    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance(b"abcdef".to_vec(), b"abcdef".to_vec()), 0);
        assert_eq!(levenshtein_distance(b"".to_vec(), b"".to_vec()), 0);
        assert_eq!(levenshtein_distance(b"abcdef".to_vec(), b"".to_vec()), 6);
        assert_eq!(levenshtein_distance(b"abcdef".to_vec(), b"abzdef".to_vec()), 1);
        assert_eq!(levenshtein_distance(b"abcdef".to_vec(), b"abcef".to_vec()), 1);
        assert_eq!(levenshtein_distance(b"abcdef".to_vec(), b"abccdef".to_vec()), 1);
    }
}