use std::{fs, path::PathBuf};

use crate::{Result, internal::utils::remove_index_entry};

pub fn rm(file_path:&PathBuf)->Result<()>{
    let file_path = file_path.strip_prefix("./").unwrap_or(file_path);
    remove_index_entry(file_path)?;
    fs::remove_file(file_path)?;
    Ok(())
}