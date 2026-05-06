use crate::{Result, internal::utils::get_commits_for_branch};

pub fn merge(branch_name:&String)->Result<()>{
    let commits = get_commits_for_branch(branch_name)?;
    for c in commits{
        println!("{}",c.to_string());
    }
    Ok(())
}