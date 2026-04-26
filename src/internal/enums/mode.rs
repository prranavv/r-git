
#[repr(u32)]
#[derive(Clone, Copy,Debug)]
pub enum Mode{
    File = 0o100644,
    Executable = 0o100755,
    Directory = 0o040000
}