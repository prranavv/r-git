use std::fs;

use crate::internal::commit_hash::CommitHash;

pub fn get_branch_commit(branch_name: &String) -> Option<CommitHash> {
    let commit_hash = match fs::read_to_string(format!(".rgit/refs/heads/{}",branch_name.trim())){
        Ok(contents) => contents.trim().to_string(),
        Err(_) => return None,
    };
    let commit_hash = CommitHash::new(commit_hash);
    Some(commit_hash)
}