pub mod region;
use crate::error::{Error, Result};
use crate::math::axisflags::AxisFlags;
use std::io::{Read, Write};
use crate::for_each_int_type;
use crate::math::bit::*;
use crate::rendering::color::{
    Rgb,
    Rgba,
};
use crate::prelude::{
    Direction,
    Cardinal,
    Rotation,
    Flip,
    Orientation,
    Axis,
    FaceFlags,
};
use crate::tag::{
    NonByte,
    Tag,
};
use glam::{
    IVec2,
    Vec2,
    IVec3,
    Vec3,
    IVec4,
    Vec4,
    Mat2,
    mat2,
    Mat3,
    mat3,
    Mat4,
    mat4,
    Quat,
    quat,
};
use rollgrid::{
    rollgrid2d::Bounds2D,
    rollgrid3d::Bounds3D,
};
use paste::paste;
use itertools::*;

const MAX_ARRAY_LENGTH: usize = 0xffffff;

pub trait Readable: Sized {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self>;
}

pub trait Writeable {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64>;
}

pub trait ReadExt {
    fn read_value<T: Readable>(&mut self) -> Result<T>;
}

pub trait WriteExt {
    fn write_value<T: Writeable>(&mut self, value: &T) -> Result<u64>;
}

impl<R: Read> ReadExt for R {
    fn read_value<T: Readable>(&mut self) -> Result<T> {
        T::read_from(self)
    }
}

impl<W: Write> WriteExt for W {
    fn write_value<T: Writeable>(&mut self, value: &T) -> Result<u64> {
        value.write_to(self)
    }
}

pub fn read_u24<R: Read>(reader: &mut R) -> Result<u32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf[1..4])?;
    Ok(u32::from_be_bytes(buf))
}


pub fn write_u24<W: Write>(writer: &mut W, value: u32) -> Result<u64> {
    if value > 0xffffff {
        return Err(Error::U24OutOfRange);
    }
    let buf = value.to_be_bytes();
    writer.write_all(&buf[1..4])?;
    Ok(3)
}

pub fn write_zeros<W: Write>(writer: &mut W, count: u64) -> Result<u64> {
    const ZEROS: [u8; 4096] = [0; 4096];
    let mut count = count;
    // If we don't use >=,  we can optimize for the case where count is a multiple of 4096.
    while count > 4096 {
        writer.write_all(&ZEROS)?;
        count -= 4096;
    }
    writer.write_all(&ZEROS[0..count as usize])?;
    Ok(count)
}

/// Reads an exact number of bytes from a reader, returning them as a [Vec].
pub fn read_bytes<R: Read>(reader: &mut R, length: usize) -> Result<Vec<u8>> {
    let mut buf: Vec<u8> = vec![0u8; length];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}

/// Writes a byte slice to a writer, returning the number of bytes that were written.
pub fn write_bytes<W: Write>(writer: &mut W, data: &[u8]) -> Result<u64> {
    Ok(writer.write_all(data).map(|_| data.len() as u64)?)
}

macro_rules! num_io {
    ($type:ty) => {
        impl Readable for $type {
            #[must_use]
            fn read_from<R: Read + Sized>(reader: &mut R) -> Result<Self> {
                let mut buffer = [0u8; std::mem::size_of::<$type>()];
                reader.read_exact(&mut buffer)?;
                let value = <$type>::from_be_bytes(buffer);
                Ok(value)
            }
        }

        impl Writeable for $type {
            #[must_use]
            fn write_to<W: Write + Sized>(&self, writer: &mut W) -> Result<u64> {
                let bytes = self.to_be_bytes();
                writer.write_all(&bytes)?;
                Ok(std::mem::size_of::<$type>() as u64)
            }
        }
    };
}

for_each_int_type!(num_io);

impl Readable for bool {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        Ok(buffer[0] != 0)
    }
}

impl Writeable for bool {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        let bytes = [*self as u8];
        writer.write_all(&bytes)?;
        Ok(1)
    }
}

