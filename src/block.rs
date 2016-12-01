use keys::RoundKey;
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
        for split in 0..15 {
            let shift = 4 * split;
            new_state += (S_BOX.apply((self.state >> shift) as u8) as u64) << shift;
        }
        self.state = new_state;
    }

    fn apply_permutation(&mut self) {
        // Send the current state through the P-Box
        self.state = P_BOX.apply(self.state);
    }
}
