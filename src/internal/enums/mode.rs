
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