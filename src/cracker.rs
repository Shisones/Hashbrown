use log::{error, info};
use rayon::prelude::*;
use std::collections::HashMap;

use crate::{
    cli::Cli,
    error::{HashbrownError, HashbrownResult},
    hash::{HashAlgorithm, HashAlgorithmType, Md5Algorithm, Sha1Algorithm, Sha256Algorithm, Sha512Algorithm},
    utils,
};

pub struct Cracker {
    wordlist: String,
    target_hash: Vec<u8>,
    algorithm: Box<dyn HashAlgorithm + Send + Sync>,
}

impl Cracker {
    pub fn new(cli: &Cli) -> HashbrownResult<Self> {
        let target_hash = utils::decode_hex(&cli.hash)?;
        let algorithm: Box<dyn HashAlgorithm + Send + Sync> = match cli.algorithm {
            HashAlgorithmType::Sha256 => Box::new(Sha256Algorithm),
            HashAlgorithmType::Sha1 => Box::new(Sha1Algorithm),
            HashAlgorithmType::Md5 => Box::new(Md5Algorithm),
            HashAlgorithmType::Sha512 => Box::new(Sha512Algorithm),
        };

        if target_hash.len() != algorithm.expected_length() {
            return Err(HashbrownError::InvalidHashLength {
                algorithm: cli.algorithm.to_string(),
                expected: algorithm.expected_length(),
                got: target_hash.len(),
            });
        }

        Ok(Cracker {
            wordlist: cli.wordlist.clone(),
            target_hash,
            algorithm,
        })
    }

    pub fn crack(&mut self) -> HashbrownResult<()> {
        info!("[i] Reading wordlist from {}", self.wordlist);
        let wordlist_content = utils::read_file(&self.wordlist)?;

        info!("[i] Cracking hash using {} algorithm", self.algorithm.name());
        let result = wordlist_content
            .par_lines()
            .find_any(|line| {
                let hash = self.algorithm.hash(line.as_bytes());
                hash == self.target_hash
            });

        match result {
            Some(plaintext) => {
                info!("[i] Hash cracked successfully");
                println!("[o] Hash cracked: {}", plaintext);
                Ok(())
            }
            None => {
                error!("[x] No matching plaintext found in wordlist");
                println!("[x] Failed to crack hash, try a better wordlist!");
                Err(HashbrownError::CrackFailure)
            }
        }
    }
}
