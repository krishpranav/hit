////////////////////////////////////////////////////////////////////////////////
/// @file       dag.rs
/// @author     Krisna Pranav
/// @date       April 3, 2026
/// @license    MIT License
/// @copyright  Copyright (c) 2026 Krisna Pranav
///
////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use crate::objects::{Blob, Commit, ObjectType, Ref, Tree, TreeEntry};
use crate::error::{HitError};
use crate::hash::verify_hash;

#[derive(Debug, Default)]
pub struct DagStore {
    blobs: HashMap<String, Blob>,
    trees: HashMap<String, Tree>,
    commits: HashMap<String, Commit>,
    refs: HashMap<String, Ref>,
}

impl DagStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_blob(&mut self, blob: Blob) -> Result<String, HitError> {
        if !verify_hash(&blob.content, &blob.hash) {
            return Err(HitError::HashMismatch {
                expected: blob.hash.clone(),
                actual: crate::hash::hash_bytes(&blob.content),
            });
        }

        let hash =  blob.hash.clone();
        self.blobs.insert(hash.clone(), blob);
        Ok(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::{Blob, Commit, ObjectType, Ref, Tree, TreeEntry};

    fn make_store_with_commit() -> (DagStore, String) {
        let mut store = DagStore::new();
        let blob = Blob::new(b"fn main() {}".to_vec());
        let blob_hash = store.insert_blob(blob.clone()).unwrap();

        let tree = Tree::new(vec![TreeEntry {
            name: "main.rs".into(),
            hash: blob_hash.clone(),
            object_type: ObjectType::Blob,
        }])
    }
}