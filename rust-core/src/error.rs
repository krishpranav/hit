////////////////////////////////////////////////////////////////////////////////
/// @file       error.rs
/// @author     Krisna Pranav
/// @date       April 3, 2026
/// @license    MIT License
/// @copyright  Copyright (c) 2026 Krisna Pranav
///
/// @brief      Error handling and type definitions for the hit core library.
///
/// @details    This module provides a comprehensive error handling framework
///             for the hit project. It defines custom error types, error kinds,
///             and utilities for robust error management and reporting across
///             the core library. All errors implement the standard error trait
///             for seamless integration with Rust's error handling ecosystem.
///
////////////////////////////////////////////////////////////////////////////////

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HitError {
    #[error("Object not found: {hash}")]
    ObjectNotFound { hash: String },

    #[error("Invalid hash: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },

    #[error("DAG validation failed: {reason}")]
    DagValidationFailed { reason: String },

    #[error("Merge conflict at path: {path}")]
    MergeConflict { path: String },

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Signature verification failed")]
    SignatureInvalid,

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}
