use std::{collections::HashSet};
use std::io;
use std::io::Write;
use crossterm::style::Stylize;
use crate::{Result, internal::{utils::{parse_head, parse_index, parse_working_dir}}};

pub fn status()->Result<()>{
    let head_index = parse_head()?;
    let index = parse_index()?;
    let mut result = Vec::new();
    let mut hash_set = HashSet::new();
    
    result.push("Changes to be commited:".to_string().white());
    for h in head_index.iter(){
        for i in index.iter(){
            if h.file_path==i.file_path{
                if h.hash != i.hash{
                    let modified = format!("\tmodified: {}",h.file_path.display()).green();
                    result.push(modified);
                    hash_set.insert(i.file_path.clone());
                }
            }
        }
    }
    for i in index.iter(){
        if !head_index.contains(i) && !hash_set.contains(&i.file_path){
            let modified = format!("\tnew file: {}",i.file_path.display()).green();
            result.push(modified);
            hash_set.insert(i.file_path.clone());
        }
    }
    for i in head_index.iter(){
        if head_index.contains(i)&& !index.contains(&i) && !hash_set.contains(&i.file_path){
            let modified = format!("\tdeleted: {}",i.file_path.display()).green();
            result.push(modified);
            hash_set.insert(i.file_path.clone());
        }
    }
    hash_set.clear();

    result.push("\nChanges not staged for commit:".to_string().white());
    let working_dir = parse_working_dir()?;
    for w in working_dir.iter(){
        for i in index.iter(){
            if w.file_path==i.file_path{
                if w.hash != i.hash{
                    let modified = format!("\tmodified: {}",i.file_path.display()).red();
                    result.push(modified);
                    hash_set.insert(i.file_path.clone());
                }
            }
        }
    }

    for i in index.iter(){
        if !working_dir.contains(i)&& index.contains(&i) && !hash_set.contains(&i.file_path){
            let modified = format!("\tdeleted: {}",i.file_path.display()).red();
            result.push(modified);
            hash_set.insert(i.file_path.clone());
        }
    }
    result.push("\nUntracked files:".to_string().white());
    for w in working_dir.iter(){
        if !index.contains(w) && !hash_set.contains(&w.file_path){
            let modified = format!("\t{}\n",w.file_path.display()).red();
            result.push(modified);
        }
    }
    let mut stdout = io::stdout();
    for r in result{
        writeln!(stdout,"{}",r)?;
    }
    Ok(())
}