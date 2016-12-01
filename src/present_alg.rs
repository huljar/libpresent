use errors::KeyError;
use sbox::S_BOX;

struct Block {
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
}

enum Key {
    Key80Bit([u16; 5]), // 80 bit key
    Key128Bit([u16; 8]), // 128 bit key
}

#[derive(Copy, Clone)] // Just for testing purposes, will probably be removed later
struct RoundKey {
    pub value: u64, // Round keys have the same size as the block size
}

fn generate_round_keys(key: &Key) -> Result<[RoundKey; 32], KeyError> {
    match key {
        &Key::Key80Bit(keyVal) => {
            Ok([RoundKey { value: 0u64 }; 32])
        },
        &Key::Key128Bit(_) => panic!("128 bit keys are not implemented yet!"),
    }
}

fn apply_substitution() {

}

fn apply_permutation() {

}
