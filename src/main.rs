use std::collections::HashMap;
use std::fs;
use sha2::{Sha256, Digest};
use hex;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    wordlist: String,

    #[arg(long)]
    hash: String,
}

fn main() {
    let cli = Cli::parse();

    let wordlist_content = fs::read_to_string(&cli.wordlist)
        .expect("[x] Failed to read wordlist file. Please ensure the path is correct and the file exists.");

    let mut hash_to_plaintext: HashMap<Vec<u8>, String> = HashMap::new();

    for line in wordlist_content.lines() {
        let mut hasher = Sha256::new();
        hasher.update(line.as_bytes());
        let hash_bytes = hasher.finalize().to_vec();
        hash_to_plaintext.insert(hash_bytes, line.to_string());
    }

    let search_hash_str = cli.hash.trim();

    let search_hash_bytes = match hex::decode(search_hash_str) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("[x] Invalid SHA256 hash format. Please enter a valid hexadecimal string.");
            return;
        }
    };

    if search_hash_bytes.len() != 32 {
        println!("[x] Invalid SHA256 hash length. A SHA256 hash should be 64 hexadecimal characters long (32 bytes).");
        return;
    }

    if let Some(plaintext) = hash_to_plaintext.get(&search_hash_bytes) {
        println!("[o] Hash cracked: {}", plaintext);
    } else {
        println!("[x] Failed to crack hash, get a better wordlist!");
    }
}
