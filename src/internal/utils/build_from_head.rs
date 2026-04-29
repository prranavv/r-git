use crate::internal::Mode;
use crate::internal::{index_entry::IndexEntry, utils::read_object};
use crate::{Result,RGitError};

pub fn build_from_head(tree_hash:String,path:String)->Result<Vec<IndexEntry>>{
    let mut result = Vec::new();
    let (_obj_type,tree_content) = read_object(&tree_hash.to_string())?;
    let tree_lines = tree_content.lines();
    for tree_line in tree_lines{
        let tree_line = tree_line.trim();
        if tree_line.starts_with("40000"){
            let parts: Vec<&str>=tree_line.split(' ').collect();
            let mut iter = parts.iter();
            let _mode = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?;
            let _file_type = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?;
            let dir_tree_hash = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?.to_string();
            let dir_name = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?;
            let indexes = build_from_head(dir_tree_hash, format!("{}{}/",path,dir_name))?;
            result.extend_from_slice(&indexes);
        }else if tree_line.starts_with("100644"){
            let parts: Vec<&str>=tree_line.split(' ').collect();
            let mut iter = parts.iter();
            let mode = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?;
            let _file_type = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?;
            let blob_hash = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?.to_string();
            let file_name = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?;
            let mode = Mode::from(*mode);
            let file_path = format!("{}{}",path,file_name);
            let index_entry = IndexEntry::new(mode, file_path.into(), blob_hash);
            result.push(index_entry);
        }else {
            continue
        }
    }
    Ok(result)
}