impl Readable for AxisFlags {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        Ok(AxisFlags::from_u8(buffer[0]))
    }
}

impl Writeable for AxisFlags {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.inner().write_to(writer)
    }
}

impl Readable for FaceFlags {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        Ok(FaceFlags::from(buffer[0]))
    }
}

impl Writeable for FaceFlags {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.inner().write_to(writer)
    }
}

impl Readable for BitFlags8 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        Ok(BitFlags8(buffer[0]))
    }
}

impl Writeable for BitFlags8 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        let buffer = [self.0];
        writer.write_all(&buffer)?;
        Ok(1)
    }
}

impl Readable for BitFlags16 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self(u16::read_from(reader)?))
    }
}

impl Writeable for BitFlags16 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.0.write_to(writer)
    }
}

impl Readable for BitFlags32 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self(u32::read_from(reader)?))
    }
}

impl Writeable for BitFlags32 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.0.write_to(writer)
    }
}

impl Readable for BitFlags64 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self(u64::read_from(reader)?))
    }
}

impl Writeable for BitFlags64 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.0.write_to(writer)
    }
}

impl Readable for BitFlags128 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self(u128::read_from(reader)?))
    }
}

impl Writeable for BitFlags128 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.0.write_to(writer)
    }
}

impl Readable for f32 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        let value = Self::from_be_bytes(buffer);
        Ok(value)
    }
}

impl Writeable for f32 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        let bytes = self.to_be_bytes();
        writer.write_all(&bytes)?;
        Ok(std::mem::size_of::<Self>() as u64)
    }
}

impl Readable for f64 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        let value = Self::from_be_bytes(buffer);
        Ok(value)
    }
}

impl Writeable for f64 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        let bytes = self.to_be_bytes();
        writer.write_all(&bytes)?;
        Ok(std::mem::size_of::<Self>() as u64)
    }
}

impl Readable for Direction {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let dir: u8 = u8::read_from(reader)?;
        // NegX = 4,
        // NegY = 3,
        // NegZ = 5,
        // PosX = 1,
        // PosY = 0,
        // PosZ = 2,
        const NEGX: u8 = Direction::NegX as u8;
        const NEGY: u8 = Direction::NegY as u8;
        const NEGZ: u8 = Direction::NegZ as u8;
        const POSX: u8 = Direction::PosX as u8;
        const POSY: u8 = Direction::PosY as u8;
        const POSZ: u8 = Direction::PosZ as u8;
        use Direction::*;
        Ok(match dir {
            NEGX => NegX,
            NEGY => NegY,
            NEGZ => NegZ,
            POSX => PosX,
            POSY => PosY,
            POSZ => PosZ,
            _ => return Err(Error::InvalidBinaryFormat),
        })
    }
}

impl Writeable for Direction {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        (*self as u8).write_to(writer)
    }
}

impl Readable for Cardinal {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let id = u8::read_from(reader)?;
        Ok(match id {
            0 => Cardinal::West,
            1 => Cardinal::North,
            2 => Cardinal::East,
            3 => Cardinal::South,
            _ => return Err(Error::InvalidBinaryFormat),
        })
    }
}

impl Writeable for Cardinal {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        match self {
            Cardinal::West => 0u8.write_to(writer),
            Cardinal::North => 1u8.write_to(writer),
            Cardinal::East => 2u8.write_to(writer),
            Cardinal::South => 3u8.write_to(writer),
        }
    }
}

impl Readable for Rotation {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Rotation(u8::read_from(reader)?))
    }
}

impl Writeable for Rotation {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.0.write_to(writer)
    }
}

impl Readable for Flip {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Flip(u8::read_from(reader)?))
    }
}

impl Writeable for Flip {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.0.write_to(writer)
    }
}

impl Readable for Orientation {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let packed = u8::read_from(reader)?;
        Ok(Orientation(packed))
    }
}

impl Writeable for Orientation {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.0.write_to(writer)
    }
}

