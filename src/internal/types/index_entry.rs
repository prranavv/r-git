use std::{fmt::Display, path::PathBuf};
use crate::internal::Mode;

#[derive(Debug)]
pub struct IndexEntry{
    pub mode: Mode,
    pub file_path: PathBuf,
    pub hash: String
}

impl IndexEntry{

    pub fn new(mode:Mode,file_path:PathBuf,hash:String)->Self{
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

        Self { mode, file_path: PathBuf::from(file_path), hash: hash.to_string() }
    }
}

impl From<&str> for IndexEntry{
    fn from(value: &str) -> Self {
        let parts:Vec<&str> = value.split(" ").collect();
        let mut iter = parts.iter();
        let mode = iter.next().unwrap();
        let file_path = iter.next().unwrap();
        let hash = iter.next().unwrap();
        let mode = Mode::from(*mode);

        Self { mode, file_path: PathBuf::from(file_path), hash: hash.to_string() }
    }
}

impl From<IndexEntry> for String{
    fn from(value: IndexEntry) -> Self {
        format!("{} {} {}",value.mode,value.file_path.display(),value.hash)
    }
}

impl Display for IndexEntry{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {} {}",self.mode,self.file_path.display(),self.hash)
    }
}