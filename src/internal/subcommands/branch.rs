use std::fs;

use crate::{Result, error::RGitError, internal::utils::create_new_branch};

pub fn branch(branch_name:String)->Result<()>{
    let files = fs::read_dir("./.rgit/refs/heads")?;
    for file in files{
        let file = file?;
        let file_name = file.file_name().into_string().unwrap();
        if file_name == branch_name{
            return Err(RGitError::BranchAlreadyExists { branch_name: file_name })
        }
    }
    create_new_branch(branch_name)?;
    Ok(())
}