impl Readable for Axis {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let axis: u8 = u8::read_from(reader)?;
        const X: u8 = Axis::X as u8;
        const Y: u8 = Axis::Y as u8;
        const Z: u8 = Axis::Z as u8;
        Ok(match axis {
            X => Axis::X,
            Y => Axis::Y,
            Z => Axis::Z,
            _ => return Err(Error::InvalidBinaryFormat)
        })
    }
}

impl Writeable for Axis {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        (*self as u8).write_to(writer)
    }
}

impl Readable for Rgb {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut rgb = [0u8; 3];
        reader.read_exact(&mut rgb)?;
        let [r,g,b] = rgb;
        Ok(Rgb::new(r,g,b))
    }
}

impl Writeable for Rgb {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        let buf = [self.r, self.g, self.b];
        writer.write_all(&buf)?;
        Ok(3)
    }
}

impl Readable for Rgba {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let [r, g, b, a] = buffer;
        Ok(Rgba::new(r, g, b, a))
    }
}

impl Writeable for Rgba {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        let buf = [self.r, self.g, self.b, self.a];
        writer.write_all(&buf)?;
        Ok(4)
    }
}

impl Readable for IVec2 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(glam::ivec2(i32::read_from(reader)?, i32::read_from(reader)?))
    }
}

impl Writeable for IVec2 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.x.write_to(writer)?;
        self.y.write_to(writer)?;
        Ok(8)
    }
}

impl Readable for IVec3 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(glam::ivec3(i32::read_from(reader)?, i32::read_from(reader)?, i32::read_from(reader)?))
    }
}

impl Writeable for IVec3 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.x.write_to(writer)?;
        self.y.write_to(writer)?;
        self.z.write_to(writer)?;
        Ok(12)
    }
}

impl Readable for IVec4 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(glam::ivec4(
            i32::read_from(reader)?,
            i32::read_from(reader)?,
            i32::read_from(reader)?,
            i32::read_from(reader)?,
        ))
    }
}

impl Writeable for IVec4 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.x.write_to(writer)?;
        self.y.write_to(writer)?;
        self.z.write_to(writer)?;
        self.w.write_to(writer)?;
        Ok(16)
    }
}

impl Readable for Vec2 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(glam::vec2(f32::read_from(reader)?, f32::read_from(reader)?))
    }
}

impl Writeable for Vec2 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.x.write_to(writer)?;
        self.y.write_to(writer)?;
        Ok(8)
    }
}

impl Readable for Vec3 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(glam::vec3(
            f32::read_from(reader)?,
            f32::read_from(reader)?,
            f32::read_from(reader)?
        ))
    }
}

impl Writeable for Vec3 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.x.write_to(writer)?;
        self.y.write_to(writer)?;
        self.z.write_to(writer)?;
        Ok(12)
    }
}

impl Readable for Vec4 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(glam::vec4(
            f32::read_from(reader)?,
            f32::read_from(reader)?,
            f32::read_from(reader)?,
            f32::read_from(reader)?
        ))
    }
}

impl Writeable for Vec4 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.x.write_to(writer)?;
        self.y.write_to(writer)?;
        self.z.write_to(writer)?;
        self.w.write_to(writer)?;
        Ok(16)
    }
}

impl Readable for Mat2 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        // 0 2
        // 1 3
        let mut data = [f32::NAN; 4];
        for i in 0..4 {
            data[i] = f32::read_from(reader)?;
        }
        Ok(mat2(glam::vec2(data[0], data[1]), glam::vec2(data[2], data[3])))
    }
}

impl Writeable for Mat2 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        let data = self.to_cols_array();
        for i in 0..4 {
            data[i].write_to(writer)?;
        }
        Ok(4*4)
    }
}

impl Readable for Mat3 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut cols = [f32::NAN; 9];
        for i in 0..9 {
            cols[i] = f32::read_from(reader)?;
        }
        Ok(mat3(
            glam::vec3(
                cols[0],
                cols[1],
                cols[2]
            ),
            glam::vec3(
                cols[3],
                cols[4],
                cols[5]
            ),
            glam::vec3(
                cols[6],
                cols[7],
                cols[8]
            )
        ))
    }
}

