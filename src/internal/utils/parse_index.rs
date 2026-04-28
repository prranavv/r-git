use crate::error::RGitError;
use crate::internal::index_entry::IndexEntry;
use crate::Result;
use std::fs;
use std::path::PathBuf;

pub fn parse_index()->Result<Vec<IndexEntry>>{
    let contents = fs::read_to_string(".rgit/index")
                        .map_err(|e|RGitError::FileReadError { path: PathBuf::from(".rgit/index"), source: Box::new(e) })?;
    let lines = contents.lines();
    let mut result = Vec::new();
    for line in lines {
        let index_entry = IndexEntry::try_from(line)?;
        result.push(index_entry);
    }
    Ok(result)
}
