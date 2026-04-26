use std::{fs, str::FromStr};
use crate::{Result, error::RGitError};

pub fn init()->Result<()>{
    fs::create_dir_all(".rgit/objects")
        .map_err(|e|RGitError::DirectoryCreateError { path: String::from_str(".rgit/objects").unwrap(), source: Box::new(e) })?;
    fs::create_dir_all(".rgit/objects/info")
        .map_err(|e|RGitError::DirectoryCreateError { path: String::from_str(".rgit/objects/info").unwrap(), source: Box::new(e) })?;
    fs::create_dir_all(".rgit/objects/pack")
        .map_err(|e|RGitError::DirectoryCreateError { path: String::from_str(".rgit/objects/pack").unwrap(), source: Box::new(e) })?;
    fs::create_dir_all(".rgit/refs/heads")
        .map_err(|e|RGitError::DirectoryCreateError { path: String::from_str(".rgit/refs/heads").unwrap(), source: Box::new(e) })?;
    fs::write(".rgit/HEAD", "ref: refs/heads/main\n")
        .map_err(|e|RGitError::FileWriteError { path: String::from_str(".rgit/HEAD").unwrap(), source: Box::new(e) })?;
    Ok(())
}