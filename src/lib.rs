#[macro_use]
extern crate lazy_static;

extern crate rand;

mod block;
mod keys;
mod sbox;
mod pbox;
mod modes;
mod errors;

pub use self::block::Block;
pub use self::keys::{Key, Key80Bit, Key128Bit};
pub use self::modes::OpMode;
pub use self::errors::DecryptError;

pub fn encrypt_str<K: Key>(text: &str, key: &K, mode: &OpMode) -> (Vec<u8>, Option<Block>) {
    // Check how much padding needs to be appended to the string
    let pad_len = match text.len() % 8 {
        0 => 8,
        x => 8 - x,
    };
    let mut ciphertext: Vec<Block> = Vec::with_capacity((text.len() + pad_len) / 8);

    match *mode {
        OpMode::ECB => {
            let mut current_bytes = [0u8; 8];
            for (i, byte) in text.bytes().enumerate() {
                current_bytes[i % 8] = byte;

                if i % 8 == 7 {
                    let mut block = Block::from_bytes(&current_bytes);
                    block.encrypt(key);
                    ciphertext.push(block);
                }
            }

            add_padding(&mut current_bytes, pad_len);

            let mut block = Block::from_bytes(&current_bytes);
            block.encrypt(key);
            ciphertext.push(block);

            (blocks_to_bytes(ciphertext), None)
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
            block.encrypt(key);

            // Add final block to ciphertext vector
            ciphertext.push(block);

            // Return ciphertext in bytes + IV
            (blocks_to_bytes(ciphertext), Some(iv))
        },
    }
}

pub fn decrypt_str<K: Key>(ciphertext: &[u8], key: &K, mode: &OpMode, init_vec: Option<Block>) -> Result<String, DecryptError> {
    // Check that ciphertext is at least one block
    if ciphertext.len() < 8 {
        return Err(DecryptError::CiphertextTooShort(ciphertext.len()));
    }

    // Check that ciphertext length aligns with block size
    if ciphertext.len() % 8 != 0 {
        return Err(DecryptError::CiphertextNotAligned(ciphertext.len()));
    }

    let mut plain_bytes: Vec<u8> = Vec::with_capacity(ciphertext.len());

    match *mode {
        OpMode::ECB => {
            let mut current_bytes = [0u8; 8];
            for (i, byte) in ciphertext.iter().enumerate() {
                current_bytes[i % 8] = *byte;

                if i % 8 == 7 {
                    let mut block = Block::from_bytes(&current_bytes);
                    block.decrypt(key);
                    plain_bytes.extend(block.to_bytes().iter());
                }
            }

            let len = plain_bytes.len();
            let to_remove = check_padding(&plain_bytes[(len - 8)..])?;
            plain_bytes.truncate(len - to_remove);

            String::from_utf8(plain_bytes).map_err(|e| DecryptError::from(e))
        },
        OpMode::CBC => {
            let mut last_block = match init_vec {
                Some(x) => x,
                None => return Err(DecryptError::InitVecMissing),
            };

            let mut current_bytes = [0u8; 8];
            for(i, byte) in ciphertext.iter().enumerate() {
                current_bytes[i % 8] = *byte;

                if i % 8 == 7 {
                    let mut block = Block::from_bytes(&current_bytes);
                    block.decrypt(key);
                    block ^= &last_block;
                    plain_bytes.extend(block.to_bytes().iter());

                    last_block = Block::from_bytes(&current_bytes);
                }
            }

            let len = plain_bytes.len();
            let to_remove = check_padding(&plain_bytes[(len - 8)..])?;
            plain_bytes.truncate(len - to_remove);

            String::from_utf8(plain_bytes).map_err(|e| DecryptError::from(e))
        },
    }
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

fn check_padding(final_block: &[u8]) -> Result<usize, DecryptError> {
    if final_block.len() != 8 {
        panic!("Logic error! Received {} element slice for padding check, expected 8 elements!", final_block.len());
    }

    let pad = final_block[7];
    if pad > 8 {
        return Err(DecryptError::InvalidPadding);
    }

    for byte in final_block.iter().rev().take(pad as usize) {
        if *byte != pad {
            return Err(DecryptError::InvalidPadding);
        }
    }

    Ok(pad as usize)
}

fn blocks_to_bytes(blocks: Vec<Block>) -> Vec<u8> {
    let mut ret = Vec::with_capacity(blocks.len() * 8);
    for block in &blocks {
        for byte in &(block.to_bytes()) {
            ret.push(*byte);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{add_padding, blocks_to_bytes, check_padding};

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
    fn test_check_padding_returns_correct_values() {
        let bytes = [0x4E, 0xDD, 0xA0, 0x34, 0x04, 0x04, 0x04, 0x04];
        assert_eq!(check_padding(&bytes).unwrap(), 4);

        let bytes = [0x4E, 0xDD, 0xA0, 0x34, 0x04, 0x03, 0x03, 0x03];
        assert_eq!(check_padding(&bytes).unwrap(), 3);

        let bytes = [0x4E, 0xDD, 0xA0, 0x34, 0xBC, 0xE5, 0xA2, 0x01];
        assert_eq!(check_padding(&bytes).unwrap(), 1);

        let bytes = [0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08];
        assert_eq!(check_padding(&bytes).unwrap(), 8);
    }

    #[test]
    #[should_panic]
    fn test_check_padding_rejects_invalid_slices() {
        let bytes = [0x34, 0x14];
        check_padding(&bytes).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_check_padding_rejects_invalid_padding() {
        let bytes = [0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09];
        check_padding(&bytes).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_check_padding_rejects_wrong_padding() {
        let bytes = [0x35, 0xD2, 0x39, 0xE5, 0xAA, 0x04, 0x03, 0x03];
        check_padding(&bytes).unwrap();
    }

    #[test]
    fn test_blocks_to_bytes() {
        let blocks = vec![Block::new(0x0123456789ABCDEF_u64), Block::new(0xFEDCBA9876543210_u64)];
        let bytes = blocks_to_bytes(blocks);
        assert_eq!(bytes[0], 0x01);
        assert_eq!(bytes[5], 0xAB);
        assert_eq!(bytes[8], 0xFE);
        assert_eq!(bytes[15], 0x10);
    }
}
