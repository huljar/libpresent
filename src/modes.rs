use rand::{Rng, OsRng};
use block::Block;

pub enum OpMode {
    ECB,
    CBC,
}

pub fn random_iv() -> Block {
    let mut rng = match OsRng::new() {
        Ok(g) => g,
        Err(e) => panic!("Unable to obtain RNG from OS: {}", e),
    };

    Block::new(rng.gen())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_that_ivs_are_actually_random() {
        let a = random_iv();
        let b = random_iv();
        assert_eq!(a.get_state(), b.get_state());
    }
}
