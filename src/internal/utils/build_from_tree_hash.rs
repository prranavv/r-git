use std::fs;
use crate::internal::utils::read_object;

pub fn build_from_tree_hash(tree_hash:String,path:String){
    let (_obj_type,content) = read_object(&tree_hash).unwrap();
    let lines = content.lines();
    for line in lines{
        let line = line.trim();
        if line.starts_with("40000"){
            let parts: Vec<&str>=line.split(' ').collect();
            let mut iter = parts.iter();
            let _mode = iter.next().unwrap();
            let _file_type = iter.next().unwrap();
            let tree_hash = iter.next().unwrap().to_string();
            let dir_name = iter.next().unwrap();
            build_from_tree_hash(tree_hash,format!("{}{}/",path,dir_name));
        }else if line.starts_with("100644"){
            let parts: Vec<&str>=line.split(' ').collect();
            let mut iter = parts.iter();
            let _mode = iter.next().unwrap();
            let _file_type = iter.next().unwrap();
            let blob_hash = iter.next().unwrap().to_string();
            let file_name = iter.next().unwrap();

            let file_path = format!("{}{}",path,file_name);

            let (_obj_type,blob_contents) = read_object(&blob_hash).unwrap();
            fs::write(file_path, blob_contents).unwrap();
        }else{  
            continue;
        }
    }
}
