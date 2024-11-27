use std::collections::BTreeMap;

use hashbrown::HashMap;
use itertools::Itertools;
use tap::Tap;

use crate::error::*;

use super::{
    region_table::OffsetTable,
    block_size::BlockSize,
    sector_offset::SectorOffset
};

// refactor

// TODO: Recreate SectorManager struct and implementation.
#[derive(Debug, Clone)]
pub struct SectorManager {
    start_sectors: HashMap<u32, u32>,
    end_sectors: HashMap<u32, u32>,
    sized_sectors: BTreeMap<u32, BTreeMap<u32, u32>>,
}

impl Default for SectorManager {
    fn default() -> Self {
        Self {
            start_sectors: HashMap::new(),
            end_sectors: HashMap::new(),
            sized_sectors: BTreeMap::new(),
        }.tap_mut(move |init| {
            init.new_insert_free_sector(Self::AFTER_HEAD);
        })
    }
}

impl SectorManager {
    // /// The max size representable by the [BlockSize] type.
    // const MAX_SECTOR_SIZE: u32 = BlockSize::MAX_BLOCK_COUNT as u32 * 4096;
    /// The offset that's calculated to be the maximum end-offset possible.
    const MAX_SECTOR_END: u32 = (2u32.pow(24) - 1) + 8034; // 24-bit unsigned max + BlockSize max
    const AFTER_HEAD: ManagedSector = ManagedSector::new(3, Self::MAX_SECTOR_END);

    pub fn new(end_sector: Option<std::ops::Range<u32>>) -> Self {
        Self {
            start_sectors: HashMap::new(),
            end_sectors: HashMap::new(),
            sized_sectors: BTreeMap::new(),
        }.tap_mut(move |init| {
            if let Some(end) = end_sector {
                init.new_insert_free_sector(ManagedSector::new(end.start, end.end));
            }
        })
    }

    pub fn from_sector_table(table: &OffsetTable) -> Self {
        let mut filtered_sectors = table.iter().cloned()
            .map(ManagedSector::from)
            .filter(|sector| sector.is_not_empty())
            .collect_vec();
        filtered_sectors.sort();
        let initial_state = (
            Self::new(None),
            ManagedSector::HEADER,
        );
        let (
            init,
            last_sector
        ) = filtered_sectors.into_iter()
            .fold(initial_state, |(mut builder, previous), sector| {
                if previous.has_gap(sector) {
                    builder.new_insert_free_sector(ManagedSector::new(previous.end, sector.start));
                }
                (
                    builder,
                    sector
                )
            });
        init.tap_mut(move |init| {
            let end_sector = ManagedSector::new(last_sector.end, Self::MAX_SECTOR_END);
            if end_sector.is_not_empty() {
                init.new_insert_free_sector(end_sector);
            }
        })
    }

    /// For insertions when there doesn't need to be any removal, such as when
    /// inserting the gaps between used sectors.
    fn new_insert_free_sector(&mut self, sector: ManagedSector) {
        self.start_sectors.insert(sector.start, sector.end);    // O(10)
        self.end_sectors.insert(sector.end, sector.start);      // O(10)
        let sized = self.sized_sectors.entry(sector.size()).or_insert_with(|| BTreeMap::new());     // O(10)
        sized.insert(sector.start, sector.end);     // O(10)
    }

    fn insert_free_sector(&mut self, sector: ManagedSector) {
        // Complexity is roughly O(100)
        // Merge adjacent sectors.
        let left = self.remove_end_sector(sector.start);    // O(20)
        let right = self.remove_start_sector(sector.end);   // O(20)
        let sector = match (left, right) {
            (Some(left), Some(right)) => {
                self.remove_sized_sector(left);     // O(30)
                self.remove_sized_sector(right);    // O(30)
                ManagedSector::new(left.start, right.end)
            }
            (Some(left), None) => {
                self.remove_sized_sector(left);     // O(30)
                sector.join_left(left)
            }
            (None, Some(right)) => {
                self.remove_sized_sector(right);   // O(30)
                sector.join_right(right)
            }
            _ => sector
        };
        self.start_sectors.insert(sector.start, sector.end);    // O(10)
        self.end_sectors.insert(sector.end, sector.start);      // O(10)
        let sized = self.sized_sectors.entry(sector.size()).or_insert_with(|| BTreeMap::new());     // O(10)
        sized.insert(sector.start, sector.end);     // O(10)
    }

    fn remove_start_sector(&mut self, start: u32) -> Option<ManagedSector> {
        let end = self.start_sectors.remove(&start)?;
        self.end_sectors.remove(&end);
        Some(ManagedSector::new(start, end))
    }

    fn remove_end_sector(&mut self, end: u32) -> Option<ManagedSector> {
        let start = self.end_sectors.remove(&end)?;
        self.start_sectors.remove(&start);
        Some(ManagedSector::new(start, end))
    }

    fn remove_sized_sector(&mut self, sector: ManagedSector) {
        if let Some(sized) = self.sized_sectors.get_mut(&sector.size()) {
            sized.remove(&sector.start);
            if sized.is_empty() {
                self.sized_sectors.remove(&sector.size());
            }
        }
    }

    fn pop_sized_sector(&mut self, size: u32) -> Option<ManagedSector> {
        let (&found_size, sized_map) = self.sized_sectors.range_mut(size..).next()?;
        // pop from the left side to ensure that the allocation is coming from the left-most sector.
        let Some((start, end)) = sized_map.pop_first() else {
            // If this panic happens, that means that the SectorManager is bugged.
            panic!("Corrupted SectorManager: Found an empty sized_map entry.");
        };
        if sized_map.is_empty() {
            self.sized_sectors.remove(&found_size);
        }
        self.start_sectors.remove(&start);
        self.end_sectors.remove(&end);
        let sector = ManagedSector::new(start, end);
        // split from the left side so that the sector manager is always allocating the left-most sector.
        let (alloc, old) = sector.split_left(size);
        if old.is_not_empty() {
            self.insert_free_sector(old);
        }
        Some(alloc)
    }

