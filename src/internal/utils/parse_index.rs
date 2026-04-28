use crate::internal::index_entry::IndexEntry;
use std::fs;

pub fn parse_index()->Vec<IndexEntry>{
    let contents = fs::read_to_string(".rgit/index").unwrap();
    let lines = contents.lines();
    let mut result = Vec::new();
    for line in lines {
        let index_entry = IndexEntry::from(line);
        result.push(index_entry);
    }
    result
}
