#[macro_use]
extern crate lazy_static;

mod block;
mod keys;
mod sbox;
mod pbox;

pub use self::keys::{Key, Key80Bit, Key128Bit};
