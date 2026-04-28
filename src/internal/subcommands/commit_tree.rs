use std::fs;
use std::path::PathBuf;
use crate::RGitError;
use crate::internal::utils::get_head_branch;
use crate::{Result, internal::utils::{commit_message, hash_content, zlib_encoder}};
use std::io;
use std::io::Write;

pub fn commit_tree(tree_hash:&String,message:&String)->Result<()>{
    let commit_content = commit_message(tree_hash, message);

    let bytes = commit_content.as_bytes();
    let header = format!("commit {}\0",bytes.len());

    let mut store =Vec::new();
    store.extend_from_slice(header.as_bytes());
    store.extend_from_slice(bytes);

    let (dirname,filename,_result) = hash_content(&store)?;
    zlib_encoder(store, &dirname, &filename)?;

    let hex_string = format!("{}{}",dirname,filename);
    let mut stdout = io::stdout();
    writeln!(stdout,"{}",hex_string.trim())?;

    let head = get_head_branch().unwrap();
    fs::write(format!(".rgit/{}",head), format!("{}\n",hex_string))
        .map_err(|e|RGitError::FileWriteError { path: PathBuf::from(format!(".rgit/{}",head)), source: Box::new(e) })?;
    Ok(())
}