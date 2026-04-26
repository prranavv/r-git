use std::{fs};

use crate::internal::utils::get_head_branch;

pub fn get_parent_hash()->Option<String>{
    let head = get_head_branch()?;
    let parent_hash =match fs::read_to_string(format!(".rgit/{}",head)){
        Ok(r)=>r,
        Err(_)=>return None
    };
    Some(parent_hash.to_string())
}