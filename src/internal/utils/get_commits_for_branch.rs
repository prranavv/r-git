use crate::Result;
use crate::error::RGitError;
use crate::internal::utils::get_branch_commit;
use crate::internal::{
    commit_hash::CommitHash,
    utils::{read_object},
};
use std::collections::VecDeque;
use std::ops::Deref;

pub fn get_commits_for_branch(branch_name: &String) -> Result<VecDeque<CommitHash>> {
    let mut result = VecDeque::new();
    let mut head_commit = get_branch_commit(branch_name);
    while let Some(ref h) = head_commit {
        let head = h.deref();
        if head == "" {
            break;
        }
        let (_obj_type, content) = read_object(&head.trim().to_string())?;
        let lines = content.lines();
        result.push_back(CommitHash::new(head.to_string()));
        for line in lines {
            if line.starts_with("parent") {
                let parts: Vec<&str> = line.split(' ').collect();
                let mut iter = parts.iter();
                let _parent = iter.next().ok_or(RGitError::NotCommitHash)?;
                let parent_hash = iter.next();
                if let Some(e) = parent_hash {
                    head_commit = Some(CommitHash::new(e.to_string()));
                } else {
                    head_commit = None;
                }
            } else {
                continue;
            }
        }
    }
    Ok(result)
}