impl Writeable for Mat3 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        let col = self.col(0);
        col.x.write_to(writer)?;
        col.y.write_to(writer)?;
        col.z.write_to(writer)?;
        let col = self.col(1);
        col.x.write_to(writer)?;
        col.y.write_to(writer)?;
        col.z.write_to(writer)?;
        let col = self.col(2);
        col.x.write_to(writer)?;
        col.y.write_to(writer)?;
        col.z.write_to(writer)?;
        Ok(4 * 3*3)
    }
}

impl Readable for Mat4 {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut cols = [f32::NAN; 16];
        for i in 0..16 {
            cols[i] = f32::read_from(reader)?;
        }
        Ok(mat4(
            glam::vec4(
                cols[0],
                cols[1],
                cols[2],
                cols[3]
            ),
            glam::vec4(
                cols[4],
                cols[5],
                cols[6],
                cols[7]
            ),
            glam::vec4(
                cols[8],
                cols[9],
                cols[10],
                cols[11]
            ),
            glam::vec4(
                cols[12],
                cols[13],
                cols[14],
                cols[15]
            )
        ))
    }
}

impl Writeable for Mat4 {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        let col = self.col(0);
        col.x.write_to(writer)?;
        col.y.write_to(writer)?;
        col.z.write_to(writer)?;
        col.w.write_to(writer)?;
        let col = self.col(1);
        col.x.write_to(writer)?;
        col.y.write_to(writer)?;
        col.z.write_to(writer)?;
        col.w.write_to(writer)?;
        let col = self.col(2);
        col.x.write_to(writer)?;
        col.y.write_to(writer)?;
        col.z.write_to(writer)?;
        col.w.write_to(writer)?;
        let col = self.col(3);
        col.x.write_to(writer)?;
        col.y.write_to(writer)?;
        col.z.write_to(writer)?;
        col.w.write_to(writer)?;
        Ok(64)
    }
}

impl Readable for Quat {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(quat(
            f32::read_from(reader)?,
            f32::read_from(reader)?,
            f32::read_from(reader)?,
            f32::read_from(reader)?
        ))
    }
}

impl Writeable for Quat {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.x.write_to(writer)?;
        self.y.write_to(writer)?;
        self.z.write_to(writer)?;
        self.w.write_to(writer)?;
        Ok(16)
    }
}

impl Readable for Bounds2D {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Bounds2D {
            min: (i32::read_from(reader)?,i32::read_from(reader)?),
            max: (i32::read_from(reader)?,i32::read_from(reader)?)
        })
    }
}

impl Writeable for Bounds2D {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.min.0.write_to(writer)?;
        self.min.1.write_to(writer)?;
        self.max.0.write_to(writer)?;
        self.max.1.write_to(writer)?;
        Ok(4 * 2 * 2)
    }
}

impl Readable for Bounds3D {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Bounds3D {
            min: (i32::read_from(reader)?,i32::read_from(reader)?,i32::read_from(reader)?),
            max: (i32::read_from(reader)?,i32::read_from(reader)?,i32::read_from(reader)?)
        })
    }
}

impl Writeable for Bounds3D {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.min.0.write_to(writer)?;
        self.min.1.write_to(writer)?;
        self.min.2.write_to(writer)?;
        self.max.0.write_to(writer)?;
        self.max.1.write_to(writer)?;
        self.max.2.write_to(writer)?;
        Ok(4 * 3 * 2)
    }
}

impl Readable for String {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[1..4])?;
        let length = u32::from_be_bytes(buf);
        let bytes = read_bytes(reader, length as usize)?;
        Ok(String::from_utf8(bytes)?)
    }
}

