use std::fs;
use crate::error::RGitError;
use crate::internal::entry::Entry;
use crate::internal::utils::{build_entries, hash_content, zlib_encoder};
use crate::internal::Mode;
use sha1::{digest::{array::Array, consts::{B0, B1}, typenum::{UInt, UTerm}}};
use crate::Result;

pub fn build_tree(path:&str)->Result<Array<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>>>{
    let mut entries:Vec<Entry> = Vec::new();

    for entry in fs::read_dir(path)
                                            .map_err(|e|RGitError::DirectoryReadError { path: path.to_string(), source: Box::new(e) })?{
        let entry = entry.map_err(|e|RGitError::DirectoryEntryError { path: path.to_string(), source: Box::new(e) })?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|e|RGitError::Io(e))?;
        let file_name = entry.file_name().into_string().unwrap();
        if file_name==".rgit".to_string() || file_name ==".git".to_string() || file_name =="target".to_string(){
            continue;
        }
        if file_type.is_dir() && !fs::read_dir(path.to_str().unwrap())?.next().is_none(){
            let tree_hash = build_tree(path.to_str().unwrap())?;
            let tree_entry = Entry::new(Mode::Directory, file_name, tree_hash);
            entries.push(tree_entry);
        }else if file_type.is_file(){
            let contents=fs::read_to_string(&path)
                                                    .map_err(|e|RGitError::FileReadError { path: format!("{}/{}",path.to_string_lossy(),file_name), source: Box::new(e) })?;
            
            let blob_contents = contents.as_bytes();
            let header = format!("blob {}\0",blob_contents.len());
            let mut store = Vec::new();
            store.extend_from_slice(header.as_bytes());
            store.extend_from_slice(blob_contents);
            let (dirname,filename,result) = hash_content(&store)?;
            zlib_encoder(store.to_vec(), &dirname, &filename)?;
            let file_entry = Entry::new(Mode::File, file_name, result);
            entries.push(file_entry);
        }else{
            continue;
        }   
    }
    entries.sort_by(|a,b| a.file_name.as_bytes().cmp(&b.file_name.as_bytes()));

    let entries_bytes = build_entries(&entries);
    let header = format!("tree {}\0",entries_bytes.len());
    let mut store = Vec::new();
    store.extend_from_slice(header.as_bytes());
    store.extend_from_slice(&entries_bytes);
    let (dirname,filename,result) = hash_content(&store)?;
    zlib_encoder(store, &dirname, &filename)?;
    Ok(result)
}