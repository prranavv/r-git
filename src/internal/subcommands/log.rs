use std::io;
use std::io::Write;

use crate::{Result, internal::utils::{parse_commit_history}};

pub fn log()->Result<()>{
    let result  = parse_commit_history()?;
    let mut stdout = io::stdout();
    writeln!(stdout,"{}",result.trim())?;
    Ok(())
}