    /// Attempts to allocate a sector.
    pub fn allocate(&mut self, block_size: BlockSize) -> Option<SectorOffset> {
        let block_count = block_size.block_count();
        let Some(sector) = self.pop_sized_sector(block_count as u32) else {
            return None;
        };
        Some(SectorOffset::new(block_size, sector.start))
    }

    #[inline]
    fn dealloc_managed(&mut self, sector: ManagedSector) {
        if sector.size() == 0 {
            return;
        }
        self.insert_free_sector(sector);
    }

    pub fn deallocate(&mut self, sector: SectorOffset) {
        let sector: ManagedSector = sector.into();
        self.dealloc_managed(sector);
    }

    pub fn reallocate(&mut self, free: SectorOffset, new_size: BlockSize) -> Option<SectorOffset> {
        if free.is_empty() {
            self.allocate(new_size)
        } else if free.block_size() > new_size {
            let old_sector = ManagedSector::from(free);
            let (new, old) = old_sector.split_left(new_size.block_count() as u32);
            self.dealloc_managed(old);
            Some(SectorOffset::new(new_size, new.start))
        } else if free.block_size() == new_size {
            Some(free)
        } else {
            self.reallocate_unchecked(free, new_size)
        }
    }

    pub fn reallocate_err(&mut self, free: SectorOffset, size: BlockSize) -> Result<SectorOffset> {
        self.reallocate(free, size).ok_or_else(|| Error::AllocationFailure(free, size))
    }
    
    fn reallocate_unchecked(&mut self, free: SectorOffset, new_size: BlockSize) -> Option<SectorOffset> {
        let free = ManagedSector::from(free);
        self.insert_free_sector(free);
        self.allocate(new_size)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct ManagedSector {
    start: u32,
    end: u32
}

impl ManagedSector {
    const TIMESTAMP_TABLE: ManagedSector = ManagedSector {
        start: 0,
        end: 2
    };
    const SECTOR_TABLE: ManagedSector = ManagedSector {
        start: 2,
        end: 3
    };
    const HEADER: ManagedSector = Self::TIMESTAMP_TABLE.join_right(Self::SECTOR_TABLE);

    const fn new(start: u32, end: u32) -> Self {
        Self {
            start,
            end
        }
    }

    // fn is_empty(self) -> bool {
    //     self.start == self.end
    // }

    pub fn is_not_empty(self) -> bool {
        self.start != self.end
    }

    pub fn size(self) -> u32 {
        self.end - self.start
    }

    pub fn split_left(self, sector_count: u32) -> (Self, Self) {
        if sector_count > self.size() {
            panic!("Sector not large enough to accomodate sector count.");
        }
        let middle = self.start + sector_count;
        (
            ManagedSector::new(self.start, middle),
            ManagedSector::new(middle, self.end)
        )
    }

    /// Joins other to left side of self.
    #[inline]
    pub const fn join_left(self, other: Self) -> Self {
        Self::new(other.start, self.end)
    }

    /// Joins other to right side of self.
    #[inline]
    pub const fn join_right(self, other: Self) -> Self {
        Self::new(self.start, other.end)
    }

    pub const fn has_gap(self, other: Self) -> bool {
        self.end < other.start || other.end < self.start
    }

    // pub const fn gap(self, other: Self) -> Option<u32> {
    //     if self.end < other.start {
    //         Some(other.start - self.end)
    //     } else if other.end < self.start {
    //         Some(self.start - other.end)
    //     } else {
    //         None
    //     }
    // }
}

impl PartialOrd for ManagedSector {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.start.partial_cmp(&other.start) {
            Some(core::cmp::Ordering::Equal) => self.end.partial_cmp(&other.end),
            ord => ord,
        }
    }
}

impl Ord for ManagedSector {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.start.cmp(&other.start) {
            core::cmp::Ordering::Equal => self.end.cmp(&other.end),
            ord => ord,
        }
    }
}

impl From<SectorOffset> for ManagedSector {
    fn from(value: SectorOffset) -> Self {
        let start = value.block_offset();
        let size = value.block_size().block_count() as u32;
        let end = start + size;
        Self::new(start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sector_manager_test() -> Result<()> {
        let mut man = SectorManager::default();
        println!("{man:#?}");
        let sect1 = man.allocate(BlockSize::reverse(1).unwrap()).ok_or(Error::Custom("Failed to allocate block"))?;
        let sect2 = man.allocate(BlockSize::reverse(4).unwrap()).ok_or(Error::Custom("Failed to allocate block"))?;
        let ms1 = ManagedSector::from(sect1);
        let ms2 = ManagedSector::from(sect2);
        assert_eq!(ms1, ManagedSector::new(3, 4));
        assert_eq!(ms2, ManagedSector::new(4, 8));
        let sect2 = man.reallocate_err(sect2, BlockSize::required(8))?;
        let ms2 = ManagedSector::from(sect2);
        assert_eq!(ms2, ManagedSector::new(4, 12));
        man.deallocate(sect1);
        let sect1 = man.allocate(BlockSize::reverse(1).unwrap()).ok_or(Error::Custom("Failed to allocate block"))?;
        let ms1 = ManagedSector::from(sect1);
        assert_eq!(ms1, ManagedSector::new(3, 4));
        man.deallocate(sect1);
        man.deallocate(sect2);
        println!("{man:#?}");
        Ok(())
    }
}