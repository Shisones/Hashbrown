use std::fs;

use crate::error::{HashbrownError, HashbrownResult};

pub fn read_file(path: &str) -> HashbrownResult<String> {
    fs::read_to_string(path).map_err(HashbrownError::Io)
}

pub fn decode_hex(hex_str: &str) -> HashbrownResult<Vec<u8>> {
    hex::decode(hex_str.trim()).map_err(HashbrownError::HexDecode)
}
