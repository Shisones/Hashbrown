use sha2::{Digest, Sha512};
use super::algorithm::HashAlgorithm;

pub struct Sha512Algorithm;

impl HashAlgorithm for Sha512Algorithm {
    fn name(&self) -> &'static str {
        "SHA512"
    }

    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    fn expected_length(&self) -> usize {
        64
    }
}