impl Writeable for String {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > MAX_ARRAY_LENGTH {
            return Err(Error::StringTooLong);
        }
        let buf = (self.len() as u32).to_be_bytes();
        writer.write_all(&buf[1..4])?;
        write_bytes(writer, self.as_bytes())?;
        Ok(self.len() as u64 + 3)
    }
}

impl Writeable for &str {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > MAX_ARRAY_LENGTH {
            return Err(Error::StringTooLong);
        }
        let buf = (self.len() as u32).to_be_bytes();
        writer.write_all(&buf[1..4])?;
        write_bytes(writer, self.as_bytes())?;
        Ok(self.len() as u64 + 3)
    }
}

impl Readable for std::ops::Range<i64> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(i64::read_from(reader)?..i64::read_from(reader)?)
    }
}

impl Writeable for std::ops::Range<i64> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.start.write_to(writer)?;
        self.end.write_to(writer)?;
        Ok(16)
    }
}

impl Readable for std::ops::RangeInclusive<i64> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(i64::read_from(reader)?..=i64::read_from(reader)?)
    }
}

impl Writeable for std::ops::RangeInclusive<i64> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.start().write_to(writer)?;
        self.end().write_to(writer)?;
        Ok(16)
    }
}

impl<T: Readable + NonByte> Readable for Vec<T> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[1..4])?;
        let length = u32::from_be_bytes(buf);
        let mut result = Vec::new();
        for _ in 0..length {
            result.push(T::read_from(reader)?);
        }
        Ok(result)
    }
}

impl<T: Writeable + NonByte> Writeable for Vec<T> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > MAX_ARRAY_LENGTH {
            return Err(Error::ArrayTooLong);
        }
        let buf = (self.len() as u32).to_be_bytes();
        writer.write_all(&buf[1..4])?;
        let mut length = 0;
        for i in 0..self.len() {
            length += self[i].write_to(writer)?;
        }
        Ok(length + 3)
    }
}

impl<T: Readable> Readable for Box<T> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Box::new(T::read_from(reader)?))
    }
}

impl<T: Writeable> Writeable for Box<T> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        self.as_ref().write_to(writer)
    }
}

impl Readable for Vec<bool> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[1..4])?;
        let length = u32::from_be_bytes(buf);
        let bytes = read_bytes(reader, length as usize)?;
        Ok(bytes.into_iter().map(|b| b != 0).collect())
    }
}

impl Writeable for Vec<bool> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > MAX_ARRAY_LENGTH {
            return Err(Error::ArrayTooLong);
        }
        let buf = (self.len() as u32).to_be_bytes();
        writer.write_all(&buf[1..4])?;
        let bytes: &[u8] = bytemuck::cast_slice(&self.as_slice());
        writer.write_all(&bytes)?;
        Ok(self.len() as u64 + 3)
    }
}

impl Readable for Vec<AxisFlags> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[1..4])?;
        let length = u32::from_be_bytes(buf);
        let bytes = read_bytes(reader, length as usize)?;
        Ok(bytes.into_iter().map(|b| AxisFlags::from_u8(b)).collect())
    }
}

impl Writeable for Vec<AxisFlags> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > MAX_ARRAY_LENGTH {
            return Err(Error::ArrayTooLong);
        }
        let buf = (self.len() as u32).to_be_bytes();
        writer.write_all(&buf[1..4])?;
        let bytes: &[u8] = bytemuck::cast_slice(&self.as_slice());
        writer.write_all(&bytes)?;
        Ok(self.len() as u64 + 3)
    }
}

impl Readable for Vec<FaceFlags> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[1..4])?;
        let length = u32::from_be_bytes(buf);
        let bytes = read_bytes(reader, length as usize)?;
        Ok(bytes.into_iter().map(|b| FaceFlags::from(b)).collect())
    }
}

impl Writeable for Vec<FaceFlags> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > MAX_ARRAY_LENGTH {
            return Err(Error::ArrayTooLong);
        }
        let buf = (self.len() as u32).to_be_bytes();
        writer.write_all(&buf[1..4])?;
        let bytes: &[u8] = bytemuck::cast_slice(&self.as_slice());
        writer.write_all(&bytes)?;
        Ok(self.len() as u64 + 3)
    }
}

