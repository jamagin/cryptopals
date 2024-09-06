#[derive(Debug, PartialEq)]
pub struct HexParseError; // make this more useful

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

const BASE64_OFFSETS: [u8; 127] = generate_base64_offsets();

const fn generate_base64_offsets() -> [u8; 127] {
    let mut table = [0u8; 127];
    let mut i = 0;
    while i < BASE64_SYMBOLS.len() {
        table[BASE64_SYMBOLS[i] as usize] = i as u8;
        i += 1;
    }
    table
}


pub trait ParseBytes {
    fn from_hex_byte_vec(src: Vec<u8>) -> Result<Vec<u8>, HexParseError>;
    fn from_hex_byte_array(src: &[u8]) -> Result<Vec<u8>, HexParseError>;
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

// Exercise 1-2
pub fn xor_byte_array(message: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(message.len() >= key.len());
    
    let key_extended_iter = key.iter().cycle().take(message.len());
    message
    .iter()
    .zip(key_extended_iter)
    .map(|(x, y)| x ^ y)
    .collect()
}

#[cfg(test)]
mod tests {
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

    // Exercise 1-1 solution
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

    // Exercise 1-2 solution
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

    // Exercise 1-5 solution
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
}
