use std::fs;

pub fn get_head_branch()->Option<String>{
    let head = fs::read_to_string(".rgit/HEAD").ok()?;
    return Some(head.trim()[5..].to_string())
}