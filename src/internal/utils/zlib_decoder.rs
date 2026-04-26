use flate2::read::ZlibDecoder;
use std::{fs, io::Read};
use crate::Result;
use crate::error::RGitError;

pub fn zlib_decoder(name:&String)->Result<Vec<u8>>{
    let dir_name = name[0..2].to_string();
    let filename = name[2..].to_string();
    let bytes = fs::read(format!(".rgit/objects/{}/{}",dir_name,filename))
                            .map_err(|e|RGitError::FileDoesNotExist{
                                path:format!(".rgit/objects/{}/{}",dir_name,filename),
                                source:Box::new(e)
                            })?;

    let mut decoder = ZlibDecoder::new(&bytes[..]);
    let mut decompressed_string = Vec::new();
    decoder.read_to_end(&mut decompressed_string)
            .map_err(|e|RGitError::FileReadError { 
                path: format!(".rgit/objects/{}/{}",dir_name,filename),
                source:Box::new(e)
            })?;
    Ok(decompressed_string)
}