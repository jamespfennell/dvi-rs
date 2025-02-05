use std::io::{self, Write};

/// A type that can be written to a stream (serialized)
pub trait Dump {
    fn dump<W>(&self, _: &mut W) -> io::Result<()>
    where
        W: Write;
}

/// A type that can be parsed from a byte slice
#[cfg(feature = "parse")]
pub trait Parse<V>
where
    V: AsRef<[u8]>,
    Self: Sized,
{
    /// Returns the Instruction and how many bytes were used
    fn parse(v: V) -> super::IResult<V, Self>;
}
