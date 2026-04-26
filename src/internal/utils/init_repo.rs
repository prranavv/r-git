use crate::{Result,RGitError};
use std::fs;

pub fn init_repo()->Result<()>{
    fs::create_dir_all(".rgit/objects")
        .map_err(|e|RGitError::DirectoryCreateError { path: format!(".rgit/objects"), source: Box::new(e) })?;
    fs::create_dir_all(".rgit/objects/info")
        .map_err(|e|RGitError::DirectoryCreateError { path: format!("rgit/objects/info"), source: Box::new(e) })?;
    fs::create_dir_all(".rgit/objects/pack")
        .map_err(|e|RGitError::DirectoryCreateError { path: format!(".rgit/objects/pack"), source: Box::new(e) })?;
    fs::create_dir_all(".rgit/refs/heads")
        .map_err(|e|RGitError::DirectoryCreateError { path: format!(".rgit/refs/heads"), source: Box::new(e) })?;
    fs::write(".rgit/HEAD", "ref: refs/heads/main\n")
        .map_err(|e|RGitError::FileWriteError { path: format!(".rgit/HEAD"), source: Box::new(e) })?;
    fs::write(".rgit/index", "")
        .map_err(|e|RGitError::FileWriteError { path: format!(".rgit/index"), source: Box::new(e) })?;
    Ok(())
}