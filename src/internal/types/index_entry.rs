use crate::internal::Mode;

pub struct IndexEntry{
    pub mode: Mode,
    pub file_path: String,
    pub hash: String
}

impl IndexEntry{

    pub fn new(mode:Mode,file_path:String,hash:String)->Self{
        Self { mode, file_path, hash }
    }

    pub fn parse(raw_entry:String)->Self{
        // 10644 src/main.rs 3e38a71ebbdf140392c28bd1e065bd2b9dd9588c
        let parts:Vec<&str> = raw_entry.split(" ").collect();
        let mut iter = parts.iter();
        let mode = iter.next().unwrap();
        let file_path = iter.next().unwrap();
        let hash = iter.next().unwrap();
        let mode = Mode::from(*mode);

        Self { mode, file_path: file_path.to_string(), hash: hash.to_string() }
    }
}