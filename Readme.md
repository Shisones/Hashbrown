# Hashbrown

> Simple hash brute-forcer using rust.

---

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](#)
[![License](https://img.shields.io/badge/license-MIT-blue)](#)
[![Rust Version](https://img.shields.io/badge/rust-2021-orange)](#)
[![Crates.io](https://img.shields.io/badge/crate-unpublished-lightgrey)](#)
[![Security](https://img.shields.io/badge/security-educational-yellow)](#)

---

## Overview

**Hashbrown** is a command-line hash cracking utility written in Rust, designed for speed, reliability, and extensibility. It brute-forces hashes using a wordlist, supporting multiple algorithms (SHA-256, SHA-1, MD5, SHA-512) with a modular design that makes adding new algorithms a breeze. Perfect for CTFs, pentesting, or learning about hash cracking in a safe, educational context.

---

## Features

- **Multi-Algorithm Support**: Crack SHA-256, SHA-1, MD5, and SHA-512 hashes.
- **High Performance**: Parallel processing with `rayon` for fast cracking.
- **Robust Error Handling**: Clear, user-friendly error messages for invalid inputs.
- **Clean CLI**: Powered by `clap` for intuitive command-line usage.
- **Logging**: Configurable `info` level logging for tracking progress.
- **Extensible**: Trait-based design for easy addition of new hash algorithms.

---

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed (`cargo` and `rustc`).
2. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/hashbrown.git
   cd hashbrown
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

The binary will be located at:
```
target/release/hashbrown
```

---

## Usage

```bash
./hashbrown --wordlist <path_to_wordlist> --hash <hex_encoded_hash> --algorithm <sha256|sha1|md5|sha512>
```

### Examples

#### Crack a SHA-256 Hash
```bash
./hashbrown --wordlist examples/wordlist.txt --hash 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824 --algorithm sha256
```
**Output**:
```
[2025-06-07T16:41:00Z INFO  hashbrown] Starting Hashbrown hash cracker
[2025-06-07T16:41:00Z INFO  hashbrown::cracker] Reading wordlist from examples/wordlist.txt
[2025-06-07T16:41:00Z INFO  hashbrown::cracker] Cracking hash using SHA256 algorithm
[2025-06-07T16:41:00Z INFO  hashbrown::cracker] Hash cracked successfully
[o] Hash cracked: hello
```

#### Crack a SHA-512 Hash
```bash
./hashbrown --wordlist examples/wordlist.txt --hash 9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043 --algorithm sha512
```
**Output**:
```
[2025-06-07T16:41:00Z INFO  hashbrown] Starting Hashbrown hash cracker
[2025-06-07T16:41:00Z INFO  hashbrown::cracker] Reading wordlist from examples/wordlist.txt
[2025-06-07T16:41:00Z INFO  hashbrown::cracker] Cracking hash using SHA512 algorithm
[2025-06-07T16:41:00Z INFO  hashbrown::cracker] Hash cracked successfully
[o] Hash cracked: hello
```

#### Failed Attempt
If the hash isn't found:
```
[2025-06-07T16:41:00Z INFO  hashbrown] Starting Hashbrown hash cracker
[2025-06-07T16:41:00Z INFO  hashbrown::cracker] Reading wordlist from examples/wordlist.txt
[2025-06-07T16:41:00Z INFO  hashbrown::cracker] Cracking hash using SHA256 algorithm
[2025-06-07T16:41:00Z ERROR hashbrown::cracker] No matching plaintext found in wordlist
[x] Failed to crack hash, try a better wordlist!
```

---

## Project Structure

```
hashbrown/
├── Cargo.toml          # Dependencies and metadata
├── src/
│   ├── main.rs        # Entry point
│   ├── cli.rs         # CLI parsing
│   ├── cracker.rs     # Hash cracking logic
│   ├── error.rs       # Custom error handling
│   ├── utils.rs       # File and hex utilities
│   └── hash/
│       ├── mod.rs     # Hash module declarations
│       ├── algorithm.rs # HashAlgorithm trait
│       ├── sha256.rs   # SHA-256 implementation
│       ├── sha1.rs     # SHA-1 implementation
│       ├── md5.rs      # MD5 implementation
│       └── sha512.rs   # SHA-512 implementation
├── tests/
│   └── integration.rs # Integration tests
└── examples/
    └── wordlist.txt   # Sample wordlist
```

---

## Dependencies

- [`clap`](https://docs.rs/clap): Command-line argument parsing.
- [`sha2`](https://docs.rs/sha2): SHA-256 and SHA-512 implementations.
- [`sha1`](https://docs.rs/sha1): SHA-1 implementation.
- [`md-5`](https://docs.rs/md-5): MD5 implementation.
- [`hex`](https://docs.rs/hex): Hex encoding/decoding.
- [`rayon`](https://docs.rs/rayon): Parallel processing.
- [`log`](https://docs.rs/log): Logging framework.
- [`env_logger`](https://docs.rs/env_logger): Info-level logging.
- [`thiserror`](https://docs.rs/thiserror): Custom error types.

---

## Adding New Algorithms

To add a new hash algorithm (e.g., SHA3):
1. Create a new file in `src/hash/` (e.g., `sha3.rs`):
   ```rust
   use sha3::{Digest, Sha3_256};
   use super::algorithm::HashAlgorithm;

   pub struct Sha3Algorithm;

   impl HashAlgorithm for Sha3Algorithm {
       fn name(&self) -> &'static str {
           "SHA3_256"
       }

       fn hash(&self, input: &[u8]) -> Vec<u8> {
           let mut hasher = Sha3_256::new();
           hasher.update(input);
           hasher.finalize().to_vec()
       }

       fn expected_length(&self) -> usize {
           32
       }
   }
   ```
2. Update `src/hash/mod.rs` to include:
   ```rust
   pub use sha3::Sha3Algorithm;
   ```
3. Add `Sha3` to `HashAlgorithmType` in `src/hash/algorithm.rs`:
   ```rust
   #[derive(Clone, Debug, PartialEq, ValueEnum)]
   pub enum HashAlgorithmType {
       Sha256,
       Sha1,
       Md5,
       Sha512,
       Sha3,
   }
   ```
4. Update `Cracker::new` in `src/cracker.rs`:
   ```rust
   HashAlgorithmType::Sha3 => Box::new(Sha3Algorithm),
   ```
5. Add the `sha3` dependency to `Cargo.toml`:
   ```toml
   sha3 = "0.10"
   ```

---

## Development

- **Run Tests**: `cargo test`
- **Format Code**: `cargo fmt`
- **Lint Code**: `cargo clippy`
- **Debug Logging**: Set `RUST_LOG=debug` for verbose output (default is `info`).

---

## Security Note

MD5 and SHA-1 are cryptographically broken and should not be used in production systems for security-critical applications. Hashbrown is intended for educational purposes, CTFs, or legacy system analysis. See [RFC6151](https://tools
