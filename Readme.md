# Hashbrown

> Crack SHA-256 hashes using a wordlist with performance and precision.

---

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](#)
[![License](https://img.shields.io/badge/license-MIT-blue)](#)
[![Rust Version](https://img.shields.io/badge/rust-2021-orange)](#)
[![Crates.io](https://img.shields.io/badge/crate-unpublished-lightgrey)](#)
[![Security](https://img.shields.io/badge/security-hardened-critical)](#)

---

## Overview

**Hashbrown** is a high-performance command-line utility written in Rust, designed to brute-force crack SHA-256 hashes using a user-supplied wordlist. Built for reliability, clarity, and speed, Hashbrown is perfect for forensic use cases, CTFs, or pentesting pipelines.

---

## Features

- Minimal dependencies
- Fast in-memory lookup via `HashMap`
- Clear error messages for invalid input
- Clean CLI interface using [`clap`](https://docs.rs/clap)
- Input validation for hash format and length

---

## Installation

```bash
git clone https://github.com/yourname/hashbrown.git
cd hashbrown
cargo build --release
```

The binary will be located at:

```
target/release/hashbrown
```

---

## Usage

```bash
./hashbrown --wordlist <path_to_wordlist.txt> --hash <sha256_hash>
```

### Example:

```bash
./hashbrown --wordlist rockyou.txt --hash e3afed0047b08059d0fada10f400c1e5b0fbe4a80f1c68ab88b5e60b0bc4e8f2
```

### Output:

```
[o] Hash cracked: password123
```

If the hash is not found:

```
[x] Failed to crack hash, get a better wordlist!
```

---

## Project Structure

```
src/
├── main.rs       # Entry point
Cargo.toml        # Dependencies and metadata
```

---

## Dependencies

* [`clap`](https://docs.rs/clap)
* [`sha2`](https://docs.rs/sha2)
* [`hex`](https://docs.rs/hex)

---

## License

No license lmao what do you take me for?

---

## Author

**Hashbrown** is just a bite-sized project for a bootcamp i made, let's see if i maintain it further and make it a fully fledged hash cracker.
