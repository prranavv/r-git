use std::{collections::HashSet};
use std::io;
use std::io::Write;
use crate::{Result, internal::{utils::{parse_head, parse_index, parse_working_dir}}};

pub fn status()->Result<()>{
    let head_index = parse_head()?;
    let index = parse_index()?;
    let mut result = String::new();
    let mut hash_set = HashSet::new();
    result.push_str("Changes to be commited:\n");
    for h in head_index.iter(){
        for i in index.iter(){
            if h.file_path==i.file_path{
                if h.hash != i.hash{
                    let modified = format!("\tmodified: {}\n",h.file_path.display());
                    result.push_str(&modified);
                    hash_set.insert(i.file_path.clone());
                }
            }
        }
    }
    for i in index.iter(){
        if !head_index.contains(i) && !hash_set.contains(&i.file_path){
            let modified = format!("\tnew file: {}\n",i.file_path.display());
            result.push_str(&modified);
            hash_set.insert(i.file_path.clone());
        }
    }
    for i in head_index.iter(){
        if head_index.contains(i)&& !index.contains(&i){
            let modified = format!("\tdeleted: {}\n",i.file_path.display());
            result.push_str(&modified);
            hash_set.insert(i.file_path.clone());
        }
    }
    hash_set.clear();

    result.push_str("\nChanges not staged for commit:\n");
    let working_dir = parse_working_dir()?;
    for w in working_dir.iter(){
        for i in index.iter(){
            if w.file_path==i.file_path{
                if w.hash != i.hash{
                    let modified = format!("\tmodified: {}\n",i.file_path.display());
                    result.push_str(&modified);
                    hash_set.insert(i.file_path.clone());
                }
            }
        }
    }

    for i in index.iter(){
        if !working_dir.contains(i)&& index.contains(&i) && !hash_set.contains(&i.file_path){
            let modified = format!("\tdeleted: {}\n",i.file_path.display());
            result.push_str(&modified);
            hash_set.insert(i.file_path.clone());
        }
    }
    result.push_str("\nUntracked files:\n");
    for w in working_dir.iter(){
        if !index.contains(w) && !hash_set.contains(&w.file_path){
            let modified = format!("\t{}\n",w.file_path.display());
            result.push_str(&modified);
        }
    }
    let mut stdout = io::stdout();
    writeln!(stdout,"{}",result)?;
    Ok(())
}