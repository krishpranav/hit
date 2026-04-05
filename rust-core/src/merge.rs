////////////////////////////////////////////////////////////////////////////////
/// @file       merge.rs
/// @author     Krisna Pranav
/// @date       April 5, 2026
/// @license    MIT License
/// @copyright  Copyright (c) 2026 Krisna Pranav
///
////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use crate::objects::{ObjectType, Tree, TreeEntry};
use crate::error::HitError;

#[derive(Debug)]
pub struct MergeResult {
    pub merged_entires: Vec<TreeEntry>,
    pub conflicts: Vec<MergeConflit>,
}

#[derive(Debug)]
pub struct MergeConflit {
    pub path: String,
    pub ours: String,
    pub theirs: String,
}

pub fn three_way_merge(
    base: &[TreeEntry],
    ours: &[TreeEntry],
    theirs: &[TreeEntry],
) -> MergeResult {
    MergeResult { merged_entires, conflicts }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::ObjectType;

    fn entry(name: &str, hash: &str) -> TreeEntry {
        TreeEntry {
            name: name.to_string(),
            hash: hash.to_string(),
            object_type: ObjectType::Blob,
        }
    }

    #[test]
    fn test_clean_merge() {
    }

    #[test]
    fn test_conflict_detection() {
        
    }
}