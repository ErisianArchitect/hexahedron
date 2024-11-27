use crate::prelude::{Replace, Readable, Writeable};
use crate::io::region::block_size::BlockSize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SectorOffset(u32);

impl SectorOffset {
    pub const MAX_OFFSET: u32 = 0xffffff;
    pub const EMPTY: Self = SectorOffset(0);

    pub const fn new(block_size: BlockSize, offset: u32) -> Self  {
        if offset > Self::MAX_OFFSET {
            panic!("Offset is greater than 0xffffff");
        }
        Self(block_size.0 as u32 | offset << 8)
    }

    pub const fn block_size(self) -> BlockSize {
        let mask = self.0 & 0xff;
        BlockSize(mask as u8)
    }

    /// The sector size in bytes.
    pub const fn file_size(self) -> u64 {
        self.block_size().file_size()
    }

    /// The offset in 4KiB blocks. (multiply by 4096 to get file offset)
    pub const fn block_offset(self) -> u32 {
        self.0 >> 8
    }

    /// The sector offset in bytes.
    pub const fn file_offset(self) -> u64 {
        self.block_offset() as u64 * 4096
    }

    /// Determines if the sector is empty.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Determines if the sector is not empty.
    pub const fn is_not_empty(self) -> bool {
        self.0 != 0
    }
}

impl Writeable for SectorOffset {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> crate::prelude::VoxelResult<u64> {
        let block_size = self.block_size().0;
        let block_offset = self.block_offset();
        let mut offset_bytes = block_offset.to_be_bytes();
        offset_bytes[0] = block_size;
        writer.write_all(&offset_bytes)?;
        Ok(4)
    }
}

impl Readable for SectorOffset {
    fn read_from<R: std::io::Read>(reader: &mut R) -> crate::prelude::VoxelResult<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        let block_size = BlockSize(buf[0].replace(0));
        let offset = u32::from_be_bytes(buf);
        Ok(SectorOffset::new(block_size, offset))
    }
}

impl std::fmt::Display for SectorOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SectorOffset(offset={}, size={})", self.file_offset(), self.block_size())
    }
}