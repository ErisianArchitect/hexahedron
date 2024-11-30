use chrono::*;

use crate::prelude::{Readable, Writeable};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(pub i64);

impl Timestamp {
    #[inline]
    pub const fn new(timestamp: i64) -> Self {
        Self(timestamp)
    }

    #[inline]
    pub fn utc_now() -> Self {
        Self(Utc::now().timestamp())
    }

    /// Gets the UNIX UTC timestamp.
    #[inline]
    pub const fn timestamp(self) -> i64 {
        self.0
    }

    /// Gets the [Utc] [DateTime].
    #[inline]
    pub fn time(self) -> DateTime<Utc> {
        chrono::DateTime::from_timestamp(self.0, 0).expect("Timestamp was invalid.")
    }
}

impl Writeable for Timestamp {
    #[inline]
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> crate::prelude::VoxelResult<u64> {
        self.0.write_to(writer)
    }
}

impl Readable for Timestamp {
    #[inline]
    fn read_from<R: std::io::Read>(reader: &mut R) -> crate::prelude::VoxelResult<Self> {
        Ok(Timestamp(i64::read_from(reader)?))
    }
}