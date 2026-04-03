////////////////////////////////////////////////////////////////////////////////
/// @file       hash.rs
/// @author     Krisna Pranav
/// @date       April 3, 2026
/// @license    MIT License
/// @copyright  Copyright (c) 2026 Krisna Pranav
///
/// @brief      Cryptographic hashing functionality for the hit core library.
///
/// @details    This module implements cryptographic hashing algorithms and
///             utilities for the hit project. It provides secure hash generation,
///             validation, and comparison operations. The module supports various
///             hashing algorithms and delivers high-performance hash computations
///             suitable for integrity verification and secure data operations.
///
////////////////////////////////////////////////////////////////////////////////

use sha2::{Digest, Sha256};

pub fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

pub fn hash_str(data: &str) -> String {
    hash_bytes(data.as_bytes())
}

pub fn verify_hash(data: &[u8], expected: &str) -> bool {
    hash_bytes(data) == expected
}