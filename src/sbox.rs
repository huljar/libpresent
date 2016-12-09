use std::collections::BTreeMap;

lazy_static! {
    pub static ref S_BOX: SBox = SBox::new();
}

pub struct SBox {
    s_map_enc: BTreeMap<u8, u8>,
    s_map_dec: BTreeMap<u8, u8>,
}

impl SBox {
    fn new() -> Self {
        let mut tmp_map_enc = BTreeMap::new();
        tmp_map_enc.insert(0, 12);
        tmp_map_enc.insert(1, 5);
        tmp_map_enc.insert(2, 6);
        tmp_map_enc.insert(3, 11);
        tmp_map_enc.insert(4, 9);
        tmp_map_enc.insert(5, 0);
        tmp_map_enc.insert(6, 10);
        tmp_map_enc.insert(7, 13);
        tmp_map_enc.insert(8, 3);
        tmp_map_enc.insert(9, 14);
        tmp_map_enc.insert(10, 15);
        tmp_map_enc.insert(11, 8);
        tmp_map_enc.insert(12, 4);
        tmp_map_enc.insert(13, 7);
        tmp_map_enc.insert(14, 1);
        tmp_map_enc.insert(15, 2);

        let mut tmp_map_dec = BTreeMap::new();
        tmp_map_dec.insert(0, 5);
        tmp_map_dec.insert(1, 14);
        tmp_map_dec.insert(2, 15);
        tmp_map_dec.insert(3, 8);
        tmp_map_dec.insert(4, 12);
        tmp_map_dec.insert(5, 1);
        tmp_map_dec.insert(6, 2);
        tmp_map_dec.insert(7, 13);
        tmp_map_dec.insert(8, 11);
        tmp_map_dec.insert(9, 4);
        tmp_map_dec.insert(10, 6);
        tmp_map_dec.insert(11, 3);
        tmp_map_dec.insert(12, 0);
        tmp_map_dec.insert(13, 7);
        tmp_map_dec.insert(14, 9);
        tmp_map_dec.insert(15, 10);

        SBox {
            s_map_enc: tmp_map_enc,
            s_map_dec: tmp_map_dec
        }
    }

    pub fn apply_enc(&self, input: u8) -> u8 {
        *self.s_map_enc.get(&input).expect("Logic error! Invalid S-Box input! (enc)")
    }

    pub fn apply_dec(&self, input: u8) -> u8 {
        *self.s_map_dec.get(&input).expect("Logic error! Invalid S-Box input! (dec)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_sbox_gives_correct_outputs() {
        assert_eq!(S_BOX.apply_enc(0), 12);
        assert_eq!(S_BOX.apply_enc(1), 5);
        assert_eq!(S_BOX.apply_enc(2), 6);
        assert_eq!(S_BOX.apply_enc(3), 11);
        assert_eq!(S_BOX.apply_enc(4), 9);
        assert_eq!(S_BOX.apply_enc(5), 0);
        assert_eq!(S_BOX.apply_enc(6), 10);
        assert_eq!(S_BOX.apply_enc(7), 13);
        assert_eq!(S_BOX.apply_enc(8), 3);
        assert_eq!(S_BOX.apply_enc(9), 14);
        assert_eq!(S_BOX.apply_enc(10), 15);
        assert_eq!(S_BOX.apply_enc(11), 8);
        assert_eq!(S_BOX.apply_enc(12), 4);
        assert_eq!(S_BOX.apply_enc(13), 7);
        assert_eq!(S_BOX.apply_enc(14), 1);
        assert_eq!(S_BOX.apply_enc(15), 2);
    }

    #[test]
    fn test_that_inverse_sbox_gives_correct_outputs() {
        assert_eq!(S_BOX.apply_dec(0), 5);
        assert_eq!(S_BOX.apply_dec(1), 14);
        assert_eq!(S_BOX.apply_dec(2), 15);
        assert_eq!(S_BOX.apply_dec(3), 8);
        assert_eq!(S_BOX.apply_dec(4), 12);
        assert_eq!(S_BOX.apply_dec(5), 1);
        assert_eq!(S_BOX.apply_dec(6), 2);
        assert_eq!(S_BOX.apply_dec(7), 13);
        assert_eq!(S_BOX.apply_dec(8), 11);
        assert_eq!(S_BOX.apply_dec(9), 4);
        assert_eq!(S_BOX.apply_dec(10), 6);
        assert_eq!(S_BOX.apply_dec(11), 3);
        assert_eq!(S_BOX.apply_dec(12), 0);
        assert_eq!(S_BOX.apply_dec(13), 7);
        assert_eq!(S_BOX.apply_dec(14), 9);
        assert_eq!(S_BOX.apply_dec(15), 10);
    }

    #[test]
    #[should_panic]
    fn test_that_invalid_input_panics() {
        S_BOX.apply_enc(16);
    }

    #[test]
    #[should_panic]
    fn test_that_invalid_input_panics_inverse() {
        S_BOX.apply_dec(42);
    }
}
