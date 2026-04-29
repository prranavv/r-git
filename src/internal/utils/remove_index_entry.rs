use std::path::Path;
use std::fs;
use crate::{Result, internal::utils::parse_index};


pub fn remove_index_entry(path:&Path)->Result<()>{
    let mut index_entries = parse_index()?;
    let mut result = String::new();
    index_entries.retain(|e|e.file_path!=path);
    for i in index_entries{
        let str_entry:String= i.into();
        result.push_str(&str_entry);
        result.push_str("\n");
    }
    let result = result.trim();

    let index_path = format!(".rgit/index");
    fs::write(index_path, result)?;
    Ok(())
}