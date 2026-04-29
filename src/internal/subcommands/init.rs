use std::{fs, path::PathBuf};

use crate::{Result, internal::utils::init_repo};

pub fn init()->Result<()>{
    if PathBuf::from(".rgit/").exists(){
        fs::remove_dir_all(PathBuf::from(".rgit/"))?;
    }
    init_repo()?;
    Ok(())
}