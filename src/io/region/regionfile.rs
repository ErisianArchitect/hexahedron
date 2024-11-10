use std::{fs::File, io::{BufReader, BufWriter, Cursor, Read, Seek, SeekFrom, Take, Write}, path::Path};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use crate::{error::*, io::region::sectoroffset::SectorOffset, prelude::{write_zeros, Readable, Writeable}};
use super::{header::RegionHeader, regioncoord::RegionCoord, sectormanager::SectorManager, blocksize::BlockSize, timestamp::Timestamp};

pub struct RegionFile {
    sector_manager: SectorManager,
    /// Used for both reading and writing. The file is kept locked while the region is open.
    io: File,
    write_buffer: Cursor<Vec<u8>>,
    header: RegionHeader,
}

impl RegionFile {
    pub fn get_timestamp<C: Into<RegionCoord>>(&self, coord: C) -> Timestamp {
        self.header.timestamps[coord.into()]
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file_handle = File::options()
            .read(true)
            .write(true)
            .open(path.as_ref())?;
        let file_size = file_handle.seek(SeekFrom::End(0))?;
        // The file is too small to contain the header.
        if file_size < RegionHeader::HEADER_SIZE {
            return Err(Error::NoHead);
        }
        file_handle.seek(SeekFrom::Start(0))?;
        let header = {
            let mut temp_reader = BufReader::new((&mut file_handle).take(4096*3));
            RegionHeader::read_from(&mut temp_reader)?
        };
        let end_sector_start = (file_size / 4096) as u32;
        let sector_manager = SectorManager::from_sector_table(&header.offsets, end_sector_start);
        Ok(Self {
            io: file_handle,
            header,
            sector_manager,
            write_buffer: Cursor::new(Vec::with_capacity(4096*2)),
        })
    }

