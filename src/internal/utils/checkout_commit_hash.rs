use std::path::PathBuf;

use crate::error::RGitError;
use crate::internal::commit_hash::CommitHash;
use crate::internal::utils::{build_from_tree_hash, read_object, remove_cur_dir,build_tree};
use crate::Result;

pub fn checkout_commit_hash(commit_hash:&CommitHash)->Result<()>{
    let (obj_type,content) = read_object(&commit_hash.to_string())?;
    if obj_type!="commit".to_string(){
        return Err(RGitError::NotCommitHash)
    }
    let lines = content.lines();
    for line in lines{
        let line = line.trim();
        if line.starts_with("tree"){
            let parts: Vec<&str>=line.split(' ').collect();
            let mut iter = parts.iter();
            let _parent = iter.next().ok_or(RGitError::NotValidHash { hash_name: commit_hash.to_string() })?;
            let tree_hash = iter.next().ok_or(RGitError::NotValidHash { hash_name: commit_hash.to_string() })?.to_string();
            let cur_tree_hash = build_tree(&PathBuf::from("./"))?;
            let hex_string = hex::encode(cur_tree_hash);
            remove_cur_dir(&hex_string,&format!("./"))?;
            build_from_tree_hash(tree_hash,format!("./"))?;
        }
    }
    Ok(())
}