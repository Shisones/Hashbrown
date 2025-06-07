use sha1::{Digest, Sha1};

use super::algorithm::HashAlgorithm;

pub struct Sha1Algorithm;

impl HashAlgorithm for Sha1Algorithm {
    fn name(&self) -> &'static str {
        "SHA1"
    }

    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Sha1::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    fn expected_length(&self) -> usize {
        20
    }
}
