use std::{fmt::Display, path::PathBuf};
use crate::{error::RGitError, internal::Mode};

#[derive(Debug,Clone,PartialEq, Eq, Hash)]
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

impl TryFrom<String> for IndexEntry{
    type Error = RGitError;
    fn try_from(value: String) -> Result<Self,Self::Error> {
        let parts:Vec<&str> = value.split(" ").collect();
        let mut iter = parts.iter();
        let mode = iter.next().ok_or(RGitError::NotValidIndexEntry { index_entry: value.clone() })?;
        let file_path = iter.next().ok_or(RGitError::NotValidIndexEntry { index_entry: value.clone() })?;
        let hash = iter.next().ok_or(RGitError::NotValidIndexEntry { index_entry: value.clone() })?;
        let mode = Mode::from(*mode);

        Ok(Self { mode, file_path: PathBuf::from(file_path), hash: hash.to_string() })
    }
}

impl TryFrom<&str> for IndexEntry{
    type Error = RGitError;
    fn try_from(value: &str) -> Result<Self,Self::Error> {
        let parts:Vec<&str> = value.split(" ").collect();
        let mut iter = parts.iter();
        let mode = iter.next().ok_or(RGitError::NotValidIndexEntry { index_entry: value.into() })?;
        let file_path = iter.next().ok_or(RGitError::NotValidIndexEntry { index_entry: value.into() })?;
        let hash = iter.next().ok_or(RGitError::NotValidIndexEntry { index_entry: value.into() })?;
        let mode = Mode::from(*mode);

        Ok(Self { mode, file_path: PathBuf::from(file_path), hash: hash.to_string() })
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