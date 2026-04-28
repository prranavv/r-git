use crate::Result;
use crate::internal::Mode;
use crate::internal::entry::Entry;
use crate::internal::index_entry::IndexEntry;
use crate::internal::utils::{build_entries, hash_content, zlib_encoder};
use std::collections::{HashSet};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use sha1::{digest::{array::Array, consts::{B0, B1}, typenum::{UInt, UTerm}}};

pub fn build_tree_from_index(entries: Vec<IndexEntry>, dir: &Path) -> Result<Array<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>>> {
    let mut files: Vec<(String, &IndexEntry)> = Vec::new();
    let mut dirs: HashSet<String> = HashSet::new();

    for entry in &entries {
        let path = entry.file_path.as_path();

        if !dir.as_os_str().is_empty() && !path.starts_with(dir) {
            continue;
        }

        let rest = if dir.as_os_str().is_empty() {
            path
        } else {
            path.strip_prefix(dir).unwrap()
        };
        
        let mut components = rest.components();

        let first = match components.next() {
            Some(c) => c.as_os_str().to_str().unwrap(),
            None => continue,
        };

        if components.next().is_some() {
            dirs.insert(first.to_string());
        } else {
            files.push((first.to_string(), &entry));
        }
    }

    let mut tree_entries: Vec<Entry> = Vec::new();

    for (name, entry) in files {
        let hash_bytes = hex::decode(&entry.hash).unwrap();

        let hash: [u8; 20] = hash_bytes
            .try_into()
            .unwrap();

        tree_entries.push(Entry::new(
            entry.mode,
            name.into(),
            hash.into(),
        ));
    }
    
    for subdir in dirs {
        let new_dir: PathBuf = dir.join(&subdir);
        let hash = build_tree_from_index(entries.clone(), &new_dir)?;
        tree_entries.push(Entry::new(
            Mode::Directory,
            subdir.into(),
            hash,
        ));
    }

    tree_entries.sort_by(|a, b| {
        a.file_name.as_bytes().cmp(b.file_name.as_bytes())
    });

    let entries_bytes = build_entries(&tree_entries);

    let header = format!("tree {}\0", entries_bytes.len());

    let mut store = Vec::new();
    store.extend_from_slice(header.as_bytes());
    store.extend_from_slice(&entries_bytes);

    let (dir_name, file_name, hash) = hash_content(&store)?;
    zlib_encoder(store, &dir_name, &file_name)?;

    Ok(hash)
}