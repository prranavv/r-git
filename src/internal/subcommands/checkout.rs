use crate::{Result, internal::utils::{checkout_branch, checkout_commit_hash}};
use std::{fs, path::PathBuf};
use std::io;
use std::io::Write;
use crate::RGitError;

pub fn checkout(commit_hash:&String)->Result<()>{
    let files = fs::read_dir("./.rgit/refs/heads")?;
    for file in files{
        let file = file?;
        let file_name = file.file_name().into_string().unwrap();
        if file_name == commit_hash.to_string(){
            return checkout_branch(commit_hash);
        }
    }
    checkout_commit_hash(commit_hash)?;
    let mut stdout = io::stdout();
    writeln!(stdout,"You are in 'detached HEAD' state.")?;

    fs::write(format!(".rgit/HEAD"), format!("{}",commit_hash))
        .map_err(|e|RGitError::FileWriteError { path:PathBuf::from(format!(".rgit/HEAD")), source: Box::new(e) })?;
    Ok(())
}