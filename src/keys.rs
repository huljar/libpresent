use sbox::S_BOX;

pub trait Key {
    // PRESENT consists of 31 rounds plus a special 32nd round
    fn generate_round_keys(&self) -> [RoundKey; 32];
}

pub struct Key80Bit {
    pub value: [u8; 10],
}

impl Key80Bit {
    fn new(value: [u8; 10]) -> Self {
        Key80Bit { value: value}
    }
}

impl Key for Key80Bit {
    fn generate_round_keys(&self) -> [RoundKey; 32] {
        // The round keys are generated as follows:
        // 1. Take the 64 leftmost bits of the key register
        // 2. Mutate the key register (cyclic bitshift, S-Box (partial),
        //    XOR with round counter (partial))
        // 3. Repeat until 32 round keys are extracted
        let mut round_keys = [RoundKey { value: 0u64 }; 32];
        let mut key_register = self.value;

        for round in 1u8..32u8 { // round counter starts at 1!
            // Get round key
            let mut key_val = 0u64;
            for byte in 0..8 {
                key_val += (key_register[byte] as u64) << ((7 - byte) * 8);
            }
            round_keys[(round - 1) as usize].value = key_val;

            // Cyclic bitshift (rotate by 61 bits to the left)
            let tmp_register = key_register;
            for byte in 0..10 {
                key_register[byte] = (tmp_register[(byte + 7) % 10] << 5) +
                                     (tmp_register[(byte + 8) % 10] >> 3);
            }

            // Apply S-Box to leftmost 4 bits
            let sbox_result = S_BOX.apply(key_register[0] >> 4);
            key_register[0] = key_register[0] % 16;
            key_register[0] += sbox_result << 4;

            // XOR bits 19..14 with the round counter
            key_register[7] ^= round >> 1;
            key_register[8] ^= round << 7;
        }

        // Get final round key
        let mut final_key = 0u64;
        for byte in 0..8 {
            final_key += (key_register[byte] as u64) << ((7 - byte) * 8);
        }
        round_keys[31].value = final_key;

        round_keys
    }
}

pub struct Key128Bit {
    pub value: [u8; 16],
}

impl Key128Bit {
    fn new(value: [u8; 16]) -> Self {
        Key128Bit { value: value}
    }
}

impl Key for Key128Bit {
    fn generate_round_keys(&self) -> [RoundKey; 32] {
        panic!("128 bit keys are not yet implemented!");
    }
}

#[derive(Copy, Clone)]
pub struct RoundKey {
    pub value: u64, // Round keys have the same size as the block size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_key_generation_80bit() {
        // Create a key with the following hex representation:
        // 0xAC0A6E76326BC7E8280
        let key = Key80Bit::new([0xA, 0xC0, 0xA6, 0xE7, 0x63, 0x26, 0xBC, 0x7E, 0x82, 0x80]);
        let round_keys = key.generate_round_keys();
        assert_eq!(round_keys[0].value, 0xAC0A6E76326BC7E_u64);
        assert_eq!(round_keys[1].value, 0x7050015814DCEC64_u64);
        assert_eq!(round_keys[2].value, 0x3AF1EE0A002B029A_u64);
    }
}
