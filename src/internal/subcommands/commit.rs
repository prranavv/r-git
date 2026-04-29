use crate::internal::commit_tree;
use crate::internal::utils::{add_all_to_index, build_tree_from_index, parse_index};
use crate::Result;
use std::path::{Path, PathBuf};

pub fn commit(message:&String,all:bool)->Result<()>{
    if all{
        add_all_to_index(&PathBuf::from("."))?;
    }
    let entries = parse_index()?;
    let tree_hash = build_tree_from_index(entries,&Path::new(""))?;
    let hex_string = hex::encode(tree_hash);

    commit_tree(&hex_string, message)?;
    Ok(())
}