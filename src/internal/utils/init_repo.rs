use crate::{Result,RGitError};
use std::fs;

pub fn init_repo()->Result<()>{
    fs::create_dir_all(".rgit/objects")
        .map_err(|e|RGitError::DirectoryCreateError { path: format!(".rgit/objects").into(), source: Box::new(e) })?;
    fs::create_dir_all(".rgit/objects/info")
        .map_err(|e|RGitError::DirectoryCreateError { path: format!("rgit/objects/info").into(), source: Box::new(e) })?;
    fs::create_dir_all(".rgit/objects/pack")
        .map_err(|e|RGitError::DirectoryCreateError { path: format!(".rgit/objects/pack").into(), source: Box::new(e) })?;
    fs::create_dir_all(".rgit/refs/heads")
        .map_err(|e|RGitError::DirectoryCreateError { path: format!(".rgit/refs/heads").into(), source: Box::new(e) })?;
    fs::write(".rgit/HEAD", "ref: refs/heads/main\n")
        .map_err(|e|RGitError::FileWriteError { path: format!(".rgit/HEAD").into(), source: Box::new(e) })?;
    fs::write(".rgit/index", "")
        .map_err(|e|RGitError::FileWriteError { path: format!(".rgit/index").into(), source: Box::new(e) })?;
    Ok(())
}