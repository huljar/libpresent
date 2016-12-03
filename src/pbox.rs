use std::cmp::Ordering;

lazy_static! {
    pub static ref P_BOX: PBox = PBox::new();
}

pub struct PBox {
    // This currently does not require any fields, but to keep it consistent
    // with the SBox implementation, I left it like this
}

impl PBox {
    fn new() -> Self {
		PBox { }
    }

    fn apply<F>(&self, calc_bit: F, input: u64) -> u64
        where F: Fn(u32) -> u32 {

        // Iterate over all input bits, shift to new position, add to result
        let mut output = 0u64;
        for bit in 0..64 {
            let new_bit = calc_bit(bit);
            let bit_value = input & (2u64.pow(bit));
            let new_bit_value = match bit.cmp(&new_bit) {
                Ordering::Less => bit_value << (new_bit - bit),
                Ordering::Equal => bit_value,
                Ordering::Greater => bit_value >> (bit - new_bit),
            };
            output += new_bit_value;
        }
        output
    }

    pub fn apply_enc(&self, input: u64) -> u64 {
        self.apply(|bit: u32| (bit % 4) * 16 + (bit / 4), input)
    }

    pub fn apply_dec(&self, input: u64) -> u64 {
        self.apply(|bit: u32| (bit / 16) + (bit % 16) * 4, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_pbox_gives_correct_outputs() {
        // Test conversion 0x1A6E7639E6166 -> 0xA30079B0FDB1164
        // Binary representation are:
        // 0000000000000001101001101110011101100011100111100110000101100110
        // 0000101000110000000001111001101100001111110110110001000101100100
        //    |60  |55  |50  |45  |40  |35  |30  |25  |20  |15  |10  |5   |0
        assert_eq!(P_BOX.apply_enc(0x1A6E7639E6166_u64), 0xA30079B0FDB1164_u64);
        assert_eq!(P_BOX.apply_dec(0xA30079B0FDB1164_u64), 0x1A6E7639E6166_u64);
    }
}
