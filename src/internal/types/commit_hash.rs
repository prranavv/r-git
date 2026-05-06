use std::{ops::Deref};

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

impl ToString for CommitHash{
    fn to_string(&self) -> String {
        let x = self.0.trim();
        String::from(x)
    }
}