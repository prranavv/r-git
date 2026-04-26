use sha1::{Digest, Sha1, digest::{array::Array, consts::{B0, B1}, typenum::{UInt, UTerm}}};
use crate::Result;

pub fn hash_content(store:&Vec<u8>)->Result<(String,String,Array<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>>)>{
    let mut hasher = Sha1::new();
    hasher.update(&store);
    let result = hasher.finalize();
    let hex_string = hex::encode(result);
    let dirname = hex_string[0..2].to_string();
    let filename = hex_string[2..].to_string();
    Ok((dirname,filename,result))
}