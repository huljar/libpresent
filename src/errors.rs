use std::convert::From;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum DecryptError {
    Utf8Error,
    CiphertextNotAligned(usize),
    CiphertextTooShort(usize),
    InvalidPadding,
    InitVecMissing,
}

impl From<FromUtf8Error> for DecryptError {
    fn from(_: FromUtf8Error) -> Self {
        DecryptError::Utf8Error
    }
}
