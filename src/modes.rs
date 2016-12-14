use rand::{Rng, OsRng};
use block::Block;

/// Enum representing block cipher modes of operation.
pub enum OpMode {
    /// Electronic Code Book (unsafe). Does not require an initialization vector.
    ECB,
    /// Cipher Block Chaining. Requires an initialization vector.
    CBC,
}

/// Generate a random initialization vector using a random
/// number generator provided by the operating system.
/// For details on how randomness is achieved, see
/// [the `OsRng` docs](https://doc.rust-lang.org/rand/rand/os/struct.OsRng.html)
/// from the `rand` crate.
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
