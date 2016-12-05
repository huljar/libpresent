#[macro_use]
extern crate lazy_static;

extern crate rand;

mod block;
mod keys;
mod sbox;
mod pbox;
mod modes;

pub use self::block::Block;
pub use self::keys::{Key, Key80Bit, Key128Bit};
pub use self::modes::OpMode;

use std::str::Utf8Error;

pub fn encrypt_str<K: Key>(text: &str, key: &K, mode: &OpMode) -> (Vec<u8>, Option<Block>) {
    // Check how much padding needs to be appended to the string
    let pad_len = match text.len() % 8 {
        0 => 8,
        x => 8 - x,
    };
    let mut ciphertext: Vec<Block> = Vec::with_capacity((text.len() + pad_len) / 8);

    match *mode {
        OpMode::ECB => {
            panic!("Not yet implemented!");
        },
        OpMode::CBC => {
            let iv = modes::random_iv();

            let mut current_bytes = [0u8; 8];
            for (i, byte) in text.bytes().enumerate() {
                // Fill current block with bytes from the input string
                current_bytes[i % 8] = byte;

                // When a block is full, process it
                if i % 8 == 7 {
                    // Encrypt current block
                    let mut block = Block::from_bytes(&current_bytes);

                    // XOR with previous block (IV for the first block)
                    match ciphertext.iter().rev().next() {
                        Some(pb) => block ^= pb,
                        None => block ^= &iv,
                    };

                    // Perform actual encryption
                    block.encrypt(key);

                    // Add encrypted block to ciphertext vector
                    ciphertext.push(block);
                }
            }

            // Add padding
            add_padding(&mut current_bytes, pad_len);

            // Encrypt final block
            let mut block = Block::from_bytes(&current_bytes);
            match ciphertext.iter().rev().next() {
                Some(pb) => block ^= pb,
                None => block ^= &iv,
            };

            // Add final block to ciphertext vector
            ciphertext.push(block);

            // Return ciphertext in bytes + IV
            (ciphertext_to_bytes(ciphertext), Some(iv))
        },
    }
}

pub fn decrypt_str<K: Key>(ciphertext: &[u8], key: &K, mode: &OpMode, init_vec: &Block) -> Result<String, Utf8Error> {
    panic!("Not yet implemented!");
}

fn add_padding(current_bytes: &mut [u8; 8], pad_len: usize) {
    if pad_len > 8 {
        panic!("Logic error! Padding length cannot be >8, but is {}", pad_len);
    }

    for byte in current_bytes.iter_mut().rev().take(pad_len) {
        // PKCS5 padding (pad with bytes all of the same value
        // as the number of padding bytes)
        *byte = pad_len as u8;
    }
}

fn ciphertext_to_bytes(ciphertext: Vec<Block>) -> Vec<u8> {
    let mut ret = Vec::with_capacity(ciphertext.len() * 8);
    for block in &ciphertext {
        for byte in &(block.to_bytes()) {
            ret.push(*byte);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::{add_padding, ciphertext_to_bytes};
    use block::Block;

    #[test]
    fn test_add_padding_to_block() {
        let mut bytes = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        add_padding(&mut bytes, 4);
        assert_eq!(bytes, [0x01, 0x23, 0x45, 0x67, 0x04, 0x04, 0x04, 0x04]);

        add_padding(&mut bytes, 1);
        assert_eq!(bytes, [0x01, 0x23, 0x45, 0x67, 0x04, 0x04, 0x04, 0x01]);

        add_padding(&mut bytes, 8);
        assert_eq!(bytes, [0x08; 8]);
    }

    #[test]
    #[should_panic]
    fn test_add_padding_rejects_invalid_length() {
        let mut bytes = [0u8; 8];
        add_padding(&mut bytes, 9);
    }

    #[test]
    fn test_ciphertext_to_bytes() {
        let blocks = vec![Block::new(0x0123456789ABCDEF_u64), Block::new(0xFEDCBA9876543210_u64)];
        let bytes = ciphertext_to_bytes(blocks);
        assert_eq!(bytes[0], 0x01);
        assert_eq!(bytes[5], 0xAB);
        assert_eq!(bytes[8], 0xFE);
        assert_eq!(bytes[15], 0x10);
    }
}
