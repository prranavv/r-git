use chrono::DateTime;
use crossterm::style::{StyledContent, Stylize};
use crate::{Result, error::RGitError, internal::utils::{get_parent_hash, read_object}};

pub fn parse_commit_history()->Result<Vec<StyledContent<String>>>{
    let mut head_commit = get_parent_hash();
    let mut result: Vec<StyledContent<String>> = Vec::new();
    while let Some(ref h)=head_commit{
        let head =h.trim();
        if head==""{
            break;
        }    
        let (_obj_type,content) = read_object(&head.trim().to_string())?;
        let lines = content.lines();
        let commit_hash_line = format!("commit {}",head).yellow();
        result.push(commit_hash_line);
        for line in lines{
            if line.trim().is_empty(){
                continue;
            }
            else if line.starts_with("tree"){
                continue
            }else if line.starts_with("\n"){

                continue;
            }
            else if line.starts_with("parent"){
                let parts:Vec<&str> = line.split(' ').collect();
                let mut iter = parts.iter();
                let _parent = iter.next().ok_or(RGitError::NotCommitHash)?;
                let parent_hash = iter.next();
                if let Some(e)=parent_hash{
                    head_commit=Some(e.to_string());
                }else{
                    head_commit=None;
                }
            }
            else if line.starts_with("author"){
                let parts: Vec<&str> = line.split(' ').collect();
                let mut iter = parts.iter();
                let author = iter.next().ok_or(RGitError::NotCommitHash)?;
                let author_name = iter.next().ok_or(RGitError::NotCommitHash)?;
                let author_email = iter.next().ok_or(RGitError::NotCommitHash)?;
                
                let timestamp= iter.next().ok_or(RGitError::NotCommitHash)?.parse::<i64>()
                            .map_err(|e|RGitError::ParseIntError { source: Box::new(e) })?;
                let time = DateTime::from_timestamp_secs(timestamp).ok_or(RGitError::CantGetDateTime)?;
                let date = format!("Date: {}\n",time.format("%a %b %d %H:%M:%S %Y %z"));
                let author_string = format!("{}: {} {}",author,author_name,author_email);

                result.push(author_string.white());
                result.push(date.white());
            }else if line.starts_with("commiter"){
                continue
            }else {
                result.push(format!("\t{}\n",line).white());
            }
        }
    }
    Ok(result)
}