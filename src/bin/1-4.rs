use cryptopals::bytes::ParseBytes;
use cryptopals::frequency::break_single_byte_xor;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Exercise 1-4 solution

fn main() -> Result<(), std::io::Error> {
    let filename = std::env::args().nth(1).expect("requires a file to read");
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let inputs: Vec<Vec<u8>> = reader
        .lines()
        .map(|x| Vec::from_hex_byte_array(x.expect("read error").as_bytes()).expect("parse error"))
        .collect();
    let all_runs = inputs.iter().map(|x| break_single_byte_xor(x.clone()));

    let (score, key, decrypt) = all_runs
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .expect("best not found");
    println!(
        "Best: {} {:x} {:x?} {}",
        score,
        key,
        decrypt,
        String::from_utf8_lossy(decrypt.as_slice())
    );

    Ok(())
}
