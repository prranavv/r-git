use crate::error::RGitError;
use crate::internal::utils::{parse_tree, zlib_decoder};
use crate::Result;

pub fn read_tree(name:&String)->Result<String>{
    let decompressed_bytes =zlib_decoder(name)?;

    let pointer = 0;
    let space = decompressed_bytes.iter().position(|x|*x==b' ').ok_or(RGitError::NotValidHash { hash_name: name.to_string() })?;
    let obj_type = &decompressed_bytes[pointer..space];
    let obj_type = std::str::from_utf8(obj_type)
                                    .map_err(|e|RGitError::BytesToStringError { source: Box::new(e) })?;
    if obj_type=="tree".to_string(){
        let content = parse_tree(&decompressed_bytes);
        Ok(content)
    }else{
        Err(RGitError::NotTree)
    }
}