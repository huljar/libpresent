use std::convert::From;
use std::string::FromUtf8Error;

/// Error type describing string decryption errors.
#[derive(Debug)]
pub enum DecryptError {
    /// Indicates that the decrypted bytes cannot be converted
    /// to a valid UTF-8-encoded string.
    Utf8Error,
    /// Indicates that the ciphertext length is not a multiple
    /// of the block size. Includes the length of the given
    /// ciphertext.
    CiphertextNotAligned(usize),
    /// Indicates that the ciphertext is too short (i.e. less
    /// than one block). Includes the length of the given
    /// ciphertext.
    CiphertextTooShort(usize),
    /// Indicates that the padding bytes at the end of the string
    /// are invalid or corrupted.
    InvalidPadding,
    /// When using an operation mode that requires an initialization
    /// vector (pretty much all except ECB), this indicates that
    /// the IV was not given in the function arguments.
    InitVecMissing,
}

impl From<FromUtf8Error> for DecryptError {
    /// Convert string encoding error to the corresponding DecryptError.
    fn from(_: FromUtf8Error) -> Self {
        DecryptError::Utf8Error
    }
}