impl Readable for Vec<BitFlags8> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[1..4])?;
        let length = u32::from_be_bytes(buf);
        let bytes = read_bytes(reader, length as usize)?;
        Ok(bytes.into_iter().map(|b| BitFlags8(b)).collect())
    }
}

impl Writeable for Vec<BitFlags8> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > MAX_ARRAY_LENGTH {
            return Err(Error::ArrayTooLong);
        }
        let buf = (self.len() as u32).to_be_bytes();
        writer.write_all(&buf[1..4])?;
        let bytes: &[u8] = bytemuck::cast_slice(&self.as_slice());
        writer.write_all(&bytes)?;
        Ok(self.len() as u64 + 3)
    }
}

impl Readable for Vec<u8> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[1..4])?;
        let length = u32::from_be_bytes(buf);
        read_bytes(reader, length as usize)
    }
}

impl Writeable for Vec<u8> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > MAX_ARRAY_LENGTH {
            return Err(Error::ArrayTooLong);
        }
        let buf = (self.len() as u32).to_be_bytes();
        writer.write_all(&buf[1..4])?;
        writer.write_all(self.as_slice())?;
        Ok(self.len() as u64 + 3)
    }
}

impl Readable for Vec<i8> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[1..4])?;
        let length = u32::from_be_bytes(buf);
        Ok(read_bytes(reader, length as usize)?.into_iter().map(|b| b as i8).collect())
    }
}

impl Writeable for Vec<i8> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > MAX_ARRAY_LENGTH {
            return Err(Error::ArrayTooLong);
        }
        let buf = (self.len() as u32).to_be_bytes();
        writer.write_all(&buf[1..4])?;
        writer.write_all(bytemuck::cast_slice(self.as_slice()))?;
        Ok(self.len() as u64 + 3)
    }
}

impl Readable for Vec<Direction> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[1..4])?;
        let length = u32::from_be_bytes(buf);
        let bytes = read_bytes(reader, length as usize)?;
        const NEGX: u8 = Direction::NegX as u8;
        const NEGY: u8 = Direction::NegY as u8;
        const NEGZ: u8 = Direction::NegZ as u8;
        const POSX: u8 = Direction::PosX as u8;
        const POSY: u8 = Direction::PosY as u8;
        const POSZ: u8 = Direction::PosZ as u8;
        Ok(bytes.into_iter().map(|b| match b {
            NEGX => Direction::NegX,
            NEGY => Direction::NegY,
            NEGZ => Direction::NegZ,
            POSX => Direction::PosX,
            POSY => Direction::PosY,
            POSZ => Direction::PosZ,
            _ => panic!("Invalid binary format for Direction (sorry for the panic)"),
        }).collect())
    }
}

impl Writeable for Vec<Direction> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > MAX_ARRAY_LENGTH {
            return Err(Error::ArrayTooLong);
        }
        let buf = (self.len() as u32).to_be_bytes();
        writer.write_all(&buf[1..4])?;
        writer.write_all(bytemuck::cast_slice(self.as_slice()))?;
        Ok(self.len() as u64 + 3)
    }
}

impl Readable for Vec<Cardinal> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[1..4])?;
        let length = u32::from_be_bytes(buf);
        let bytes = read_bytes(reader, length as usize/*  / 4 + (length % 4 != 0) as usize */)?;
        bytes.into_iter().map(|b| {
            Ok(match b {
                0 => Cardinal::West,
                1 => Cardinal::North,
                2 => Cardinal::East,
                3 => Cardinal::West,
                _ => return Err(Error::InvalidBinaryFormat)
            })
        }).collect()
    }
}

