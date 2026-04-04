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
use crate::error::HitError;
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
        let hash = blob.hash.clone();
        self.blobs.insert(hash.clone(), blob);
        Ok(hash)
    }

    pub fn get_blob(&self, hash: &str) -> Result<&Blob, HitError> {
        self.blobs.get(hash).ok_or_else(|| HitError::ObjectNotFound {
            hash: hash.to_string(),
        })
    }

    pub fn insert_tree(&mut self, tree: Tree) -> Result<String, HitError> {
        self.validate_tree(&tree)?;
        let hash = tree.hash.clone();
        self.trees.insert(hash.clone(), tree);
        Ok(hash)
    }

    pub fn get_tree(&self, hash: &str) -> Result<&Tree, HitError> {
        self.trees.get(hash).ok_or_else(|| HitError::ObjectNotFound {
            hash: hash.to_string(),
        })
    }

    fn validate_tree(&self, tree: &Tree) -> Result<(), HitError> {
        for entry in &tree.children {
            match entry.object_type {
                ObjectType::Blob => {
                    if !self.blobs.contains_key(&entry.hash) {
                        return Err(HitError::DagValidationFailed {
                            reason: format!("blob {} referenced but not stored", entry.hash),
                        });
                    }
                }
                ObjectType::Tree => {
                    if !self.trees.contains_key(&entry.hash) {
                        return Err(HitError::DagValidationFailed {
                            reason: format!("tree {} referenced but not stored", entry.hash),
                        });
                    }
                }
                ObjectType::Commit => {
                    return Err(HitError::DagValidationFailed {
                        reason: "commits cannot be children of a tree".to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn insert_commit(&mut self, commit: Commit) -> Result<String, HitError> {
        if !self.trees.contains_key(&commit.root_tree_hash) {
            return Err(HitError::DagValidationFailed {
                reason: format!("root tree {} not found", commit.root_tree_hash),
            });
        }

        for parent in &commit.parent_hashes {
            if !self.commits.contains_key(parent) {
                return Err(HitError::DagValidationFailed {
                    reason: format!("parent commit {} not found", parent),
                });
            }
        }
        let hash = commit.hash.clone();
        self.commits.insert(hash.clone(), commit);
        Ok(hash)
    }

    pub fn get_commit(&self, hash: &str) -> Result<&Commit, HitError> {
        self.commits.get(hash).ok_or_else(|| HitError::ObjectNotFound {
            hash: hash.to_string(),
        })
    }

    pub fn set_ref(&mut self, r: Ref) {
        self.refs.insert(r.name.clone(), r);
    }

    pub fn get_ref(&self, name: &str) -> Result<&Ref, HitError> {
        self.refs.get(name).ok_or_else(|| HitError::ObjectNotFound {
            hash: name.to_string(),
        })
    }

    pub fn commit_history(&self, start: &str) -> Result<Vec<&Commit>, HitError> {
        let mut history = Vec::new();
        let mut current = start.to_string();
        loop {
            let commit = self.get_commit(&current)?;
            history.push(commit);
            if commit.parent_hashes.is_empty() {
                break;
            }
            current = commit.parent_hashes[0].clone();
        }
        Ok(history)
    }

    pub fn reachable_objects(&self, commit_hash: &str) -> Result<Vec<String>, HitError> {
        let mut reachable = Vec::new();
        let commit = self.get_commit(commit_hash)?;
        reachable.push(commit.hash.clone());
        self.collect_tree_objects(&commit.root_tree_hash, &mut reachable)?;
        Ok(reachable)
    }

    fn collect_tree_objects(
        &self,
        tree_hash: &str,
        acc: &mut Vec<String>,
    ) -> Result<(), HitError> {
        let tree = self.get_tree(tree_hash)?;
        acc.push(tree.hash.clone());
        for entry in &tree.children {
            match entry.object_type {
                ObjectType::Blob => acc.push(entry.hash.clone()),
                ObjectType::Tree => self.collect_tree_objects(&entry.hash, acc)?,
                ObjectType::Commit => {}
            }
        }
        Ok(())
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
        }]);
        store.insert_tree(tree.clone()).unwrap();

        let commit = Commit::new(
            vec![],
            tree.hash.clone(),
            "0xAuthor".into(),
            "Initial commit".into(),
            "sig_placeholder".into(),
            1_700_000_000,
        );
        let ch = store.insert_commit(commit).unwrap();
        (store, ch)
    }

    #[test]
    fn test_insert_and_retrieve_blob() {
        let mut store = DagStore::new();
        let blob = Blob::new(b"hello".to_vec());
        let hash = store.insert_blob(blob.clone()).unwrap();
        assert_eq!(store.get_blob(&hash).unwrap().size, 5);
    }

    #[test]
    fn test_dag_validation_missing_blob() {
        let mut store = DagStore::new();
        let tree = Tree::new(vec![]);

        let bad_tree = Tree {
            hash: "fake".into(),
            children: vec![TreeEntry {
                name: "x".into(),
                hash: "nonexistent".into(),
                object_type: ObjectType::Blob,
            }],
        };
        assert!(store.insert_tree(bad_tree).is_err());
    }

    #[test]
    fn test_commit_history() {
        let (store, ch) = make_store_with_commit();
        let history = store.commit_history(&ch).unwrap();
        assert_eq!(history.len(), 1);
    }

    #[test]
    fn test_reachable_objects() {
        let (store, ch) = make_store_with_commit();
        let objs = store.reachable_objects(&ch).unwrap();
        assert!(objs.len() >= 3);
    }
}