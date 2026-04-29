use crate::error::RGitError;
use crate::internal::index_entry::IndexEntry;
use crate::{Result, internal::{utils::{build_from_head, get_parent_hash, read_object}}};

pub fn parse_head()->Result<Vec<IndexEntry>>{
    let head_commit = get_parent_hash();
    let mut result = Vec::new();
    if let Some(ref h)=head_commit{
        let head = h.trim();
        let (_obj_type,content) = read_object(&head.trim().to_string())?;
        let lines = content.lines();
        
        for line in lines{
            if line.trim().is_empty(){
                continue;
            }else if line.starts_with("tree"){
                let parts:Vec<&str> = line.split(' ').collect();
                let mut iter = parts.iter();
                let _tree = iter.next().ok_or(RGitError::NotValidIndexEntry { index_entry: h.clone() })?.trim();
                let tree_hash = iter.next().ok_or(RGitError::NotValidIndexEntry { index_entry: h.clone() })?.trim();
                result = build_from_head(tree_hash.to_string(), format!(""))?;
            }else{
                continue;
            }
        }
    }
    Ok(result)
}


