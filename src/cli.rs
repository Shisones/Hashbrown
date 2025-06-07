use clap::{Parser, ValueEnum};
use crate::hash::HashAlgorithmType;

#[derive(Parser, Debug)]
#[command(name = "hashbrown", version = "v.0.1.0", about = "A simple hash cracking tool", long_about = None)]
pub struct Cli {
    #[arg(long, help = "Path to the wordlist file")]
    pub wordlist: String,

    #[arg(long, help = "Hash to crack (hex-encoded)")]
    pub hash: String,

    #[arg(long, value_enum, default_value_t = HashAlgorithmType::Sha256, help = "Hash algorithm to use")]
    pub algorithm: HashAlgorithmType,
}
