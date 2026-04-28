use std::fs;
use crate::{Result, internal::index_entry::IndexEntry};

pub fn update_index_contents(index_contents:String,index_entry:String)->Result<()>{
    let lines = index_contents.lines();
    let new_entry = IndexEntry::try_from(index_entry)?;
    let mut result = String::new();
    for line in lines{
        let line_entry = IndexEntry::try_from(line.to_string())?;
        if line_entry.file_path==new_entry.file_path{
            continue;
        }
        let str_line_entry:String = line_entry.into();
        result.push_str(&str_line_entry);
        result.push_str("\n");
    }
    result.push_str(&new_entry.to_string());
    let result =result.trim_end();

    let index_path = format!(".rgit/index");
    fs::write(index_path, result)?;
    Ok(())
}