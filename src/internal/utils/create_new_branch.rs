use crate::Result;
use crate::RGitError;
use crate::internal::utils::get_parent_hash;
use std::fs;

pub fn create_new_branch(branch_name:String)->Result<()>{
    let head_commit = get_parent_hash().unwrap().trim().to_string();
    fs::write(format!(".rgit/refs/heads/{}",branch_name), head_commit)
        .map_err(|e|RGitError::FileWriteError { path: format!(".rgit/refs/heads/{}",branch_name), source: Box::new(e) })?;
    Ok(())
}