use crate::{RGitError, Result, internal::utils::{hash_content, update_index_contents}};
use std::fs;

pub fn add_all_to_index(file_path:&String)->Result<()>{

    for entry in fs::read_dir(&file_path).unwrap(){
        let entry = entry.unwrap();
        let path = entry.path();
        let file_type = entry.file_type().unwrap();
        let file_name = entry.file_name().into_string().unwrap();
        if file_name==".rgit".to_string() || file_name ==".git".to_string() || file_name =="target".to_string(){
            continue;
        }
        if file_type.is_dir() && !fs::read_dir(path.to_str().unwrap())?.next().is_none(){
            add_all_to_index(&path.to_string_lossy().to_string())?;
        }else if file_type.is_file(){
            let contents=fs::read_to_string(&path)
                        .map_err(|e|RGitError::FileReadError { path: file_path.to_string(), source: Box::new(e) })?;                

            let blob_contents = contents.as_bytes();
            let header = format!("blob {}\0",blob_contents.len());
            let mut store = Vec::new();
            store.extend_from_slice(header.as_bytes());
            store.extend_from_slice(blob_contents);

            let (dirname,filename,_result) = hash_content(&store)?;
            let hex_string =format!("{}{}",dirname,filename);
            let index_path = format!(".rgit/index");
            let abs_path = &path.to_str().unwrap()[2..];
            let index_entry = format!("100644 {} {}\n",abs_path,hex_string);
            let index_contents = fs::read_to_string(&index_path)?;
            update_index_contents(index_contents,index_entry)?;
        }else{
            continue;
        }
    }
    Ok(())
}