#[derive(Debug, PartialEq)]
pub struct HexParseError; // make this more useful
#[derive(Clone, Copy, Debug)]
pub struct Base64ParseError;

fn hex_u8_to_u8(x: u8) -> Result<u8, HexParseError> {
    let is_letter = ((b'A'..=b'F').contains(&x) | (b'a'..=b'f').contains(&x)) as u8;
    let letter_off = (x & 0b0000111) + 9;
    let is_digit = x.is_ascii_digit() as u8;
    let digit_off = x & 0b0001111;
    let output = is_letter * letter_off + is_digit * digit_off;
    match is_letter | is_digit {
        0 => Err(HexParseError),
        _ => Ok(output),
    }
}

const BASE64_SYMBOLS: [u8; 64] =
    *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

const BASE64_OFFSETS: [Result<u8, Base64ParseError>; 127] = generate_base64_offsets();
const fn generate_base64_offsets() -> [Result<u8, Base64ParseError>; 127] {
    let mut table = [Err(Base64ParseError); 127];
    table[b'=' as usize] = Ok(0u8); // padding, will not be left in result
    let mut i = 0;
    while i < BASE64_SYMBOLS.len() {
        table[BASE64_SYMBOLS[i] as usize] = Ok(i as u8);
        i += 1;
    }
    table
}

pub trait ParseBytes {
    fn from_hex_byte_vec(src: Vec<u8>) -> Result<Vec<u8>, HexParseError>;
    fn from_hex_byte_array(src: &[u8]) -> Result<Vec<u8>, HexParseError>;
    fn from_base64_byte_array(src: &[u8]) -> Result<Vec<u8>, Base64ParseError>;
}

impl ParseBytes for Vec<u8> {
    fn from_hex_byte_vec(src: Vec<u8>) -> Result<Self, HexParseError> {
        if src.len() % 2 == 1 {
            return Err(HexParseError);
        }

        let input = src.to_vec();
        let mut output = Vec::with_capacity(src.len() * 2);
        for chunk in input.chunks(2) {
            output.push(hex_u8_to_u8(chunk[0])? << 4 | hex_u8_to_u8(chunk[1])?);
        }
        Ok(output)
    }

    fn from_hex_byte_array(src: &[u8]) -> Result<Self, HexParseError> {
        Self::from_hex_byte_vec(src.to_vec())
    }

    fn from_base64_byte_array(src: &[u8]) -> Result<Self, Base64ParseError> {
        let src_filtered: Vec<u8> = src
            .iter()
            .filter(|x| BASE64_SYMBOLS.contains(x) || **x == b'=')
            .copied()
            .collect();
        if src_filtered.len() % 4 != 0 {
            return Err(Base64ParseError);
        }
        let mut result: Vec<u8> = Vec::with_capacity(src_filtered.len() * 3 / 4);
        for chunk in src_filtered.chunks(4) {
            let resolved = ((BASE64_OFFSETS[chunk[0] as usize]? as u32) << 18)
                + ((BASE64_OFFSETS[chunk[1] as usize]? as u32) << 12)
                + ((BASE64_OFFSETS[chunk[2] as usize]? as u32) << 6)
                + (BASE64_OFFSETS[chunk[3] as usize]? as u32);
            result.push((resolved >> 16) as u8);
            result.push(((resolved >> 8) & 0xff) as u8);
            result.push((resolved & 0xff) as u8);
        }
        let padding = src_filtered[src_filtered.len() - 2..src_filtered.len()]
            .iter()
            .filter(|x| **x == b'=')
            .count();
        if padding > 0 {
            result.truncate(result.len() - padding);
        }
        Ok(result)
    }
}

pub trait RenderBytes {
    fn to_base64_byte_vec(&self) -> Vec<u8>;
    fn to_hex_byte_vec(&self) -> Vec<u8>;
}

impl RenderBytes for Vec<u8> {
    fn to_base64_byte_vec(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(self.len() * 4 / 3);
        for chunk in self.chunks(3) {
            output.push(BASE64_SYMBOLS[(chunk[0] >> 2) as usize]);
            output.push(BASE64_SYMBOLS[((chunk[0] & 0b11) << 4 | chunk[1] >> 4) as usize]);
            output.push(BASE64_SYMBOLS[((chunk[1] & 0b1111) << 2 | chunk[2] >> 6) as usize]);
            output.push(BASE64_SYMBOLS[(chunk[2] & 0b111111) as usize]);
        }
        output
    }

    fn to_hex_byte_vec(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(self.len() * 2);
        for byte in self {
            let pair = format!("{:02x}", byte).into_bytes();
            output.push(pair[0]);
            output.push(pair[1]);
        }
        output
    }
}

