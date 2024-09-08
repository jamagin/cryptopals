// for AES-128 per FIPS 197:
// 16 byte (128 bit key)
// 10 rounds

pub fn decrypt_aes_128_ecb(key: &[u8], cyphertext: &[u8]) -> Vec<u8> {
    vec![]
}

// multiplication in GF(2^8)
// FIXME I don't like this
const fn xtimes(a: u8, b: u8) -> u8 {
    let mut a_mut = a;
    let mut b_mut = b;
    let mut result: u16 = 0;
    let mut i = 0;
    while i < 8 {
        if b_mut & 1 == 1 {
            result ^= a_mut as u16;
        }
        let carry = a_mut & 0x80 == 0x80;
        a_mut <<= 1;
        if carry {
            a_mut ^= 0x1b;
        }
        b_mut >>= 1;
        i += 1;
    }
    result as u8
}

const Rcon: [u32; 11] = generate_rcon();
const fn generate_rcon() -> [u32; 11] {
    let mut table = [0u32; 11];
    let mut acc: u8 = 1;
    let mut i = 1;
    while i <= 10 {
        table[i] = (acc as u32) << 24;
        acc = xtimes(acc, 2);
        i += 1;
    }
    table
}

const fn rotword(a: u32) -> u32 {
    a.rotate_left(8)
}

const fn inverse(a: u8) -> u8 {
    let mut a_inv = a;
    if a_inv != 0 {
        let mut i = 0;
        while i < 253 {
            a_inv = xtimes(a_inv, a);
            i += 1;
        }
    }
    a_inv
}

// left to right, this is the columns of the matrix as bytes
const transform_matrix: [u8; 8] = [
    0b11111000,
    0b01111100,
    0b00111110,
    0b00011111,
    0b10001111,
    0b11000111,
    0b11100011,
    0b11110001,
];

const fn sbox (a: u8) -> u8 {


    let a_inv = inverse(a);

    let mut acc: u8 = 0;
    let mut i = 0;
    let reduction: u8 = 0b01100011;
    let first_transform_row: u8 = 0b11110001;
    while i < 8 {
        let row = first_transform_row.rotate_left(i);
        let tmp = row & a_inv;
        acc |= ((tmp.count_ones() as u8) & 1) << i;
        i += 1;
    }
    acc ^ reduction
}

fn subword(a: u32) -> u32 {
    ((sbox((a >> 24) as u8) as u32) << 24) |
    ((sbox((a >> 16) as u8) as u32) << 16) |
    ((sbox((a >> 8) as u8) as u32) << 8) |
    (sbox(a as u8) as u32)
}

fn key_expansion(key: &[u8], Nk: usize, Nr: usize) -> Vec<u32> {
    let mut w = vec![0u32; Nk * (Nr+1)];
    for i in 0..Nk {
        for j in 0..Nk {
            w[i] |= (key[j] as u32) << ((Nk - 1 - j) * 8);
        }
    }
    for i in Nk..=4*Nr+3 {
        let mut temp = w[i - 1];
        if i % Nk == 0 {
            temp = subword(rotword(temp)) ^ Rcon[i / Nk];
        } else if (Nk > 6) && (i % Nk == 4) {
            temp = subword(temp)
        }
        w[i] = w[i - Nk] ^ temp;
    }
    w
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bytes::ParseBytes;

    #[test]
    // based on Appendix A of FIPS 197
    fn test_key_expansion() {


        for i in 0x0..=0xf {
            for j in 0x0..=0xf {
                print!("{:2x} ", sbox((i<<4) + j));
           }
           println!();
        }

        // let key = Vec::from_hex_byte_array(b"2b7e151628aed2a6abf7158809cf4f3c").unwrap();
        // let expanded: Vec<u32> = vec![
        //     0x2b7e1516, 0x28aed2a6, 0xabf71588, 0xabf71588, 0xa0fafe17, 0x88542cb1, 0x23a33939,
        //     0x2a6c7605, 0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f, 0x3d80477d, 0x4716fe3e,
        //     0x1e237e44, 0x6d7a883b, 0xef44a541, 0xa8525b7f, 0xb671253b, 0xdb0bad00, 0xd4d1c6f8,
        //     0x7c839d87, 0xcaf2b8bc, 0x11f915bc, 0x6d88a37a, 0x110b3efd, 0xdbf98641, 0xca0093fd,
        //     0x4e54f70e, 0x5f5fc9f3, 0x84a64fb2, 0x4ea6dc4f, 0xead27321, 0xb58dbad2, 0x312bf560,
        //     0x7f8d292f, 0xac7766f3, 0x19fadc21, 0x28d12941, 0x575c006e, 0xd014f9a8, 0xc9ee2589,
        //     0xe13f0cc8, 0xb6630ca6,
        // ];
        // assert_eq!(key_expansion(&key, key.len()/4, 10), expanded);
    }

//     #[test]
//     fn test_aes_128_ecb_decrypt() {
//         let cyphertext = Vec::from_base64_byte_array(
//             b"wFHE//yjH+f8ZNyYulYNmDcBxXOgLkqTFp5jcyiO6wVf7WGDdECNqhUuG9TMW6sP\
// exwZineeuL0xuuXdLP8BrxWV+XNHdR/yBAVgnOSDRoiAxugMHjs06GuRF/ihwFQJ\
// 1qhhuwAXzo7k7DfG5s/JmGkw+i9BcnnO4QBnqixHzzuv0kFyUpRW4O1hlIyr5bo3\
// r0aCB9FlVf+tB8f9SteYZ9Y12G+f1n3n1hVSdOiAMuU1qfgy6VmH350PrbdwNv5K",
//         )
//         .unwrap();
//         let key = Vec::from_hex_byte_array(b"9c1501ffb829537afba091def401a25c").unwrap();
//         assert_eq!(
//             decrypt_aes_128_ecb(key.as_slice(), cyphertext.as_slice()).as_slice(),
//             b"I know you wanted me to stay\n\
// But I can't ignore the crazy visions of me in LA\n\
// And I heard that there's a special place\n\
// Where boys and girls can all be queens every single day\n"
//         )
//     }
}
