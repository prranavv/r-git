use std::fs;
use crate::{Result, error::RGitError, internal::utils::read_object};

pub fn build_from_tree_hash(tree_hash:String,path:String)->Result<()>{
    let (_obj_type,content) = read_object(&tree_hash)?;
    let lines = content.lines();
    for line in lines{
        let line = line.trim();
        if line.starts_with("40000"){
            let parts: Vec<&str>=line.split(' ').collect();
            let mut iter = parts.iter();
            let _mode = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?;
            let _file_type = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?;
            let tree_hash = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?.to_string();
            let dir_name = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?;
            build_from_tree_hash(tree_hash,format!("{}{}/",path,dir_name))?;
        }else if line.starts_with("100644"){
            let parts: Vec<&str>=line.split(' ').collect();
            let mut iter = parts.iter();
            let _mode = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?;
            let _file_type = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?;
            let blob_hash = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?.to_string();
            let file_name = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.clone() })?;

            let file_path = format!("{}{}",path,file_name);

            let (_obj_type,blob_contents) = read_object(&blob_hash)?;
            fs::write(&file_path, blob_contents).map_err(|e|RGitError::FileWriteError { path: file_path.clone().into(), source: Box::new(e) })?;
        }else{  
            continue;
        }
    }
    Ok(())
}
