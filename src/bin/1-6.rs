use std::{collections::BTreeMap, fs::File, io::Read};

use cryptopals::{
    bytes::{xor_byte_array, ParseBytes},
    distance::hamming_distance,
    frequency::break_single_byte_xor,
};

// Exercise 1-6 solution

fn main() -> Result<(), std::io::Error> {
    let filename = std::env::args().nth(1).expect("requires a file to read");
    let mut file = File::open(filename)?;
    let mut file_buf = vec![];
    file.read_to_end(&mut file_buf).expect("read error");

    let input = Vec::from_base64_byte_array(file_buf.as_slice()).expect("bad base64 input");

    fn normalized_key_size_score(key_size: usize, data: &[u8]) -> usize {
        // I didn't want to use f32 scores so I could use BTreeMap
        // multiplying times the data length seems sensible
        (0..2)
            .map(|i| {
                hamming_distance(
                    &data[0..key_size],
                    &data[(i + 1) * key_size..(i + 2) * key_size],
                ) * data.len()
                    / key_size
            })
            .sum()
    }

    let mut key_size_scores = BTreeMap::new();
    for key_size in 2..40 {
        key_size_scores.insert(
            normalized_key_size_score(key_size, input.as_slice()),
            key_size,
        );
    }

    fn transpose_by_block(block_size: usize, data: &[u8]) -> Vec<Vec<u8>> {
        let mut result = vec![];
        for offset in 0..block_size {
            let mut slice = vec![];
            let mut ptr = 0;
            while ptr + offset < data.len() {
                slice.push(data[ptr + offset]);
                ptr += block_size;
            }
            result.push(slice);
        }
        result
    }

    for _ in 0..3 {
        println!("{:?}", key_size_scores.pop_first());
        let (_, key_size) = key_size_scores.pop_first().expect("didn't find a key_size");
        let likely_key: Vec<u8> = transpose_by_block(key_size, &input)
            .iter()
            .map(|block| break_single_byte_xor(block.clone()).1)
            .collect();
        let decrypt = xor_byte_array(input.as_slice(), likely_key.as_slice());
        println!("{}", String::from_utf8_lossy(decrypt.as_slice()));
    }
    Ok(())
}