    /// Returns error if the file already exists.
    pub fn create_new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let parent = path.parent().ok_or(Error::ParentNotFound)?;
        std::fs::create_dir_all(parent)?;
        let mut io = File::options()
            .read(true).write(true)
            .create_new(true)
            .open(path)?;
        write_zeros(&mut io, RegionHeader::HEADER_SIZE)?;
        Ok(Self {
            io,
            write_buffer: Cursor::new(Vec::with_capacity(4096*2)),
            header: RegionHeader::new(),
            sector_manager: SectorManager::new(),
        })
    }

    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let parent = path.parent().ok_or(Error::ParentNotFound)?;
        std::fs::create_dir_all(parent)?;
        let mut io = File::options()
            .read(true).write(true)
            .create(true)
            .open(path)?;
        write_zeros(&mut io, RegionHeader::HEADER_SIZE)?;
        Ok(Self {
            io,
            write_buffer: Cursor::new(Vec::with_capacity(4096*2)),
            header: RegionHeader::new(),
            sector_manager: SectorManager::new(),
        })
    }

    /// Returns error if the path exists but isn't a file.
    pub fn open_or_create<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        if !path_ref.exists() {
            let parent = path_ref.parent().ok_or(Error::ParentNotFound)?;
            std::fs::create_dir_all(parent)?;
            Self::create_new(path)
        } else if path_ref.is_file() {
            Self::open(path)
        } else {
            Err(Error::NotAFile)
        }
    }

    pub fn read<'a, C: Into<RegionCoord>, R, F: FnMut(&mut GzDecoder<Take<BufReader<&'a mut File>>>) -> Result<R>>(&'a mut self, coord: C, mut read: F) -> Result<R> {
        let coord: RegionCoord = coord.into();
        let sector = self.header.offsets[coord];
        if sector.is_empty() {
            return Err(Error::ChunkNotFound);
        }
        let mut reader = BufReader::new(&mut self.io);
        reader.seek(SeekFrom::Start(sector.file_offset()))?;
        let length = u32::read_from(&mut reader)?;
        let mut decoder = GzDecoder::new(reader.take(length as u64));
        read(&mut decoder)
    }

    pub fn read_value<C: Into<RegionCoord>, T: Readable>(&mut self, coord: C) -> Result<T> {
        fn read_inner<'a, T: Readable>(reader: &mut GzDecoder<Take<BufReader<&'a mut File>>>) -> Result<T> {
            T::read_from(reader)
        }
        self.read(coord, read_inner)
    }

    /// If nothing is written to the writer, then it will delete the entry in the
    /// region file.
    pub fn write<C: Into<RegionCoord>, F: FnMut(&mut GzEncoder<&mut Cursor<Vec<u8>>>) -> Result<()>>(&mut self, coord: C, mut write: F) -> Result<()> {
        let coord: RegionCoord = coord.into();
        self.write_buffer.seek(SeekFrom::Start(0))?;
        self.write_buffer.get_mut().clear();
        let mut encoder = GzEncoder::new(&mut self.write_buffer, Compression::fast());
        // write to encoder in the write buffer
        write(&mut encoder)?;
        // Check if length is zero, which means nothing was written.
        // This will be treated as deleting the chunk.
        if encoder.get_ref().get_ref().len() == 0 {
            encoder.finish()?;
            return self.delete_data(coord);
        }
        encoder.finish()?;
        let length = self.write_buffer.get_ref().len() as u64;
        let padded_size = padded_size(length + 4);
        if padded_size > BlockSize::MAX_BLOCK_COUNT as u64 * 4096 {
            return Err(Error::ChunkTooLarge);
        }
        let block_size = (padded_size / 4096) as u16;
        let required_size = BlockSize::required(block_size);
        let old_sector = self.header.offsets[coord];
        let new_sector = self.sector_manager.reallocate_err(old_sector, required_size)?;
        self.header.offsets[coord] = new_sector;
        let mut writer = BufWriter::new(&mut self.io);
        writer.seek(SeekFrom::Start(new_sector.file_offset()))?;
        let len = length as u32;
        len.write_to(&mut writer)?;
        // write write_buffer to file.
        writer.write_all(self.write_buffer.get_ref().as_slice())?;
        write_zeros(&mut writer, pad_size(length as u64 + 4))?;
        // Write sector to header
        writer.seek(SeekFrom::Start(coord.sector_offset()))?;
        new_sector.write_to(&mut writer)?;
        writer.flush()?;
        Ok(())
    }

    pub fn write_value<C: Into<RegionCoord>, T: Writeable>(&mut self, coord: C, value: &T) -> Result<()> {
        self.write(coord, move |writer| {
            value.write_to(writer)?;
            Ok(())
        })
    }

    pub fn write_timestamped<C: Into<RegionCoord>, Ts: Into<Timestamp>, F: FnMut(&mut GzEncoder<&mut Cursor<Vec<u8>>>) -> Result<()>>(&mut self, coord: C, timestamp: Ts, write: F) -> Result<()> {
        let coord: RegionCoord = coord.into();
        self.write(coord, write)?;
        self.write_timestamp(coord, timestamp)
    }

    pub fn write_timestamp<C: Into<RegionCoord>, Ts: Into<Timestamp>>(&mut self, coord: C, timestamp: Ts) -> Result<()> {
        let coord: RegionCoord = coord.into();
        let timestamp: Timestamp = timestamp.into();
        self.header.timestamps[coord] = timestamp;
        let mut writer = BufWriter::new(&mut self.io);
        writer.seek(SeekFrom::Start(coord.timestamp_offset()))?;
        timestamp.write_to(&mut writer)?;
        writer.flush()?;
        Ok(())
    }

    pub fn delete_data<C: Into<RegionCoord>>(&mut self, coord: C) -> Result<()> {
        let coord: RegionCoord = coord.into();
        let sector = self.header.offsets[coord];
        if sector.is_empty() {
            return Ok(());
        }
        self.sector_manager.deallocate(sector);
        self.header.offsets[coord] = SectorOffset::default();
        self.header.timestamps[coord] = Timestamp::default();
        let mut writer = BufWriter::new(&mut self.io);
        writer.seek(SeekFrom::Start(coord.sector_offset()))?;
        write_zeros(&mut writer, 4)?;
        writer.seek(SeekFrom::Start(coord.timestamp_offset()))?;
        write_zeros(&mut writer, 8)?;
        writer.flush()?;
        Ok(())
    }
} 

#[inline]
fn pad_size(length: u64) -> u64 {
    4096 - (length & 4095) & 4095
}

#[inline]
fn padded_size(length: u64) -> u64 {
    const NEG4096_U64: u64 = -4096i64 as u64;
    (length + 4095) & NEG4096_U64
}