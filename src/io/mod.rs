pub mod region;
use crate::error::Result;
use std::io::{Read, Write};

pub trait Readable: Sized {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self>;
}

pub trait Writeable {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64>;
}