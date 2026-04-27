use std::fmt::Display;

#[repr(u32)]
#[derive(Clone, Copy,Debug)]
pub enum Mode{
    File = 0o100644,
    Executable = 0o100755,
    Directory = 0o040000
}

impl From<&str> for Mode{
    fn from(value: &str) -> Self {
        match value{
            "100644" => return Mode::File,
            "40000" => return Mode::Directory,
            _=> return Mode::Executable
        }
    }
}

impl Display for Mode{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Mode::File=>write!(f, "100644"),
            Mode::Directory=>write!(f, "40000"),
            Mode::Executable=>write!(f,"100755")
        }
    }
}