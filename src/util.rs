
// Exercise 1-1
#[derive(Debug, PartialEq)]
pub struct HexParseError; // make this more useful

fn hex_u8_to_u8(x: u8) -> Result<u8, HexParseError> {
    let is_letter = (((x >= b'A') & (x <= b'F')) | ((x >= b'a') & (x <=b'f'))) as u8;
    let letter_off = (x & 0b0000111) + 9;
    let is_digit = ((x >= b'0') & (x <= b'9')) as u8;
    let digit_off = x & 0b0001111;
    let output = is_letter * letter_off + is_digit * digit_off;
    match is_letter | is_digit {
        0 => Err(HexParseError),
        _ => Ok(output),
    }
}

pub fn hex_to_base64(hex_bytes: Vec<u8>) -> Result<Vec<u8>, HexParseError> {
    const SYMBOLS: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let input = Vec::from_hex_byte_vec(hex_bytes)?;
    let mut output = vec![0u8; input.len() * 4 / 3];
    let mut output_pos = 0;
    for chunk in input.chunks(3) {
        output[output_pos] = SYMBOLS[(chunk[0] >> 2) as usize];
        output_pos += 1;
        output[output_pos] = SYMBOLS[((chunk[0] & 0b11) << 4 | chunk[1] >> 4) as usize];
        output_pos += 1;
        output[output_pos] = SYMBOLS[((chunk[1] & 0b1111) << 2 | chunk[2] >> 6) as usize];
        output_pos += 1;
        output[output_pos] = SYMBOLS[(chunk[2] & 0b111111) as usize];
        output_pos += 1;
    }
    Ok(output)
}





// Exercise 1-2


trait ParseHexByteArray {
    fn from_hex_byte_vec(src: Vec<u8>) -> Result<Vec<u8>, HexParseError>;
    fn from_hex_byte_array(src: &[u8]) -> Result<Vec<u8>, HexParseError>;
}

impl ParseHexByteArray for Vec<u8> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_u8_to_u8() {
        for x in 0..255 {
            let y = hex_u8_to_u8(x);
            if x >= b'0' && x <= b'9' {
                assert_eq!(y, Ok(x - b'0'));
            } else if x >= b'A' && x <= b'F' {
                assert_eq!(y, Ok(x - b'A' + 10));
            } else if x >= b'a' && x <= b'f' {
                assert_eq!(y, Ok(x - b'a' + 10));
            } else {
                assert_eq!(y, Err(HexParseError));
            }
        }
    }

    #[test]
    fn test_hex_to_base_64() {
        let hex = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex_to_base64(hex.to_vec()), Ok(base64.to_vec()));
    }

    #[test]
    fn test_from_hex_byte_array() {
        assert_eq!(Vec::from_hex_byte_array(b"123456").unwrap(), vec![0x12u8, 0x34u8, 0x56u8]);
        assert_eq!(Vec::from_hex_byte_array(b"12345"), Err(HexParseError));
        
    }

}