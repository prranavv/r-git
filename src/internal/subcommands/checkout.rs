use crate::{Result, internal::utils::checkout_commit_hash};

pub fn checkout(commit_hash:&String)->Result<()>{
    checkout_commit_hash(commit_hash)?;
    
    Ok(())
}