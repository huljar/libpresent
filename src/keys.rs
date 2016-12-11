use sbox::S_BOX;

/// The `Key` trait.
///
/// Any struct implementing this trait can be used as a key for
/// encryption and decryption within this crate.
pub trait Key {
    // PRESENT consists of 31 rounds plus a special 32nd round
    /// Generate 32 round keys that will be used for the 32 rounds
    /// of the PRESENT algorithm.
    fn generate_round_keys(&self) -> [RoundKey; 32];
}

/// An 80-bit key.
///
/// The [paper](https://link.springer.com/chapter/10.1007%2F978-3-540-74735-2_31)
/// introduces two key lengths: 80-bit and 128-bit. This struct represents
/// an 80-bit key and implements the appropriate key schedule.
pub struct Key80Bit {
    /// The value of the key as a byte array.
    pub value: [u8; 10],
}

impl Key80Bit {
    /// Constructs a new 80-bit key from the given bytes.
    pub fn new(value: [u8; 10]) -> Self {
        Key80Bit { value: value }
    }
}

impl Key for Key80Bit {
    /// The key schedule for 80-bit keys.
    ///
    /// This function generates 32 round keys that are derived
    /// from the value of this key.
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
            let sbox_result = S_BOX.apply_enc(key_register[0] >> 4);
            key_register[0] = key_register[0] % 16;
            key_register[0] += sbox_result << 4;

            // XOR bits 19, ..., 15 with the round counter
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

/// A 128-bit key.
///
/// The [paper](https://link.springer.com/chapter/10.1007%2F978-3-540-74735-2_31)
/// introduces two key lengths: 80-bit and 128-bit. This struct represents
/// a 128-bit key and implements the appropriate key schedule.
pub struct Key128Bit {
    /// The value of the key as a byte array.
    pub value: [u8; 16],
}

impl Key128Bit {
    /// Constructs a new 128-bit key from the given bytes.
    pub fn new(value: [u8; 16]) -> Self {
        Key128Bit { value: value}
    }
}

impl Key for Key128Bit {
    /// The key schedule for 128-bit keys.
    ///
    /// This function generates 32 round keys that are derived
    /// from the value of this key.
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
            for byte in 0..16 {
                key_register[byte] = (tmp_register[(byte + 7) % 16] << 5) +
                                     (tmp_register[(byte + 8) % 16] >> 3);
            }

            // Apply S-Box to leftmost 8 bits
            let sbox_result_1 = S_BOX.apply_enc(key_register[0] >> 4);
            let sbox_result_2 = S_BOX.apply_enc(key_register[0] % 16);
            key_register[0] = (sbox_result_1 << 4) + sbox_result_2;

            // XOR bits 66, ..., 62 with the round counter
            key_register[7] ^= round >> 2;
            key_register[8] ^= round << 6;
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

/// A single round key. Its length is always 64 bit (same as the block size).
#[derive(Copy, Clone)]
pub struct RoundKey {
    /// The value of the round key.
    pub value: u64,
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

    #[test]
    fn test_round_key_generation_128bit() {
        let key = Key128Bit::new([0xA, 0xC0, 0xA6, 0xE7, 0x63, 0x26, 0xBC, 0x7E, 0x82, 0x80, 0x12, 0xAA, 0x5F, 0xDF, 0x39, 0x25]);
        let round_keys = key.generate_round_keys();
        assert_eq!(round_keys[0].value, 0xAC0A6E76326BC7E_u64);
        assert_eq!(round_keys[1].value, 0x7C5002554BFBE724_u64);
        assert_eq!(round_keys[2].value, 0xE42B029B9D8C9AF1_u64);
    }
}
