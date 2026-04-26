use chrono::Utc;

use crate::{internal::utils::get_parent_hash};


pub fn commit_message(tree_hash:&String,message:&String)->String{
    let mut content = String::new();
    content.push_str(&format!("tree {}\n",tree_hash));

    let parent_hash =match get_parent_hash(){
        Some(p)=>p,
        None=>" ".to_string()
    };

    content.push_str(&format!("parent {}\n",parent_hash));
    
    let time = Utc::now();
    let timestamp = time.timestamp();
    let timezone = "+530";

    content.push_str(&format!("author Pranav pranvvkumar03@gmail.com {} {}\n",timestamp,timezone));
    content.push_str(&format!("commiter Pranav pranvvkumar03@gmail.com {} {}\n",timestamp,timezone));
    
    content.push_str("\n");

    content.push_str(&format!("{}",message));

    content
}