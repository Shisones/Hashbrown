use sha2::{Digest, Sha256};

use super::algorithm::HashAlgorithm;

pub struct Sha256Algorithm;

impl HashAlgorithm for Sha256Algorithm {
    fn name(&self) -> &'static str {
        "SHA256"
    }

    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    fn expected_length(&self) -> usize {
        32
    }
}
