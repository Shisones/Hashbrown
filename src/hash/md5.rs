use md5::{Digest, Md5};

use super::algorithm::HashAlgorithm;

pub struct Md5Algorithm;

impl HashAlgorithm for Md5Algorithm {
    fn name(&self) -> &'static str {
        "MD5"
    }

    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Md5::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    fn expected_length(&self) -> usize {
        16
    }
}
