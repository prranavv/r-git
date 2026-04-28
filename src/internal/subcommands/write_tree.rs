use crate::internal::utils::{build_tree_from_index, parse_index};
use crate::Result;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn write_tree()->Result<()>{
    let entries = parse_index()?;
    let tree_hash = build_tree_from_index(entries,&Path::new(""))?;
    let hex_string = hex::encode(tree_hash);
    let mut stdout = io::stdout();
    writeln!(stdout,"{}",hex_string.trim())?;
    Ok(())
}