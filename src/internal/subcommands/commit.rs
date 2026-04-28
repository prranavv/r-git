use crate::internal::commit_tree;
use crate::internal::utils::{build_tree_from_index, parse_index};
use crate::Result;
use std::path::Path;

pub fn commit(message:&String)->Result<()>{
    let entries = parse_index()?;
    let tree_hash = build_tree_from_index(entries,&Path::new(""))?;
    let hex_string = hex::encode(tree_hash);

    commit_tree(&hex_string, message)?;
    Ok(())
}