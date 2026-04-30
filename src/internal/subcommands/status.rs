use std::collections::VecDeque;
use std::{collections::HashSet};
use std::io;
use std::io::Write;
use crossterm::style::Stylize;
use crate::{Result, internal::{utils::{parse_head, parse_index, parse_working_dir}}};

pub fn status()->Result<()>{
    let head_index = parse_head()?;
    let index = parse_index()?;
    let mut result = VecDeque::new();
    let mut hash_set = HashSet::new();
    
    let mut commit_msg = VecDeque::new();
    
    for h in head_index.iter(){
        for i in index.iter(){
            if h.file_path==i.file_path{
                if h.hash != i.hash{
                    let modified = format!("\tmodified: {}",h.file_path.display()).green();
                    commit_msg.push_back(modified);
                    hash_set.insert(i.file_path.clone());
                }
            }
        }
    }
    for i in index.iter(){
        if !head_index.contains(i) && !hash_set.contains(&i.file_path){
            let modified = format!("\tnew file: {}",i.file_path.display()).green();
            commit_msg.push_back(modified);
            hash_set.insert(i.file_path.clone());
        }
    }
    for i in head_index.iter(){
        if head_index.contains(i)&& !index.contains(&i) && !hash_set.contains(&i.file_path){
            let modified = format!("\tdeleted: {}",i.file_path.display()).green();
            commit_msg.push_back(modified);
            hash_set.insert(i.file_path.clone());
        }
    }
    if commit_msg.len()!=0{
        commit_msg.push_front("Changes to be commited:".to_string().white());
    }

    result.extend(commit_msg);
    hash_set.clear();

    let mut not_commited_msg = VecDeque::new();
    let working_dir = parse_working_dir()?;
    for w in working_dir.iter(){
        for i in index.iter(){
            if w.file_path==i.file_path{
                if w.hash != i.hash{
                    let modified = format!("\tmodified: {}",i.file_path.display()).red();
                    not_commited_msg.push_back(modified);
                    hash_set.insert(i.file_path.clone());
                }
            }
        }
    }

    for i in index.iter(){
        if !working_dir.contains(i)&& index.contains(&i) && !hash_set.contains(&i.file_path){
            let modified = format!("\tdeleted: {}",i.file_path.display()).red();
            not_commited_msg.push_back(modified);
            hash_set.insert(i.file_path.clone());
        }
    }

    if not_commited_msg.len()!=0{
        not_commited_msg.push_front("\nChanges not staged for commit:".to_string().white());
    }

    result.extend(not_commited_msg);

    let mut untracked = VecDeque::new();
    for w in working_dir.iter(){
        if !index.contains(w) && !hash_set.contains(&w.file_path){
            let modified = format!("\t{}\n",w.file_path.display()).red();
            untracked.push_back(modified);
        }
    }

    if untracked.len()!=0{
        untracked.push_front("\nUntracked files:".to_string().white());
    }
    result.extend(untracked);
    let mut stdout = io::stdout();
    if result.len()==0{
        write!(stdout,"nothing to commit, working tree clean\n")?;
        return Ok(())
    }
    for r in result{
        writeln!(stdout,"{}",r)?;
    }
    Ok(())
}