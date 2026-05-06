use std::{fs, path::PathBuf};

pub fn get_head_branch_name()->Option<String>{
    let head = fs::read_to_string(".rgit/HEAD").ok()?;
    let head_path = PathBuf::from(head);
    let x = head_path.file_name().unwrap().to_str().unwrap().trim().to_string();
    Some(x)
}