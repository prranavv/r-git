use std::ops::Deref;

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