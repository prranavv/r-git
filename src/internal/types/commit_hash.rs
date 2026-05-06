use std::{fmt::Display, ops::Deref};

pub struct CommitHash(String);

impl CommitHash{
    pub fn new(hash:String)->Self{
        Self(hash)
    }
}

impl Deref for CommitHash{
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for CommitHash{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{}",self.0)
    }
}