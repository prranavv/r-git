use crate::internal::utils::{build_tree};
use crate::Result;
use std::io;
use std::io::Write;

pub fn write_tree()->Result<()>{
    let tree_hash = build_tree("./")?;
    let hex_string = hex::encode(tree_hash);
    let mut stdout = io::stdout();
    writeln!(stdout,"{}",hex_string.trim())?;
    Ok(())
}