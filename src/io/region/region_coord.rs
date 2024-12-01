use super::{region_table::RegionTableItem, sector_offset::SectorOffset, time_stamp::Timestamp};

/// A coordinate within a region file. A value from (0, 0) to (31, 31).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionCoord(u16);

impl RegionCoord {
    /// Creates a new [RegionCoord] at the specified coordinate.
    /// The coordinate is wrapped by a factor of 32.
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
    pub const fn to_tuple(self) -> (i32, i32) {
        (
            self.x(),
            self.z(),
        )
    }

    #[inline]
    pub const fn from_tuple((x, z): (i32, i32)) -> Self {
        Self::new(x, z)
    }

    /// The index in 32x32 flat array.
    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    /// The offset of this [RegionCoord]'s [SectorOffset] in the file header.
    #[inline]
    pub const fn sector_offset(self) -> u64 {
        SectorOffset::OFFSET + 4 * self.0 as u64
    }

    /// The offset of this [RegionCoord]'s [Timestamp] in the file header.
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn coord_test() {
        let c1 = RegionCoord::new(3, 5);
        let c2 = RegionCoord::new(5, 3);
        assert_eq!(c1.x(), 3);
        assert_eq!(c1.z(), 5);
        assert_eq!(c2.x(), 5);
        assert_eq!(c2.z(), 3);
    }
}