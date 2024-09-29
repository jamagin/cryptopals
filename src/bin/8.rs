use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use cryptopals::{
    bytes::{ParseBytes, RenderBytes},
    distance::hamming_distance,
};

// Exercise 8 solution

fn most_similar_16_block_score(cyphertext: &[u8]) -> (usize, usize, usize) {
    let blocks: Vec<Vec<u8>> = cyphertext.chunks_exact(16).map(|x| x.into()).collect();
    let mut results: Vec<(usize, usize, usize)> = vec![];
    for i in 0..blocks.len() {
        for j in 0..i {
            if i != j {
                results.push((i, j, hamming_distance(&blocks[i], &blocks[j])));
            }
        }
    }
    results
        .into_iter()
        .min_by_key(|x| x.2)
        .expect("couldn't find min")
}

fn pretty_print_blocks(cyphertext: &[u8]) -> String {
    let blocks: Vec<Vec<u8>> = cyphertext.chunks_exact(16).map(|x| x.into()).collect();
    let strings: Vec<String> = blocks
        .into_iter()
        .map(|x| String::from_utf8_lossy(x.to_vec().to_hex_byte_vec().as_slice()).to_string())
        .collect();
    strings.join(" ")
}

fn main() -> Result<(), std::io::Error> {
    let filename = std::env::args().nth(1).expect("requires a file to read");
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let inputs: Vec<Vec<u8>> = reader
        .lines()
        .map(|x| Vec::from_hex_byte_array(x.expect("read error").as_bytes()).expect("parse error"))
        .collect();
    let results = inputs
        .into_iter()
        .map(|cyphertext| {
            let (i, j, score) = most_similar_16_block_score(&cyphertext);
            (i, j, score, cyphertext)
        })
        .min_by_key(|x| x.2)
        .expect("couldn't find min");
    println!("{:?} {}", results, pretty_print_blocks(&results.3));
    Ok(())
}
