use cryptopals::bytes::ParseBytes;
use cryptopals::frequency::break_single_byte_xor;

// Exercise 1-3 solution
fn main() {
    let cyphertext = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let cypher_bytes = Vec::from_hex_byte_array(cyphertext).unwrap();
    let (key, text) = break_single_byte_xor(cypher_bytes);
    println!(
        "{:x} {:x?} {}",
        key,
        text,
        String::from_utf8(text.clone()).unwrap()
    );
}
