use crate::internal::index_entry::IndexEntry;
use std::fs;
use crate::Result;

pub fn parse_index()->Result<Vec<IndexEntry>>{
    let contents = fs::read_to_string(".rgit/index").unwrap();
    let lines = contents.lines();
    let mut result = Vec::new();
    for line in lines {
        let index_entry = IndexEntry::try_from(line)?;
        result.push(index_entry);
    }
    Ok(result)
}
