use crate::error::RGitError;
use crate::Result;

pub fn parse_tree(content:&Vec<u8>)->Result<String>{
    let mut i=0;
    let null_pos = content[i..].iter().position(|x|*x==b'\0').ok_or(RGitError::NotTree)?;
    i+=null_pos;
    i+=1;
    let mut result = String::new();
    while i<content.len(){
        let space = content[i..].iter().position(|x|*x==b' ').ok_or(RGitError::NotTree)?;
        let mode = std::str::from_utf8(&content[i..i+space]).map_err(|e|RGitError::BytesToStringError { source: Box::new(e) })?;
        i+=space;
        i+=1;
        let null_pos = content[i..].iter().position(|x|*x==b'\0').ok_or(RGitError::NotTree)?;
        let file_name = std::str::from_utf8(&content[i..i+null_pos]).map_err(|e|RGitError::BytesToStringError { source: Box::new(e) })?;
        i+=null_pos;
        i+=1;
        let bytes = &content[i..i+20];
        let hex_string =hex::encode(bytes);
        i+=20;
        let obj_type = if mode == "100644"{
            "blob"
        }else{
            "tree"
        };    
        result.push_str(&format!("{} {} {} {}\n",mode,obj_type,hex_string,file_name));
    }
    let result = result.trim_end().to_string();
    Ok(result)
}