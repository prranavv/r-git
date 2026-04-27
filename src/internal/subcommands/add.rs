use crate::{RGitError, Result, internal::utils::{hash_content, update_index_contents}};
use std::fs;

pub fn add(file_path: &String)->Result<()>{
    let contents=fs::read_to_string(&file_path)
                            .map_err(|e|RGitError::FileReadError { path: file_path.to_string(), source: Box::new(e) })?;                

    let blob_contents = contents.as_bytes();
    let header = format!("blob {}\0",blob_contents.len());
    let mut store = Vec::new();
    store.extend_from_slice(header.as_bytes());
    store.extend_from_slice(blob_contents);

    let (dirname,filename,_result) = hash_content(&store)?;
    let hex_string =format!("{}{}",dirname,filename);

    let index_path = format!(".rgit/index");
    let index_entry = format!("100644 {} {}\n",file_path,hex_string);
    let index_contents = fs::read_to_string(&index_path)?;
    
    update_index_contents(index_contents,index_entry)?;
    
    Ok(())
}