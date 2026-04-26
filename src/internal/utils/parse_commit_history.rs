use chrono::DateTime;

use crate::{Result,internal::utils::{get_parent_hash, read_object}};

pub fn parse_commit_history()->Result<String>{
    let mut head_commit = get_parent_hash();
    let mut result = String::new();
    while let Some(ref h)=head_commit{
        let head =h.trim();
        if head==""{
            break;
        }    
        let (_obj_type,content) = read_object(&head.trim().to_string())?;
        let lines = content.lines();
        let commit_hash_line = format!("commit {}\n",head);
        result.push_str(&commit_hash_line);
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
                let _parent = iter.next().unwrap();
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
                let author = iter.next().unwrap();
                let author_name = iter.next().unwrap();
                let author_email = iter.next().unwrap();
                
                let timestamp: i64 = iter.next().unwrap().parse().unwrap();
                let time = DateTime::from_timestamp_secs(timestamp).unwrap();
                let date = format!("Date: {}\n\n",time.format("%a %b %d %H:%M:%S %Y %z"));
                let author_string = format!("{}: {} {}\n",author,author_name,author_email);

                result.push_str(&author_string);
                result.push_str(&date);
            }else if line.starts_with("commiter"){
                continue
            }else {
                result.push_str(&format!("\t{}\n\n",line));
            }
        }
    }
    Ok(result)
}