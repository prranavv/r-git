use crate::internal::utils::read_object;
use crate::Result;
use std::io;
use std::io::Write;

pub fn cat_file(name:&String,type_of:bool,pretty_print:bool)->Result<()>{
    if type_of&& pretty_print{
        return Err(crate::error::RGitError::CantPrettyPrintAndType)
    }
    let (obj_type,content) = read_object(name)?;
    let mut stdout = io::stdout();
    if type_of{
        writeln!(stdout,"{}",obj_type)?;
        return Ok(())
    }
    writeln!(stdout,"{}",content.trim())?;
    Ok(())
}