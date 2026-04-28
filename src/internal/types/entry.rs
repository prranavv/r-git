use std::ffi::OsString;

use crate::internal::Mode;
use sha1::{digest::{array::Array, consts::{B0, B1}, typenum::{UInt, UTerm}}};

#[derive(Debug)]
pub struct Entry{
    pub mode:Mode,
    pub file_name:OsString,
    pub hash:Array<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>>
}

impl Entry{
    pub fn new(mode:Mode,file_name: OsString,hash: Array<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>>)->Entry{
        Entry { mode, file_name, hash}
    }
}