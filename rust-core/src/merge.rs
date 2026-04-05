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
    pub merged_entries: Vec<TreeEntry>,
    pub conflicts: Vec<MergeConflict>,
}

#[derive(Debug)]
pub struct MergeConflict {
    pub path: String,
    pub ours: String,
    pub theirs: String,
}

pub fn three_way_merge(
    base: &[TreeEntry],
    ours: &[TreeEntry],
    theirs: &[TreeEntry],
) -> MergeResult {
    let base_map: HashMap<_, _> = base.iter().map(|e| (e.name.as_str(), e)).collect();
    let ours_map: HashMap<_, _> = ours.iter().map(|e| (e.name.as_str(), e)).collect();
    let theirs_map: HashMap<_, _> = theirs.iter().map(|e| (e.name.as_str(), e)).collect();

    let mut all_names: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for e in base.iter().chain(ours.iter()).chain(theirs.iter()) {
        all_names.insert(e.name.as_str());
    }

    let mut merged_entries = Vec::new();
    let mut conflicts = Vec::new();

    for name in &all_names {
        let b = base_map.get(name);
        let o = ours_map.get(name);
        let t = theirs_map.get(name);

        match (b, o, t) {
            (Some(_), None, None) => {}

            (None, Some(o_entry), None) => merged_entries.push((*o_entry).clone()),
            (Some(_), Some(o_entry), None) => {}
            (None, None, Some(t_entry)) => merged_entries.push((*t_entry).clone()),

            (_, Some(o_entry), Some(t_entry)) => {
                if o_entry.hash == t_entry.hash {
                    merged_entries.push((*o_entry).clone());
                } else {
                    conflicts.push(MergeConflict {
                        path: name.to_string(),
                        ours: o_entry.hash.clone(),
                        theirs: t_entry.hash.clone(),
                    });
                }
            }

            (Some(b_entry), None, Some(t_entry)) => {
                if b_entry.hash == t_entry.hash {
                } else {
                    conflicts.push(MergeConflict {
                        path: name.to_string(),
                        ours: "DELETED".to_string(),
                        theirs: t_entry.hash.clone(),
                    });
                }
            }

            _ => {}
        }
    }

    MergeResult { merged_entries, conflicts }
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
        let base = vec![entry("a.rs", "hash_a"), entry("b.rs", "hash_b")];
        let ours = vec![entry("a.rs", "hash_a2"), entry("b.rs", "hash_b")];
        let theirs = vec![entry("a.rs", "hash_a"), entry("b.rs", "hash_b2")];

        let result = three_way_merge(&base, &ours, &theirs);
        assert_eq!(result.conflicts.len(), 0);
        assert_eq!(result.merged_entries.len(), 2);
    }

    #[test]
    fn test_conflict_detection() {
        let base = vec![entry("a.rs", "hash_a")];
        let ours = vec![entry("a.rs", "hash_a_ours")];
        let theirs = vec![entry("a.rs", "hash_a_theirs")];

        let result = three_way_merge(&base, &ours, &theirs);
        assert_eq!(result.conflicts.len(), 1);
        assert_eq!(result.conflicts[0].path, "a.rs");
    }

    #[test]
    fn test_addition_both_sides() {
        let base = vec![];
        let ours = vec![entry("new.rs", "hash_new")];
        let theirs = vec![entry("new.rs", "hash_new")];

        let result = three_way_merge(&base, &ours, &theirs);
        assert_eq!(result.conflicts.len(), 0);
        assert_eq!(result.merged_entries.len(), 1);
    }
}