// Exercise 2
pub fn xor_byte_array(message: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(message.len() >= key.len());

    let key_extended_iter = key.iter().cycle().take(message.len());
    message
        .iter()
        .zip(key_extended_iter)
        .map(|(x, y)| x ^ y)
        .collect()
}

// Exercise 9 solution
pub fn pkcs7_pad(message: &[u8], block_size: usize) -> Vec<u8> {
    assert!(block_size <= 256);
    let padding_size = block_size - (message.len() % block_size);
    let mut padding = vec![padding_size as u8; padding_size];
    let mut padded_message = Vec::from(message);
    padded_message.append(&mut padding);
    padded_message
}

#[cfg(test)]
mod tests {
    use assert_hex::assert_eq_hex;

    use super::*;

    #[test]
    fn test_hex_u8_to_u8() {
        for x in 0..255 {
            let y = hex_u8_to_u8(x);
            if x.is_ascii_digit() {
                assert_eq!(y, Ok(x - b'0'));
            } else if (b'A'..=b'F').contains(&x) {
                assert_eq!(y, Ok(x - b'A' + 10));
            } else if (b'a'..=b'f').contains(&x) {
                assert_eq!(y, Ok(x - b'a' + 10));
            } else {
                assert_eq!(y, Err(HexParseError));
            }
        }
    }

    // Exercise 1 solution
    #[test]
    fn test_hex_to_base_64() {
        let hex = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let input = Vec::from_hex_byte_array(hex).unwrap();
        assert_eq!(input.to_base64_byte_vec(), base64.to_vec());
    }

    #[test]
    fn test_from_hex_byte_array() {
        assert_eq!(
            Vec::from_hex_byte_array(b"123456").unwrap(),
            vec![0x12u8, 0x34u8, 0x56u8]
        );
        assert_eq!(Vec::from_hex_byte_array(b"12345"), Err(HexParseError));
    }

    #[test]
    fn test_as_hex_byte_vec() {
        let hex = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(
            Vec::from_hex_byte_array(hex).unwrap().to_hex_byte_vec(),
            hex.to_vec()
        );
    }

    // Exercise 2 solution
    #[test]
    fn test_fixed_xor_byte_vec() {
        let a = b"1c0111001f010100061a024b53535009181c";
        let b = b"686974207468652062756c6c277320657965";
        let expected = b"746865206b696420646f6e277420706c6179";

        let xored = xor_byte_array(
            &Vec::from_hex_byte_array(a).unwrap(),
            &Vec::from_hex_byte_array(b).unwrap(),
        );
        assert_eq!(xored.to_hex_byte_vec(), expected.to_vec());
    }

    #[test]
    fn test_repeated_xor_byte_vec() {
        let plaintext = b"000000ffffff";
        let key = b"ff00";
        let expected = b"ff00ffff00ff";
        assert_eq!(
            xor_byte_array(
                &Vec::from_hex_byte_array(plaintext).unwrap(),
                &Vec::from_hex_byte_array(key).unwrap()
            ),
            Vec::from_hex_byte_array(expected).unwrap()
        );
    }

    // Exercise 5 solution
    #[test]
    fn test_repeated_xor_byte_vec_for_1_5() {
        let plaintext = b"Burning 'em, if you ain't quick and nimble\n\
        I go crazy when I hear a cymbal";
        let key = b"ICE";
        let expected =
            b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
            a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        assert_eq!(
            xor_byte_array(plaintext, key),
            Vec::from_hex_byte_array(expected).unwrap()
        );
    }

    #[test]
    fn test_base64_decode() {
        assert_eq!(Vec::from_base64_byte_array(b"Q2F0").unwrap(), b"Cat");
        assert_eq!(
            Vec::from_base64_byte_array(b"dGFuZ2libGU=").unwrap(),
            b"tangible"
        );
        assert_eq!(
            Vec::from_base64_byte_array(b"Y29nbml6YW5jZQ==").unwrap(),
            b"cognizance"
        );
        assert_eq!(
            Vec::from_base64_byte_array(b"VGhpc\nyBpc yBhIE1\nJTUUgdGVzdA==\n").unwrap(),
            b"This is a MIME test"
        );
    }

    // Exercise 9
    #[test]
    fn test_pkcs7_pad() {
        assert_eq_hex!(
            pkcs7_pad(b"YELLOW SUBMARINE", 20).as_slice(),
            b"YELLOW SUBMARINE\x04\x04\x04\x04"
        );
    }
}
