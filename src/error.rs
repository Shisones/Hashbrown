use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashbrownError {
    #[error("[x] IO error: {0}")]
    Io(#[from] io::Error),

    #[error("[x] Invalid hex string: {0}")]
    HexDecode(#[from] hex::FromHexError),

    #[error("[x] Invalid hash length for {algorithm}: expected {expected} bytes, got {got}")]
    InvalidHashLength {
        algorithm: String,
        expected: usize,
        got: usize,
    },

    #[error("[x] Failed to crack hash with provided wordlist")]
    CrackFailure,

    #[allow(dead_code)]
    #[error("[x] Unsupported hash algorithm: {0}")]
    UnsupportedAlgorithm(String),
}

pub type HashbrownResult<T> = Result<T, HashbrownError>;
