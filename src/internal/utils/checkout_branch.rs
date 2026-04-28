use crate::{Result, internal::utils::{checkout_commit_hash, get_parent_hash}};
use std::fs;
use crate::RGitError;

pub fn checkout_branch(branch_name:&String)->Result<()>{
    fs::write(format!(".rgit/HEAD"), format!("ref: refs/heads/{}\n",branch_name))
        .map_err(|e|RGitError::FileWriteError { path: format!(".rgit/HEAD").into(), source: Box::new(e) })?;


    let head_commit = get_parent_hash().unwrap().trim().to_string();
    checkout_commit_hash(&head_commit)?;
    Ok(())
}