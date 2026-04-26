use crate::{Result, internal::utils::read_tree};
use std::io;
use std::io::Write;

pub fn ls_tree(name:&String)->Result<()>{
    let content = read_tree(name)?;
    let mut stdout = io::stdout();
    writeln!(stdout,"{}",content.trim())?;

    Ok(())
}