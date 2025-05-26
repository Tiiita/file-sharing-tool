use std::{collections::{HashMap, HashSet}, path::PathBuf};

#[derive(Debug)]
enum ChangeType {
    Created(PathBuf),
    Deleted(PathBuf),
    Modified(PathBuf),
    Moved { from: PathBuf, to: PathBuf },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FileInfo {
    path: PathBuf,
    size: u64,
    hash: String,
}

fn compare_snapshots(
    old_files: &[FileInfo], 
    new_files: &[FileInfo]
) -> Vec<ChangeType> {
    let old_map: HashMap<_, _> = old_files.iter().map(|f| (f.path.clone(), f)).collect();
    let new_map: HashMap<_, _> = new_files.iter().map(|f| (f.path.clone(), f)).collect();

    let mut old_hash_map: HashMap<&str, Vec<&FileInfo>> = HashMap::new();
    for f in old_files {
        old_hash_map.entry(&f.hash[..]).or_default().push(f);
    }

    let mut changes = Vec::new(); 
    let mut matched_old_paths = HashSet::new();
    let mut matched_new_paths = HashSet::new();

    for (path, old_f) in &old_map {
        if let Some(new_f) = new_map.get(path) {
            if old_f.hash != new_f.hash {
                changes.push(ChangeType::Modified(path.clone()));
            }
            matched_old_paths.insert(path.clone());
            matched_new_paths.insert(path.clone());
        }
    }

    for new_f in new_files {
        if matched_new_paths.contains(&new_f.path) {
            continue;
        }
        if let Some(old_candidates) = old_hash_map.get(&new_f.hash[..]) {
            for old_f in old_candidates {
                if !matched_old_paths.contains(&old_f.path) && old_f.path != new_f.path {
                    changes.push(ChangeType::Moved { from: old_f.path.clone(), to: new_f.path.clone() });
                    matched_old_paths.insert(old_f.path.clone());
                    matched_new_paths.insert(new_f.path.clone());
                    break;
                }
            }
        }
    }

    for new_f in new_files {
        if !matched_new_paths.contains(&new_f.path) {
            changes.push(ChangeType::Created(new_f.path.clone()));
        }
    }

    for old_f in old_files {
        if !matched_old_paths.contains(&old_f.path) {
            changes.push(ChangeType::Deleted(old_f.path.clone()));
        }
    }

    changes
}