use clap::ValueEnum;
use std::fmt;

pub trait HashAlgorithm {
    fn name(&self) -> &'static str;
    fn hash(&self, input: &[u8]) -> Vec<u8>;
    fn expected_length(&self) -> usize;
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum HashAlgorithmType {
    Sha256,
    Sha1,
    Md5,
    Sha512,
}

impl fmt::Display for HashAlgorithmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sha256 => write!(f, "SHA256"),
            Self::Sha1 => write!(f, "SHA1"),
            Self::Md5 => write!(f, "MD5"),
            Self::Sha512 => write!(f, "SHA512"),
        }
    }
}
