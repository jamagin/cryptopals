use std::{cmp::min, collections::HashMap};
use crate::bytes::xor_byte_vec;


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

// Exercise 1-3
const MOST_FREQUENT_ENGLISH_LETTERS: [u8; 26] = *b"etaoinshrdlcumwfgypbvkjxqz";

fn count_frequencies(text: &Vec<u8>) -> Vec<u8> {
    let frequencies = text.iter().filter(|x| x.is_ascii_alphabetic())
    .map(|x| x.to_ascii_lowercase())
    .fold(HashMap::new(), |mut map, val| {
        map.entry(val)
        .and_modify(|frq|*frq+=1usize)
        .or_insert(1usize);
        map
    });
    
    let mut most_frequent_vec = frequencies.iter().collect::<Vec<(&u8, &usize)>>();
    // let mut most_frequent_vec = frequencies.iter().collect::<Vec<(&u8, &usize)>>();
    most_frequent_vec.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    
    most_frequent_vec.iter().map(|(v, _)| *v).cloned().collect()
}

pub fn crack_single_byte_xor(cyphertext: Vec<u8>) -> (u8, Vec<u8>) {
    let mut min_distance: Option<usize> = None;
    let mut best_key = 0x00;
    let mut best_decrypt = cyphertext.clone();

    println!("{:x?}", cyphertext.clone());
    for key in 0x00..=0xff {
        let decrypt = xor_byte_vec(&cyphertext, &vec![key]);
        println!("{} {:x?} {}", key, decrypt, String::from_utf8_lossy(decrypt.clone().as_slice()));
        let frequencies = count_frequencies(&decrypt);
        let distance = levenshtein_distance(MOST_FREQUENT_ENGLISH_LETTERS.to_vec(), frequencies.clone());
        if (min_distance == None) || (distance < min_distance.unwrap()) {
            min_distance = Some(distance);
            best_key = key;
            best_decrypt = decrypt.clone();
        }
    }
    (best_key, best_decrypt)    
}
    

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