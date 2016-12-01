use std::collections::BTreeMap;

lazy_static! {
    pub static ref S_BOX: SBox = SBox::new();
}

pub struct SBox {
    s_map: BTreeMap<u8, u8>,
}

impl SBox {
    fn new() -> Self {
        let mut tmp_map = BTreeMap::new();
        tmp_map.insert(0, 12);
        tmp_map.insert(1, 5);
        tmp_map.insert(2, 6);
        tmp_map.insert(3, 11);
        tmp_map.insert(4, 9);
        tmp_map.insert(5, 0);
        tmp_map.insert(6, 10);
        tmp_map.insert(7, 13);
        tmp_map.insert(8, 3);
        tmp_map.insert(9, 14);
        tmp_map.insert(10, 15);
        tmp_map.insert(11, 8);
        tmp_map.insert(12, 4);
        tmp_map.insert(13, 7);
        tmp_map.insert(14, 1);
        tmp_map.insert(15, 2);

        SBox {
            s_map: tmp_map,
        }
    }

    pub fn apply(&self, input: u8) -> u8 {
        match self.s_map.get(&input) {
            Some(&output) => output,
            None => panic!("Logic error! Invalid S-Box input!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_sbox_gives_correct_outputs() {
        assert_eq!(S_BOX.apply(0), 12);
        assert_eq!(S_BOX.apply(1), 5);
        assert_eq!(S_BOX.apply(2), 6);
        assert_eq!(S_BOX.apply(3), 11);
        assert_eq!(S_BOX.apply(4), 9);
        assert_eq!(S_BOX.apply(5), 0);
        assert_eq!(S_BOX.apply(6), 10);
        assert_eq!(S_BOX.apply(7), 13);
        assert_eq!(S_BOX.apply(8), 3);
        assert_eq!(S_BOX.apply(9), 14);
        assert_eq!(S_BOX.apply(10), 15);
        assert_eq!(S_BOX.apply(11), 8);
        assert_eq!(S_BOX.apply(12), 4);
        assert_eq!(S_BOX.apply(13), 7);
        assert_eq!(S_BOX.apply(14), 1);
        assert_eq!(S_BOX.apply(15), 2);
    }

    #[test]
    #[should_panic]
    fn test_that_invalid_input_panics() {
        S_BOX.apply(16);
    }
}
