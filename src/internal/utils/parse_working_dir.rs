use std::path::PathBuf;

use crate::internal::index_entry::IndexEntry;
use crate::Result;
use crate::internal::utils::{build_from_head, build_tree};

pub fn parse_working_dir()->Result<Vec<IndexEntry>>{
    let tree_hash = build_tree(&PathBuf::from("./"))?;
    let hex_string = hex::encode(tree_hash);
    let result = build_from_head(hex_string, format!(""))?;
    Ok(result)
}