impl Writeable for Vec<Cardinal> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > MAX_ARRAY_LENGTH {
            return Err(Error::ArrayTooLong);
        }
        let buf = (self.len() as u32).to_be_bytes();
        writer.write_all(&buf[1..4])?;
        writer.write_all(bytemuck::cast_slice(self.as_slice()))?;
        Ok(self.len() as u64 + 3)
    }
}

macro_rules! read_tuple_impls {
    ($($tn:ident),+) => {
        impl<$($tn: Readable),*> Readable for ($($tn),*) {
            fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
                Ok((
                    $(
                        $tn::read_from(reader)?,
                    )*
                ))
            }
        }
    };
}

impl Readable for Vec<Rotation> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[1..4])?;
        let length = u32::from_be_bytes(buf);
        let bytes = read_bytes(reader, length as usize)?;
        Ok(bytes.into_iter().map(|b| Rotation(b)).collect())
    }
}

impl Writeable for Vec<Rotation> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > MAX_ARRAY_LENGTH {
            return Err(Error::ArrayTooLong);
        }
        let buf = (self.len() as u32).to_be_bytes();
        writer.write_all(&buf[1..4])?;
        writer.write_all(bytemuck::cast_slice(self.as_slice()))?;
        Ok(self.len() as u64 + 3)
    }
}

impl Readable for Vec<Flip> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let length = read_u24(reader)?;
        let bit_width = 3 * length;
        let byte_count = bit_width / 8 + (bit_width % 8 != 0) as u32;
        let bytes = read_bytes(reader, byte_count as usize)?;
        let mut flips = (0..length).map(|_| Flip::NONE).collect_vec();
        struct BitReader<'a> {
            flips: &'a mut [Flip],
            index: usize,
            accum: u8,
            accum_size: u32,
        }
        impl<'a> BitReader<'a> {
            const BIT_WIDTH: u32 = 3;
            fn push_value(&mut self, value: u8) {
                self.flips[self.index] = Flip(value);
                self.index += 1;
            }

            fn push_bits(&mut self, bits: u8, count: u32) {
                if self.index == self.flips.len() {
                    return;
                }
                let space = Self::BIT_WIDTH - self.accum_size;
                if space >= count {
                    let start = self.accum_size;
                    let end = start + count;
                    self.accum = self.accum.set_bitmask(start..end, bits);
                    self.accum_size += count;
                    if self.accum_size == Self::BIT_WIDTH {
                        self.push_value(self.accum);
                        self.accum = 0;
                        self.accum_size = 0;
                    }
                } else {
                    self.push_bits(bits, space);
                    self.push_bits(bits >> space, count - space);
                }
            }
        }
        let mut bitreader = BitReader {
            flips: &mut flips,
            index: 0,
            accum: 0,
            accum_size: 0
        };
        bytes.into_iter().for_each(|byte| bitreader.push_bits(byte, 8));
        assert_eq!(bitreader.flips.len(), bitreader.index);
        Ok(flips)
    }
}

impl Writeable for Vec<Flip> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > 0xffffff {
            return Err(Error::ArrayTooLong);
        }
        let length = write_u24(writer, self.len() as u32)?;
        let bit_width = 3 * self.len() as u32;
        let byte_count = bit_width / 8 + (bit_width % 8 != 0) as u32;
        let mut bytes = (0..byte_count).map(|_| 0u8).collect::<Box<_>>();
        struct BitWriter<'a> {
            bytes: &'a mut [u8],
            index: usize,
            accum: u8,
            accum_size: u32,
        }
        impl<'a> BitWriter<'a> {
            const BIT_WIDTH: u32 = 3;
            fn push_byte(&mut self, byte: u8) {
                self.bytes[self.index] = byte;
                self.index += 1;
            }

            fn push_flip(&mut self, Flip(flip): Flip) {
                let end_size = 8 - self.accum_size;
                if end_size < Self::BIT_WIDTH {
                    let mask = flip.get_bitmask(0..end_size) as u8;
                    self.push_byte(self.accum | mask << self.accum_size);
                    self.accum = flip >> end_size;
                    self.accum_size = Self::BIT_WIDTH - end_size;
                } else { // end_size >= bit_width
                    let start = self.accum_size;
                    let end = start + Self::BIT_WIDTH;
                    self.accum = self.accum.set_bitmask(start..end, flip);
                    self.accum_size += Self::BIT_WIDTH;
                    if self.accum_size == 8 {
                        self.push_byte(self.accum);
                        self.accum = 0;
                        self.accum_size = 0;
                    }
                }
            }
        }
        let mut bitwriter = BitWriter {
            bytes: &mut bytes,
            index: 0,
            accum: 0,
            accum_size: 0
        };
        self.iter().cloned().for_each(|flip| {
            bitwriter.push_flip(flip);
        });
        if bitwriter.accum_size > 0 {
            bitwriter.push_byte(bitwriter.accum);
        }
        Ok(length + write_bytes(writer, &bytes)?)
    }
}

