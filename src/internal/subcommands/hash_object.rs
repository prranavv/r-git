use std::fs;
use std::path::PathBuf;
use crate::Result;
use crate::error::RGitError;
use crate::internal::utils::{hash_content, zlib_encoder};
use std::io;
use std::io::Write;

pub fn hash_object(name:&String,write:&bool,stdin:&bool)->Result<()>{
    let contents = if *stdin{
        name.to_owned()
    }else{
    let contents=fs::read_to_string(&name)
                            .map_err(|e|RGitError::FileReadError { path: PathBuf::from(name), source: Box::new(e) })?;                
        contents
    };

    let blob_contents = contents.as_bytes();
    let header = format!("blob {}\0",blob_contents.len());
    let mut store = Vec::new();
    store.extend_from_slice(header.as_bytes());
    store.extend_from_slice(blob_contents);

    let (dirname,filename,_result) = hash_content(&store)?;
    let hex_string =format!("{}{}",dirname,filename);
    let mut stdout = io::stdout();
    writeln!(stdout,"{}",hex_string.trim())?;

    if *write{
        zlib_encoder(store.to_vec(), &dirname, &filename)?
    };
    Ok(())
}