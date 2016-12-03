use std::ops::BitXorAssign;

use keys::{Key, RoundKey};
use sbox::S_BOX;
use pbox::P_BOX;

pub struct Block {
    state: u64, // PRESENT block size is fixed to 64 bit
}

impl Block {
    fn add_round_key(&mut self, round_key: &RoundKey) {
        self.state ^= round_key.value;
    }

    fn apply_substitution(&mut self) {
        // Split the 64 bit state into sixteen 4 bit nibbles
        // Apply the S-Box to each of them independently
        let mut new_state = 0u64;
        for split in 0..16 {
            let shift = 4 * split;
            new_state += (S_BOX.apply(((self.state >> shift) as u8) % 16) as u64) << shift;
        }
        self.state = new_state;
    }

    fn apply_permutation(&mut self) {
        // Send the current state through the P-Box
        self.state = P_BOX.apply(self.state);
    }

    pub fn encrypt<K: Key>(&mut self, key: &K) {
        // Generate round keys
        let round_keys = key.generate_round_keys();

        // Iterate over rounds
        for round in 0..31 {
            *self ^= &round_keys[round];
            self.apply_substitution();
            self.apply_permutation();
        }

        // Add final round key
        *self ^= &round_keys[31];
    }

    pub fn decrypt<K: Key>(&mut self, key: &K) {
        panic!("Decryption is not yet implemented!");
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

#[cfg(test)]
mod tests {
    use super::*;
    use keys::Key80Bit;

    #[test]
    fn test_correct_sbox_application() {
        let mut block = Block { state: 0u64 };
        block.apply_substitution();
        assert_eq!(block.get_state(), 0xCCCCCCCCCCCCCCCC_u64);

        let mut block = Block { state: 0x0123456789ABCDEF_u64 };
        block.apply_substitution();
        assert_eq!(block.get_state(), 0xC56B90AD3EF84712_u64);
    }

    #[test]
    fn test_correct_pbox_application() {
        let mut block = Block { state: 0u64 };
        block.apply_permutation();
        assert_eq!(block.get_state(), 0u64);

        let mut block = Block { state: 0x0123456789ABCDEF };
        block.apply_permutation();
        assert_eq!(block.get_state(), 0x00FF0F0F33335555);
    }

    #[test]
    fn test_block_encryption_80bit_key() {
        // Test block encryption with the test vectors from the paper
        let mut block = Block { state: 0u64 };
        let key = Key80Bit { value: [0u8; 10] };
        block.encrypt(&key);
        assert_eq!(block.get_state(), 0x5579C1387B228445_u64);

        let mut block = Block { state: 0u64 };
        let key = Key80Bit { value: [0xFF_u8; 10] };
        block.encrypt(&key);
        assert_eq!(block.get_state(), 0xE72C46C0F5945049_u64);

        let mut block = Block { state: 0xFFFFFFFFFFFFFFFF_u64 };
        let key = Key80Bit { value: [0u8; 10] };
        block.encrypt(&key);
        assert_eq!(block.get_state(), 0xA112FFC72F68417B_u64);

        let mut block = Block { state: 0xFFFFFFFFFFFFFFFF_u64 };
        let key = Key80Bit { value: [0xFF_u8; 10] };
        block.encrypt(&key);
        assert_eq!(block.get_state(), 0x3333DCD3213210D2_u64);
    }
}