impl Readable for Vec<Orientation> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let length = read_u24(reader)?;
        let bytes = read_bytes(reader, length as usize)?;
        Ok(bytes.into_iter().map(|packed| Orientation(packed)).collect())
    }
}

impl Writeable for Vec<Orientation> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > MAX_ARRAY_LENGTH {
            return Err(Error::ArrayTooLong);
        }
        let length = write_u24(writer, self.len() as u32)?;
        let bytes = self.iter().cloned().map(|orientation| orientation.0).collect_vec();
        Ok(length + write_bytes(writer, &bytes)?)
    }
}

impl Readable for Vec<Axis> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[1..4])?;
        let length = u32::from_be_bytes(buf);
        let bytes = read_bytes(reader, length as usize)?;
        Ok(bytes.into_iter().map(|b| match b {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            _ => panic!("Invalid binary format for Axis")
        }).collect())
    }
}

impl Writeable for Vec<Axis> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        if self.len() > MAX_ARRAY_LENGTH {
            return Err(Error::ArrayTooLong);
        }
        let buf = (self.len() as u32).to_be_bytes();
        writer.write_all(&buf[1..4])?;
        writer.write_all(bytemuck::cast_slice(self.as_slice()))?;
        Ok(self.len() as u64 + 3)
    }
}

impl Readable for hashbrown::HashMap<String, Tag> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        // read name, read tag, use 255 as Stop
        let mut map = hashbrown::HashMap::new();
        loop {
            let id: u8 = u8::read_from(reader)?;
            // Stop
            if id == 255 {
                break;
            }
            let name: String = String::read_from(reader)?;
            let tag: Tag = Tag::read_from(reader)?;
            map.insert(name, tag);
        }
        Ok(map)
    }
}

impl Writeable for hashbrown::HashMap<String, Tag> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        let size = self.iter().try_fold(0u64, |mut size, (name, tag)| {
            size += tag.id().write_to(writer)?;
            size += name.write_to(writer)?;
            size += tag.write_to(writer)?;
            Result::Ok(size)
        })?;
        255u8.write_to(writer)?;
        // writer.write_value(&255u8)?;
        Ok(size + 1)
    }
}

macro_rules! write_tuple_impls {
    ($($tn:ident),+) => {
        impl<$($tn: Writeable,)*> Writeable for ($($tn),*) {
            fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
                let mut length = 0;
                paste!{
                    #[allow(non_snake_case)]
                    let (
                        $(
                            [<_ $tn>],
                        )*
                    ) = self;
                    $(
                        length += [<_ $tn>].write_to(writer)?;
                    )*
                    Ok(length)
                }
            }

        }
    };
}

macro_rules! tuple_impls {
    ($($tn:ident),+) => {
        read_tuple_impls!($($tn),*);
        write_tuple_impls!($($tn),*);
    };
}

tuple_impls!(T0, T1);
tuple_impls!(T0, T1, T2);
tuple_impls!(T0, T1, T2, T3);
tuple_impls!(T0, T1, T2, T3, T4);
tuple_impls!(T0, T1, T2, T3, T4, T5);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);