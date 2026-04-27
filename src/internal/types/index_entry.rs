use std::fmt::Display;

use crate::internal::Mode;

#[derive(Debug)]
pub struct IndexEntry{
    pub mode: Mode,
    pub file_path: String,
    pub hash: String
}

impl IndexEntry{

    pub fn new(mode:Mode,file_path:String,hash:String)->Self{
        Self { mode, file_path, hash }
    }
}

impl From<String> for IndexEntry{
    fn from(value: String) -> Self {
        let parts:Vec<&str> = value.split(" ").collect();
        let mut iter = parts.iter();
        let mode = iter.next().unwrap();
        let file_path = iter.next().unwrap();
        let hash = iter.next().unwrap();
        let mode = Mode::from(*mode);

        Self { mode, file_path: file_path.to_string(), hash: hash.to_string() }
    }
}

impl From<IndexEntry> for String{
    fn from(value: IndexEntry) -> Self {
        format!("{} {} {}",value.mode,value.file_path,value.hash)
    }
}

impl Display for IndexEntry{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {} {}",self.mode,self.file_path,self.hash)
    }
}