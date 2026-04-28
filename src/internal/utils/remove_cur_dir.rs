use std::{fs};
use crate::error::RGitError;
use crate::internal::utils::read_object;
use crate::Result;

pub fn remove_cur_dir(tree_hash:&String,path:&String)->Result<()>{
    let (_obj_type,content) = read_object(&tree_hash)?;
    let lines = content.lines();
    for line in lines{
        let line = line.trim();
        if line.starts_with("40000"){
            let parts: Vec<&str>=line.split(' ').collect();
            let mut iter = parts.iter();
            let _mode = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.to_string() })?;
            let _file_type = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.to_string() })?;
            let tree_hash = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.to_string() })?.to_string();
            let dir_name = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.to_string() })?;
            remove_cur_dir(&tree_hash,&format!("{}{}/",path,dir_name))?;
        }else if line.starts_with("100644"){
            let parts: Vec<&str>=line.split(' ').collect();
            let mut iter = parts.iter();
            let _mode = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.to_string() })?;
            let _file_type = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.to_string() })?;
            let _blob_hash = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.to_string() })?;
            let file_name = iter.next().ok_or(RGitError::NotValidHash { hash_name: tree_hash.to_string() })?;

            let file_path = format!("{}{}",path,file_name);
            fs::remove_file(file_path).map_err(|e|RGitError::Io(e))?;
        }else{  
            continue;
        }
    }
    Ok(())
}