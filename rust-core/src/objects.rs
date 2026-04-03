////////////////////////////////////////////////////////////////////////////////
/// @file       objects.rs
/// @author     Krisna Pranav
/// @date       April 3, 2026
/// @license    MIT License
/// @copyright  Copyright (c) 2026 Krisna Pranav
////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};
use crate::hash::hash_bytes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blob {
    pub hash: String,
    pub size: usize,
    pub content: Vec<u8>,
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Self {
        let hash = hash_bytes(&content);
        let size = content.len();
        Self { hash, size, content }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeEntry {
    pub name: String,
    pub hash: String,
    pub object_type: ObjectType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    pub hash: String,
    pub children: Vec<TreeEntry>,
}

impl Tree {
    pub fn new(children: Vec<TreeEntry>) -> Self {
        let serialized = serde_json::to_vec(&children).expect("tree serialization");
        let hash = hash_bytes(&serialized);
        Self { hash, children }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub hash: String,
    pub parent_hashes: Vec<String>,
    pub root_tree_hash: String,
    pub author: String,
    pub message: String,
    pub signature: String,
    pub timestamp: i64,
}

impl Commit {
    pub fn new(
        parent_hashes: Vec<String>,
        root_tree_hash: String,
        author: String,
        message: String,
        signature: String,
        timestamp: i64,
    ) -> Self {
        let payload = format!(
            "{}|{}|{}|{}|{}",
            parent_hashes.join(","),
            root_tree_hash,
            author,
            message,
            timestamp
        );
        let hash = hash_bytes(payload.as_bytes());
        Self {
            hash,
            parent_hashes,
            root_tree_hash,
            author,
            message,
            signature,
            timestamp,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ref {
    pub name: String,
    pub commit_hash: String,
}