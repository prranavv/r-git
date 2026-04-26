use crate::internal::utils::{parse_tree, zlib_decoder};
use crate::Result;

pub fn read_object(name:&String)->Result<(String,String)>{
    let decompressed_bytes = zlib_decoder(name)?;
    let mut pointer = 0;
    let space = decompressed_bytes.iter().position(|x|*x==b' ').unwrap();
    let obj_type = &decompressed_bytes[pointer..space];
    let obj_type = std::str::from_utf8(obj_type).unwrap();
    pointer+=space;
    let null_position= &decompressed_bytes[pointer..].iter().position(|x|*x==b'\0').unwrap();
    pointer+=null_position;
    pointer+=1;
    if obj_type=="blob".to_string(){
        let content= String::from_utf8_lossy(&decompressed_bytes[pointer..]).to_string();
        Ok((obj_type.to_string(),content))
    }else if obj_type=="tree".to_string(){
        let content =parse_tree(&decompressed_bytes);
        Ok((obj_type.to_string(),content))
    }else{
        let content= String::from_utf8_lossy(&decompressed_bytes[pointer..]).to_string();
        Ok((obj_type.to_string(),content))
    }
}