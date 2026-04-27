use crate::internal::{entry::Entry};

fn build_entry(file_entry:&Entry)->Vec<u8>{
    let mut entry:Vec<u8> = Vec::new();
    let mode = file_entry.mode as u32;

    let mode_str = format!("{:o}",mode);
    entry.extend_from_slice(&mode_str.as_bytes());
    entry.extend_from_slice(b" ");
    
    entry.extend_from_slice(file_entry.file_name.as_bytes());
    entry.push(0);
    entry.extend_from_slice(&file_entry.hash);

    entry
}

pub fn build_entries(entries:&Vec<Entry>)->Vec<u8>{
    let mut result:Vec<u8> = Vec::new();
    for file_entry in entries.iter(){
        let hash = build_entry(file_entry);
        result.extend(hash);
    }
    result
}