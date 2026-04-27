use std::fs;
use std::io::Write;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use crate::Result;
use crate::RGitError;

pub fn zlib_encoder(content:Vec<u8>,dirname:&String,filename:&String)->Result<()>{
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&content)
        .map_err(|e|RGitError::FileWriteError { 
            path:format!(".rgit/objects/{}/{}",dirname,filename) , 
            source: Box::new(e) 
        })?;
    let compressed_bytes = encoder.finish()
                                            .map_err(|e|RGitError::FileWriteError { 
                                                path:format!(".rgit/objects/{}/{}",dirname,filename) , 
                                                source: Box::new(e) 
                                            })?;
    fs::create_dir_all(format!(".rgit/objects/{}",dirname))
            .map_err(|e|RGitError::DirectoryCreateError { 
                    path:format!(".rgit/objects/{}/{}",dirname,filename) , 
                    source: Box::new(e)  
                })?;
    
    fs::write(format!(".rgit/objects/{}/{}",dirname,filename), compressed_bytes)
        .map_err(|e|RGitError::FileWriteError { 
            path:format!(".rgit/objects/{}/{}",dirname,filename) , 
            source: Box::new(e) 
        })?;
    Ok(())
}