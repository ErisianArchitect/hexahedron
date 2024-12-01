use super::{region_table::RegionTableItem, sector_offset::SectorOffset, time_stamp::Timestamp};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionCoord(u16);

impl RegionCoord {
    #[inline]
    pub const fn new(x: i32, z: i32) -> Self {
        RegionCoord((x & 31) as u16 | ((z & 31) as u16) << 5)
    }

    #[inline]
    pub const fn x(self) -> i32 {
        (self.0 & 31) as i32
    }

    #[inline]
    pub const fn z(self) -> i32 {
        (self.0 >> 5 & 31) as i32
    }

    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub const fn sector_offset(self) -> u64 {
        SectorOffset::OFFSET + 4 * self.0 as u64
    }

    #[inline]
    pub const fn timestamp_offset(self) -> u64 {
        Timestamp::OFFSET + 8 * self.0 as u64
    }
}

impl Into<(i32, i32)> for RegionCoord {
    #[inline]
    fn into(self) -> (i32, i32) {
        (self.x(), self.z())
    }
}

impl From<(i32, i32)> for RegionCoord {
    #[inline]
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0, value.1)
    }
}