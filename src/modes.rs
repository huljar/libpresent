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
