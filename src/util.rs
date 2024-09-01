
// Exercise 1-1

fn hex_u8_to_u8(x: u8) -> u8 {
    let is_letter = (((x >= b'A') & (x <= b'F')) | ((x >= b'a') & (x <=b'f'))) as u8;
    let letter_off = (x & 0b0000111) + 9;
    let is_digit = ((x >= b'0') & (x <= b'9')) as u8;
    let digit_off = x & 0b0001111;
    is_letter * letter_off + is_digit * digit_off
}

pub fn hex_to_base64(hex_bytes: Vec<u8>) -> Vec<u8> {
    const SYMBOLS: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut input = hex_bytes.clone();
    input.resize(input.len() / 3 * 3, 0);
    let mut output = vec![0u8; input.len() * 2 / 3];
    let mut output_pos = 0;
    for chunk in input.chunks(3) {
        let a: u16 = ((hex_u8_to_u8(chunk[0]) as u16) << 8) +
            ((hex_u8_to_u8(chunk[1]) as u16) << 4) +
            hex_u8_to_u8(chunk[2]) as u16;
            output[output_pos] = SYMBOLS[(a >> 6 & 0b111111) as usize];
            output_pos += 1;
            output[output_pos] = SYMBOLS[(a & 0b111111) as usize];
            output_pos += 1;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_u8_to_u8() {
        for x in 0..255 {
            let y = hex_u8_to_u8(x);
            if x >= b'0' && x <= b'9' {
                assert_eq!(y, x - b'0');
            } else if x >= b'A' && x <= b'F' {
                assert_eq!(y, x - b'A' + 10);
            } else if x >= b'a' && x <= b'f' {
                assert_eq!(y, x - b'a' + 10);
            } else {
                assert_eq!(y, 0);
            }
        }
    }

    #[test]
    fn test_hex_to_base_64() {
        let hex = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex_to_base64(hex.to_vec()), base64);
    }
}