use std::{fs, io, path::PathBuf};
use crate::{Result, error::RGitError, internal::utils::{create_new_branch, get_head_branch}};
use std::io::Write;

pub fn branch(branch_name:Option<String>,a:bool)->Result<()>{
    if a & branch_name.is_some(){
        return Err(RGitError::NotValidArgForBranch)
    }
    if a{
        let mut stdout = io::stdout();
        let files = fs::read_dir("./.rgit/refs/heads")?;
        let mut result = String::new();
        let cur_head = get_head_branch().unwrap_or("main".to_string());
        let cur_head = PathBuf::from(cur_head);
        let cur_head= cur_head.file_name().unwrap();
        for file in files{
            let file = file?;
            let file_name =file.file_name();
            if cur_head==file_name{
                result.push_str(&format!("{}*\n",file_name.display()));
            }else{
                result.push_str(&format!("{}\n",file_name.display()));
            }
        }
        
        writeln!(stdout,"{}",result.trim())?;
        return Ok(());
    }
    if branch_name.is_none(){
        return Err(RGitError::NeedToHaveBranchName)
    }
    let files = fs::read_dir("./.rgit/refs/heads")?;
    for file in files{
        let file = file?;
        let file_name_2 = file.file_name().into_string().unwrap();
        if let Some(ref file_name) = branch_name{
            if file_name_2==*file_name{
                return Err(RGitError::BranchAlreadyExists { branch_name: file_name.clone() })
            }
        }
    }
    create_new_branch(branch_name.unwrap())?;
    Ok(())
}