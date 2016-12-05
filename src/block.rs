use std::ops::BitXorAssign;

use keys::{Key, RoundKey};
use sbox::S_BOX;
use pbox::P_BOX;

pub struct Block {
    state: u64, // PRESENT block size is fixed to 64 bit
}

impl Block {
    pub fn new(state: u64) -> Self {
        Block { state: state }
    }

    pub fn from_bytes(bytes: &[u8; 8]) -> Self {
        let mut state = 0u64;
        for (i, byte) in bytes.iter().rev().enumerate() {
            state += (*byte as u64) << (i * 8);
        }
        Block { state: state }
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        let mut ret = [0u8; 8];
        for (i, byte) in ret.iter_mut().rev().enumerate() {
            *byte = (self.state >> (i * 8)) as u8;
        }
        ret
    }

    fn add_round_key(&mut self, round_key: &RoundKey) {
        self.state ^= round_key.value;
    }

    fn apply_substitution_enc(&mut self) {
        // Split the 64 bit state into sixteen 4 bit nibbles
        // Apply the S-Box to each of them independently
        let mut new_state = 0u64;
        for split in 0..16 {
            let shift = 4 * split;
            new_state += (S_BOX.apply_enc(((self.state >> shift) as u8) % 16) as u64) << shift;
        }
        self.state = new_state;
    }

    fn apply_permutation_enc(&mut self) {
        // Send the current state through the P-Box
        self.state = P_BOX.apply_enc(self.state);
    }

    fn apply_substitution_dec(&mut self) {
        let mut new_state = 0u64;
        for split in 0..16 {
            let shift = 4 * split;
            new_state += (S_BOX.apply_dec(((self.state >> shift) as u8) % 16) as u64) << shift;
        }
        self.state = new_state;
    }

    fn apply_permutation_dec(&mut self) {
        self.state = P_BOX.apply_dec(self.state);
    }

    pub fn encrypt<K: Key>(&mut self, key: &K) {
        // Generate round keys
        let round_keys = key.generate_round_keys();

        // Iterate over rounds
        for round in 0..31 {
            *self ^= &round_keys[round];
            self.apply_substitution_enc();
            self.apply_permutation_enc();
        }

        // Add final round key
        *self ^= &round_keys[31];
    }

    pub fn decrypt<K: Key>(&mut self, key: &K) {
        // Generate round keys
        let round_keys = key.generate_round_keys();

        // Iterate over rounds in reverse order
        for round in (1..32).rev() {
            *self ^= &round_keys[round];
            self.apply_permutation_dec();
            self.apply_substitution_dec();
        }

        // Add final (first) round key
        *self ^= &round_keys[0];
    }

    pub fn get_state(&self) -> u64 {
        self.state
    }
}

impl<'a> BitXorAssign<&'a RoundKey> for Block {
    fn bitxor_assign(&mut self, rhs: &RoundKey) {
        self.add_round_key(rhs);
    }
}

impl<'a> BitXorAssign<&'a Block> for Block {
    fn bitxor_assign(&mut self, rhs: &Block) {
        self.state ^= rhs.state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use keys::Key80Bit;

    #[test]
    fn test_block_init_from_bytes() {
        let block = Block::from_bytes(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]);
        assert_eq!(block.get_state(), 0x0123456789ABCDEF_u64);
    }

    #[test]
    fn test_block_to_bytes() {
        let block = Block { state: 0x0123456789ABCDEF_u64 };
        assert_eq!(block.to_bytes(), [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]);
    }

    #[test]
    fn test_correct_sbox_application() {
        let mut block = Block { state: 0u64 };
        block.apply_substitution_enc();
        assert_eq!(block.get_state(), 0xCCCCCCCCCCCCCCCC_u64);
        block.apply_substitution_dec();
        assert_eq!(block.get_state(), 0u64);

        let mut block = Block { state: 0x0123456789ABCDEF_u64 };
        block.apply_substitution_enc();
        assert_eq!(block.get_state(), 0xC56B90AD3EF84712_u64);
        block.apply_substitution_dec();
        assert_eq!(block.get_state(), 0x0123456789ABCDEF_u64);
    }

    #[test]
    fn test_correct_pbox_application() {
        let mut block = Block { state: 0u64 };
        block.apply_permutation_enc();
        assert_eq!(block.get_state(), 0u64);
        block.apply_permutation_dec();
        assert_eq!(block.get_state(), 0u64);

        let mut block = Block { state: 0x0123456789ABCDEF };
        block.apply_permutation_enc();
        assert_eq!(block.get_state(), 0x00FF0F0F33335555);
        block.apply_permutation_dec();
        assert_eq!(block.get_state(), 0x0123456789ABCDEF);
    }

    #[test]
    fn test_block_encryption_decryption_80bit_key() {
        // Test block encryption with the test vectors from the paper
        let mut block = Block { state: 0u64 };
        let key = Key80Bit { value: [0u8; 10] };
        block.encrypt(&key);
        assert_eq!(block.get_state(), 0x5579C1387B228445_u64);
        block.decrypt(&key);
        assert_eq!(block.get_state(), 0u64);

        let mut block = Block { state: 0u64 };
        let key = Key80Bit { value: [0xFF_u8; 10] };
        block.encrypt(&key);
        assert_eq!(block.get_state(), 0xE72C46C0F5945049_u64);
        block.decrypt(&key);
        assert_eq!(block.get_state(), 0u64);

        let mut block = Block { state: 0xFFFFFFFFFFFFFFFF_u64 };
        let key = Key80Bit { value: [0u8; 10] };
        block.encrypt(&key);
        assert_eq!(block.get_state(), 0xA112FFC72F68417B_u64);
        block.decrypt(&key);
        assert_eq!(block.get_state(), 0xFFFFFFFFFFFFFFFF_u64);

        let mut block = Block { state: 0xFFFFFFFFFFFFFFFF_u64 };
        let key = Key80Bit { value: [0xFF_u8; 10] };
        block.encrypt(&key);
        assert_eq!(block.get_state(), 0x3333DCD3213210D2_u64);
        block.decrypt(&key);
        assert_eq!(block.get_state(), 0xFFFFFFFFFFFFFFFF_u64);
    }
}
