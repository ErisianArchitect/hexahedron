// pub mod region;
pub mod error;
use crate::error::{Error, Result};
// use crate::math::axis_flags::AxisFlags;
// use crate::voxel::block::block_property::Property;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};
use hexmacros::for_each_int_type;
// use crate::math::bit::*;
// use crate::rendering::color::{
//     Color, Rgb, Rgba
// };
// use crate::prelude::{
//     Axis, Cardinal, Direction, FaceFlags, Flip, Increment, Orientation, PropertyArray, Rotation
// };
// use crate::tag::{
//     NonByte,
//     Tag,
// };
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
// use rollgrid::{
//     bounds2d::Bounds2D,
//     bounds3d::Bounds3D,
// };
use paste::paste;
// use itertools::*;
// must use crate as hexahedron in order for it to be used by the deterministic macro.
use hexmacros::mark;

const MAX_ARRAY_LENGTH: usize = 0xffffff;

/// A marker trait that describes a Writeable type as being deterministic, which means that equal values of that type
/// will always result in the same binary representation.
/// 
/// Examples of non-deterministic types:
/// - HashMap, because the order of iteration is not guaranteed.
/// - HashSet, same as HashMap.
/// - ObjectPool, order is not preserved in the ObjectPool.
/// -- isize/usize, because their size is architecture-dependent.
pub trait Deterministic {}
pub trait NonByte {}
pub trait Byte {}

pub trait Readable: Sized {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self>;
}

pub trait Writeable {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64>;
}

impl<T: Writeable> Writeable for &T {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        (*self).write_to(writer)
    }
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

pub fn read_vec<T: Readable, R: std::io::Read>(reader: &mut R) -> Result<Vec<T>> {
    let length = read_u24(reader)?;
    let mut result = Vec::with_capacity(length as usize);
    for _ in 0..length {
        result.push(T::read_from(reader)?);
    }
    Ok(result)
}

pub fn write_slice<T: Writeable, W: std::io::Write>(writer: &mut W, slice: &[T]) -> Result<u64> {
    let mut length = write_u24(writer, slice.len() as u32)?;
    for item in slice {
        length += item.write_to(writer)?;
    }
    Ok(length)
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

for_each_int_type!(num_io; all !sized);

macro_rules! impl_byte {
    ($type:ty) => {
        impl Byte for $type {}
    };
}

macro_rules! impl_nonbyte {
    ($type:ty) => {
        impl NonByte for $type {}
    };
}

for_each_int_type!(impl_byte; 8);
for_each_int_type!(impl_nonbyte; all !8);

impl_byte!(bool);

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

impl_nonbyte!(char);

impl Readable for char {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let codepoint = u32::read_from(reader)?;
        char::from_u32(codepoint).ok_or(Error::InvalidCodepoint)
    }
}

impl Writeable for char {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        (*self as u32).write_to(writer)
    }
}

// impl Readable for AxisFlags {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let mut buffer = [0u8; 1];
//         reader.read_exact(&mut buffer)?;
//         Ok(AxisFlags::from_u8(buffer[0]))
//     }
// }

// impl Writeable for AxisFlags {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         self.inner().write_to(writer)
//     }
// }

// impl Readable for FaceFlags {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let mut buffer = [0u8; 1];
//         reader.read_exact(&mut buffer)?;
//         Ok(FaceFlags::from(buffer[0]))
//     }
// }

// impl Writeable for FaceFlags {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         self.inner().write_to(writer)
//     }
// }

// impl Readable for BitFlags8 {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let mut buffer = [0u8; 1];
//         reader.read_exact(&mut buffer)?;
//         Ok(BitFlags8(buffer[0]))
//     }
// }

// impl Writeable for BitFlags8 {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         let buffer = [self.0];
//         writer.write_all(&buffer)?;
//         Ok(1)
//     }
// }

// impl Readable for BitFlags16 {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         Ok(Self(u16::read_from(reader)?))
//     }
// }

// impl Writeable for BitFlags16 {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         self.0.write_to(writer)
//     }
// }

// impl Readable for BitFlags32 {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         Ok(Self(u32::read_from(reader)?))
//     }
// }

// impl Writeable for BitFlags32 {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         self.0.write_to(writer)
//     }
// }

// impl Readable for BitFlags64 {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         Ok(Self(u64::read_from(reader)?))
//     }
// }

// impl Writeable for BitFlags64 {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         self.0.write_to(writer)
//     }
// }

// impl Readable for BitFlags128 {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         Ok(Self(u128::read_from(reader)?))
//     }
// }

// impl Writeable for BitFlags128 {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         self.0.write_to(writer)
//     }
// }

impl_nonbyte!(f32);

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

impl_nonbyte!(f64);

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

// impl Readable for Direction {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let dir: u8 = u8::read_from(reader)?;
//         // NegX = 4,
//         // NegY = 3,
//         // NegZ = 5,
//         // PosX = 1,
//         // PosY = 0,
//         // PosZ = 2,
//         const NEGX: u8 = Direction::NegX as u8;
//         const NEGY: u8 = Direction::NegY as u8;
//         const NEGZ: u8 = Direction::NegZ as u8;
//         const POSX: u8 = Direction::PosX as u8;
//         const POSY: u8 = Direction::PosY as u8;
//         const POSZ: u8 = Direction::PosZ as u8;
//         use Direction::*;
//         Ok(match dir {
//             NEGX => NegX,
//             NEGY => NegY,
//             NEGZ => NegZ,
//             POSX => PosX,
//             POSY => PosY,
//             POSZ => PosZ,
//             _ => return Err(Error::InvalidBinaryFormat),
//         })
//     }
// }

// impl Writeable for Direction {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         (*self as u8).write_to(writer)
//     }
// }

// impl Readable for Cardinal {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let id = u8::read_from(reader)?;
//         Ok(match id {
//             0 => Cardinal::West,
//             1 => Cardinal::North,
//             2 => Cardinal::East,
//             3 => Cardinal::South,
//             _ => return Err(Error::InvalidBinaryFormat),
//         })
//     }
// }

// impl Writeable for Cardinal {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         match self {
//             Cardinal::West => 0u8.write_to(writer),
//             Cardinal::North => 1u8.write_to(writer),
//             Cardinal::East => 2u8.write_to(writer),
//             Cardinal::South => 3u8.write_to(writer),
//         }
//     }
// }

// impl Readable for Rotation {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         Ok(Rotation(u8::read_from(reader)?))
//     }
// }

// impl Writeable for Rotation {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         self.0.write_to(writer)
//     }
// }

// impl Readable for Flip {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         Ok(Flip(u8::read_from(reader)?))
//     }
// }

// impl Writeable for Flip {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         self.0.write_to(writer)
//     }
// }

// impl Readable for Orientation {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let packed = u8::read_from(reader)?;
//         Ok(Orientation(packed))
//     }
// }

// impl Writeable for Orientation {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         self.0.write_to(writer)
//     }
// }

// impl Readable for Axis {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let axis: u8 = u8::read_from(reader)?;
//         const X: u8 = Axis::X as u8;
//         const Y: u8 = Axis::Y as u8;
//         const Z: u8 = Axis::Z as u8;
//         Ok(match axis {
//             X => Axis::X,
//             Y => Axis::Y,
//             Z => Axis::Z,
//             _ => return Err(Error::InvalidBinaryFormat)
//         })
//     }
// }

// impl Writeable for Axis {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         (*self as u8).write_to(writer)
//     }
// }

// impl Readable for Color {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         Ok(Color::from_byte(u8::read_from(reader)?).unwrap_or(Color::Black))
//     }
// }

// impl Writeable for Color {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         self.to_byte().write_to(writer)
//     }
// }

// impl Readable for Rgb {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let mut rgb = [0u8; 3];
//         reader.read_exact(&mut rgb)?;
//         let [r,g,b] = rgb;
//         Ok(Rgb::new(r,g,b))
//     }
// }

// impl Writeable for Rgb {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         let buf = [self.r, self.g, self.b];
//         writer.write_all(&buf)?;
//         Ok(3)
//     }
// }

// impl Readable for Rgba {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let mut buffer = [0u8; 4];
//         reader.read_exact(&mut buffer)?;
//         let [r, g, b, a] = buffer;
//         Ok(Rgba::new(r, g, b, a))
//     }
// }

// impl Writeable for Rgba {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         let buf = [self.r, self.g, self.b, self.a];
//         writer.write_all(&buf)?;
//         Ok(4)
//     }
// }

impl_nonbyte!(IVec2);

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

impl_nonbyte!(IVec3);

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

impl_nonbyte!(IVec4);

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

impl_nonbyte!(Vec2);

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

impl_nonbyte!(Vec3);

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

impl_nonbyte!(Vec4);

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

impl_nonbyte!(Mat2);

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

impl_nonbyte!(Mat3);

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

impl_nonbyte!(Mat4);

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

impl_nonbyte!(Quat);

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

// impl Readable for Bounds2D {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         Ok(Bounds2D {
//             min: (i32::read_from(reader)?,i32::read_from(reader)?),
//             max: (i32::read_from(reader)?,i32::read_from(reader)?)
//         })
//     }
// }

// impl Writeable for Bounds2D {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         self.min.0.write_to(writer)?;
//         self.min.1.write_to(writer)?;
//         self.max.0.write_to(writer)?;
//         self.max.1.write_to(writer)?;
//         Ok(4 * 2 * 2)
//     }
// }

// impl Readable for Bounds3D {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         Ok(Bounds3D {
//             min: (i32::read_from(reader)?,i32::read_from(reader)?,i32::read_from(reader)?),
//             max: (i32::read_from(reader)?,i32::read_from(reader)?,i32::read_from(reader)?)
//         })
//     }
// }

// impl Writeable for Bounds3D {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         self.min.0.write_to(writer)?;
//         self.min.1.write_to(writer)?;
//         self.min.2.write_to(writer)?;
//         self.max.0.write_to(writer)?;
//         self.max.1.write_to(writer)?;
//         self.max.2.write_to(writer)?;
//         Ok(4 * 3 * 2)
//     }
// }

impl_nonbyte!(String);

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

impl_nonbyte!(&str);

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

impl<T: Readable + Writeable> NonByte for std::ops::Range<T> {}

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

impl<T: Readable + Writeable> NonByte for std::ops::RangeInclusive<T> {}

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

// TODO: HERE

impl<T: Readable + Writeable> NonByte for Vec<T> {}

impl<T: Readable + NonByte> Readable for Vec<T> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        read_vec(reader)
    }
}

impl<T: Writeable + NonByte> Writeable for Vec<T> {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
        write_slice(writer, self)
    }
}

// impl Readable for Vec<Property> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         read_vec(reader)
//     }
// }

// impl Writeable for Vec<Property> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         write_slice(writer, self)
//     }
// }

// impl Readable for Vec<PropertyArray> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         read_vec(reader)
//     }
// }

// impl Writeable for Vec<PropertyArray> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         write_slice(writer, self)
//     }
// }

// impl Readable for Vec<Vec<u8>> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         read_vec(reader)
//     }
// }

// impl Writeable for Vec<Vec<u8>> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         write_slice(writer, self)
//     }
// }

// impl Readable for Vec<BTreeMap<String, Property>> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         read_vec(reader)
//     }
// }

// impl Writeable for Vec<BTreeMap<String, Property>> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         write_slice(writer, self)
//     }
// }

impl<T: Readable + Writeable> NonByte for Box<T> {}

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

// impl Readable for Vec<AxisFlags> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let mut buf = [0u8; 4];
//         reader.read_exact(&mut buf[1..4])?;
//         let length = u32::from_be_bytes(buf);
//         let bytes = read_bytes(reader, length as usize)?;
//         Ok(bytes.into_iter().map(|b| AxisFlags::from_u8(b)).collect())
//     }
// }

// impl Writeable for Vec<AxisFlags> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         if self.len() > MAX_ARRAY_LENGTH {
//             return Err(Error::ArrayTooLong);
//         }
//         let buf = (self.len() as u32).to_be_bytes();
//         writer.write_all(&buf[1..4])?;
//         let bytes: &[u8] = bytemuck::cast_slice(&self.as_slice());
//         writer.write_all(&bytes)?;
//         Ok(self.len() as u64 + 3)
//     }
// }

// impl Readable for Vec<FaceFlags> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let mut buf = [0u8; 4];
//         reader.read_exact(&mut buf[1..4])?;
//         let length = u32::from_be_bytes(buf);
//         let bytes = read_bytes(reader, length as usize)?;
//         Ok(bytes.into_iter().map(|b| FaceFlags::from(b)).collect())
//     }
// }

// impl Writeable for Vec<FaceFlags> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         if self.len() > MAX_ARRAY_LENGTH {
//             return Err(Error::ArrayTooLong);
//         }
//         let buf = (self.len() as u32).to_be_bytes();
//         writer.write_all(&buf[1..4])?;
//         let bytes: &[u8] = bytemuck::cast_slice(&self.as_slice());
//         writer.write_all(&bytes)?;
//         Ok(self.len() as u64 + 3)
//     }
// }

// impl Readable for Vec<BitFlags8> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let mut buf = [0u8; 4];
//         reader.read_exact(&mut buf[1..4])?;
//         let length = u32::from_be_bytes(buf);
//         let bytes = read_bytes(reader, length as usize)?;
//         Ok(bytes.into_iter().map(|b| BitFlags8(b)).collect())
//     }
// }

// impl Writeable for Vec<BitFlags8> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         if self.len() > MAX_ARRAY_LENGTH {
//             return Err(Error::ArrayTooLong);
//         }
//         let buf = (self.len() as u32).to_be_bytes();
//         writer.write_all(&buf[1..4])?;
//         let bytes: &[u8] = bytemuck::cast_slice(&self.as_slice());
//         writer.write_all(&bytes)?;
//         Ok(self.len() as u64 + 3)
//     }
// }

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

// impl Readable for Vec<Color> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let mut buf = [0u8; 4];
//         reader.read_exact(&mut buf[1..4])?;
//         let length = u32::from_be_bytes(buf);
//         let bytes = read_bytes(reader, length as usize)?;
//         Ok(bytes.into_iter().map(|byte| Color::from_byte(byte).unwrap_or(Color::Black)).collect())
//     }
// }

// impl Writeable for Vec<Color> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         if self.len() > MAX_ARRAY_LENGTH {
//             return Err(Error::ArrayTooLong);
//         }
//         let buf = (self.len() as u32).to_be_bytes();
//         writer.write_all(&buf[1..4])?;
//         writer.write_all(bytemuck::cast_slice(self.as_slice()))?;
//         Ok(self.len() as u64 + 3)
//     }
// }

// impl Readable for Vec<Direction> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let mut buf = [0u8; 4];
//         reader.read_exact(&mut buf[1..4])?;
//         let length = u32::from_be_bytes(buf);
//         let bytes = read_bytes(reader, length as usize)?;
//         const NEGX: u8 = Direction::NegX as u8;
//         const NEGY: u8 = Direction::NegY as u8;
//         const NEGZ: u8 = Direction::NegZ as u8;
//         const POSX: u8 = Direction::PosX as u8;
//         const POSY: u8 = Direction::PosY as u8;
//         const POSZ: u8 = Direction::PosZ as u8;
//         Ok(bytes.into_iter().map(|b| match b {
//             NEGX => Direction::NegX,
//             NEGY => Direction::NegY,
//             NEGZ => Direction::NegZ,
//             POSX => Direction::PosX,
//             POSY => Direction::PosY,
//             POSZ => Direction::PosZ,
//             _ => panic!("Invalid binary format for Direction (sorry for the panic)"),
//         }).collect())
//     }
// }

// impl Writeable for Vec<Direction> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         if self.len() > MAX_ARRAY_LENGTH {
//             return Err(Error::ArrayTooLong);
//         }
//         let buf = (self.len() as u32).to_be_bytes();
//         writer.write_all(&buf[1..4])?;
//         writer.write_all(bytemuck::cast_slice(self.as_slice()))?;
//         Ok(self.len() as u64 + 3)
//     }
// }

// impl Readable for Vec<Cardinal> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let length = read_u24(reader)?;
//         if length == 0 {
//             return Ok(Vec::new());
//         }
//         let bit_len = length * 2;
//         let byte_count = if bit_len % 8 != 0 {
//             bit_len / 8 + 1
//         } else {
//             bit_len / 8
//         } as usize;
//         let bytes = read_bytes(reader, byte_count as usize)?;
//         // I'm taking advantage of the fact that Cardinal only takes up
//         // 2 bits, which means that 4 of them fit in a byte
//         // and there's no cross-byte splitting.
//         struct BitReader<'a> {
//             bytes: &'a [u8],
//             top: usize,
//         }
//         impl<'a> BitReader<'a> {
//             fn new(bytes: &'a [u8]) -> Self {
//                 Self {
//                     bytes,
//                     top: 0,
//                 }
//             }
//             fn read_next(&mut self) -> Result<Cardinal> {
//                 let byte_index = self.top / 8;
//                 let byte = self.bytes[byte_index];
//                 let sub_index = self.top % 8;
//                 let bits = (byte >> sub_index) & 0b11;
//                 self.top += 2;
//                 Ok(match bits {
//                     0 => Cardinal::West,
//                     1 => Cardinal::North,
//                     2 => Cardinal::East,
//                     3 => Cardinal::South,
//                     // We know this is unreachable because bits was & with 0b11.
//                     _ => unreachable!(),
//                 })
//             }
//         }
//         let mut bit_reader = BitReader::new(&bytes);
//         let mut accum = Vec::with_capacity(length as usize);
//         for _ in 0..length {
//             accum.push(bit_reader.read_next()?);
//         }
//         Ok(accum)
//     }
// }

// impl Writeable for Vec<Cardinal> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         if self.len() > MAX_ARRAY_LENGTH {
//             return Err(Error::ArrayTooLong);
//         }
//         struct BitWriter {
//             top: u32,
//             accum: u16,
//             bytes: Box<[u8]>,
//             index: usize,
//         }
//         impl BitWriter {
//             fn new(count: usize) -> Self {
//                 let bit_len = count * 2;
//                 let byte_count = if bit_len % 8 != 0 {
//                     bit_len / 8 + 1
//                 } else {
//                     bit_len / 8
//                 };
//                 Self {
//                     top: 0,
//                     accum: 0,
//                     index: 0,
//                     bytes: (0..byte_count).map(|_| 0u8).collect(),
//                 }
//             }

//             fn push_byte(&mut self, byte: u8) {
//                 self.bytes[self.index.post_increment()] = byte;
//             }

//             fn push_bits(&mut self, bits: u8) {
//                 let bits = bits as u16;
//                 self.accum |= bits << self.top;
//                 self.top += 2;
//                 if self.top >= 8 {
//                     let byte = (self.accum & 0xff) as u8;
//                     self.push_byte(byte);
//                     self.accum >>= 8;
//                     self.top -= 8;
//                 }
//             }

//             fn finish(mut self) -> Box<[u8]> {
//                 if self.top % 8 != 0 {
//                     self.push_byte((self.accum & 0xff) as u8);
//                 }
//                 self.bytes
//             }
//         }
//         let mut bit_writer = BitWriter::new(self.len());
//         self.iter().map(|&cardinal| {
//             match cardinal {
//                 Cardinal::West => 0u8,
//                 Cardinal::North => 1u8,
//                 Cardinal::East => 2u8,
//                 Cardinal::South => 3u8,
//             }
//         }).for_each(|bits| {
//             bit_writer.push_bits(bits);
//         });
//         Ok(
//             write_u24(writer, self.len() as u32)? +
//             write_bytes(writer, &bit_writer.finish())?
//         )
//     }
// }

// impl Readable for Vec<Rotation> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let mut buf = [0u8; 4];
//         reader.read_exact(&mut buf[1..4])?;
//         let length = u32::from_be_bytes(buf);
//         let bytes = read_bytes(reader, length as usize)?;
//         Ok(bytes.into_iter().map(|b| Rotation(b)).collect())
//     }
// }

// impl Writeable for Vec<Rotation> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         if self.len() > MAX_ARRAY_LENGTH {
//             return Err(Error::ArrayTooLong);
//         }
//         let buf = (self.len() as u32).to_be_bytes();
//         writer.write_all(&buf[1..4])?;
//         writer.write_all(bytemuck::cast_slice(self.as_slice()))?;
//         Ok(self.len() as u64 + 3)
//     }
// }

// impl Readable for Vec<Flip> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let length = read_u24(reader)?;
//         let bit_width = 3 * length;
//         let byte_count = bit_width / 8 + (bit_width % 8 != 0) as u32;
//         let bytes = read_bytes(reader, byte_count as usize)?;
//         let mut flips = (0..length).map(|_| Flip::NONE).collect_vec();
//         struct BitReader<'a> {
//             flips: &'a mut [Flip],
//             index: usize,
//             accum: u8,
//             accum_size: u32,
//         }
//         impl<'a> BitReader<'a> {
//             const BIT_WIDTH: u32 = 3;
//             fn push_value(&mut self, value: u8) {
//                 self.flips[self.index] = Flip(value);
//                 self.index += 1;
//             }

//             fn push_bits(&mut self, bits: u8, count: u32) {
//                 if self.index == self.flips.len() {
//                     return;
//                 }
//                 let space = Self::BIT_WIDTH - self.accum_size;
//                 if space >= count {
//                     let start = self.accum_size;
//                     let end = start + count;
//                     self.accum = self.accum.set_bitmask(start..end, bits);
//                     self.accum_size += count;
//                     if self.accum_size == Self::BIT_WIDTH {
//                         self.push_value(self.accum);
//                         self.accum = 0;
//                         self.accum_size = 0;
//                     }
//                 } else {
//                     self.push_bits(bits, space);
//                     self.push_bits(bits >> space, count - space);
//                 }
//             }
//         }
//         let mut bitreader = BitReader {
//             flips: &mut flips,
//             index: 0,
//             accum: 0,
//             accum_size: 0
//         };
//         bytes.into_iter().for_each(|byte| bitreader.push_bits(byte, 8));
//         debug_assert_eq!(bitreader.flips.len(), bitreader.index);
//         Ok(flips)
//     }
// }

// impl Writeable for Vec<Flip> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         if self.len() > 0xffffff {
//             return Err(Error::ArrayTooLong);
//         }
//         let length = write_u24(writer, self.len() as u32)?;
//         let bit_width = 3 * self.len() as u32;
//         let byte_count = bit_width / 8 + (bit_width % 8 != 0) as u32;
//         let mut bytes = (0..byte_count).map(|_| 0u8).collect::<Box<_>>();
//         struct BitWriter<'a> {
//             bytes: &'a mut [u8],
//             index: usize,
//             accum: u8,
//             accum_size: u32,
//         }
//         impl<'a> BitWriter<'a> {
//             const BIT_WIDTH: u32 = 3;
//             fn push_byte(&mut self, byte: u8) {
//                 self.bytes[self.index] = byte;
//                 self.index += 1;
//             }

//             fn push_flip(&mut self, Flip(flip): Flip) {
//                 let end_size = 8 - self.accum_size;
//                 if end_size < Self::BIT_WIDTH {
//                     let mask = flip.get_bitmask(0..end_size) as u8;
//                     self.push_byte(self.accum | mask << self.accum_size);
//                     self.accum = flip >> end_size;
//                     self.accum_size = Self::BIT_WIDTH - end_size;
//                 } else { // end_size >= bit_width
//                     let start = self.accum_size;
//                     let end = start + Self::BIT_WIDTH;
//                     self.accum = self.accum.set_bitmask(start..end, flip);
//                     self.accum_size += Self::BIT_WIDTH;
//                     if self.accum_size == 8 {
//                         self.push_byte(self.accum);
//                         self.accum = 0;
//                         self.accum_size = 0;
//                     }
//                 }
//             }
//         }
//         let mut bitwriter = BitWriter {
//             bytes: &mut bytes,
//             index: 0,
//             accum: 0,
//             accum_size: 0
//         };
//         self.iter().cloned().for_each(|flip| {
//             bitwriter.push_flip(flip);
//         });
//         if bitwriter.accum_size > 0 {
//             bitwriter.push_byte(bitwriter.accum);
//         }
//         Ok(length + write_bytes(writer, &bytes)?)
//     }
// }

// impl Readable for Vec<Orientation> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let length = read_u24(reader)?;
//         let bytes = read_bytes(reader, length as usize)?;
//         Ok(bytes.into_iter().map(|packed| Orientation(packed)).collect())
//     }
// }

// impl Writeable for Vec<Orientation> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         if self.len() > MAX_ARRAY_LENGTH {
//             return Err(Error::ArrayTooLong);
//         }
//         let length = write_u24(writer, self.len() as u32)?;
//         let bytes = self.iter().cloned().map(|orientation| orientation.0).collect_vec();
//         Ok(length + write_bytes(writer, &bytes)?)
//     }
// }

// impl Readable for Vec<Axis> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let length = read_u24(reader)?;
//         if length == 0 {
//             return Ok(Vec::new());
//         }
//         let bit_len = length * 2;
//         let byte_count = if bit_len % 8 != 0 {
//             bit_len / 8 + 1
//         } else {
//             bit_len / 8
//         } as usize;
//         let bytes = read_bytes(reader, byte_count as usize)?;
//         // I'm taking advantage of the fact that Axis only takes up
//         // 2 bits, which means that 4 of them fit in a byte
//         // and there's no cross-byte splitting.
//         struct BitReader<'a> {
//             bytes: &'a [u8],
//             top: usize,
//         }
//         impl<'a> BitReader<'a> {
//             fn new(bytes: &'a [u8]) -> Self {
//                 Self {
//                     bytes,
//                     top: 0,
//                 }
//             }
//             fn read_next(&mut self) -> Result<Axis> {
//                 let byte_index = self.top / 8;
//                 let byte = self.bytes[byte_index];
//                 let sub_index = self.top % 8;
//                 let bits = (byte >> sub_index) & 0b11;
//                 self.top += 2;
//                 Ok(match bits {
//                     0 => Axis::X,
//                     1 => Axis::Y,
//                     2 => Axis::Z,
//                     3 => return Err(Error::InvalidBinaryFormat),
//                     // We know this is unreachable because bits was & with 0b11.
//                     _ => unreachable!(),
//                 })
//             }
//         }
//         let mut bit_reader = BitReader::new(&bytes);
//         let mut accum = Vec::with_capacity(length as usize);
//         for _ in 0..length {
//             accum.push(bit_reader.read_next()?);
//         }
//         Ok(accum)
//     }
// }

// impl Writeable for Vec<Axis> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         if self.len() > MAX_ARRAY_LENGTH {
//             return Err(Error::ArrayTooLong);
//         }
//         struct BitWriter {
//             top: u32,
//             accum: u16,
//             bytes: Box<[u8]>,
//             index: usize,
//         }
//         impl BitWriter {
//             fn new(count: usize) -> Self {
//                 let bit_len = count * 2;
//                 let byte_count = if bit_len % 8 != 0 {
//                     bit_len / 8 + 1
//                 } else {
//                     bit_len / 8
//                 };
//                 Self {
//                     top: 0,
//                     accum: 0,
//                     index: 0,
//                     bytes: (0..byte_count).map(|_| 0u8).collect(),
//                 }
//             }

//             fn push_byte(&mut self, byte: u8) {
//                 self.bytes[self.index.post_increment()] = byte;
//             }

//             fn push_bits(&mut self, bits: u8) {
//                 let bits = bits as u16;
//                 self.accum |= bits << self.top;
//                 self.top += 2;
//                 if self.top >= 8 {
//                     let byte = (self.accum & 0xff) as u8;
//                     self.push_byte(byte);
//                     self.accum >>= 8;
//                     self.top -= 8;
//                 }
//             }

//             fn finish(mut self) -> Box<[u8]> {
//                 if self.top % 8 != 0 {
//                     self.push_byte((self.accum & 0xff) as u8);
//                 }
//                 self.bytes
//             }
//         }
//         let mut bit_writer = BitWriter::new(self.len());
//         self.iter().map(|&axis| {
//             match axis {
//                 Axis::X => 0u8,
//                 Axis::Y => 1u8,
//                 Axis::Z => 2u8,
//             }
//         }).for_each(|bits| {
//             bit_writer.push_bits(bits);
//         });
//         Ok(
//             write_u24(writer, self.len() as u32)? +
//             write_bytes(writer, &bit_writer.finish())?
//         )
//     }
// }

// impl Readable for hashbrown::HashMap<String, Tag> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         // read name, read tag, use 255 as Stop
//         let mut map = hashbrown::HashMap::new();
//         loop {
//             let id: u8 = u8::read_from(reader)?;
//             // Stop
//             if id == 255 {
//                 break;
//             }
//             let name: String = String::read_from(reader)?;
//             let tag: Tag = Tag::read_with_id(id, reader)?;
//             map.insert(name, tag);
//         }
//         Ok(map)
//     }
// }

// impl Writeable for hashbrown::HashMap<String, Tag> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         let size = self.iter().try_fold(0u64, |mut size, (name, tag)| {
//             size += tag.id().write_to(writer)?;
//             size += name.write_to(writer)?;
//             size += tag.write_without_id(writer)?;
//             Result::Ok(size)
//         })?;
//         255u8.write_to(writer)?;
//         Ok(size + 1)
//     }
// }

// impl Readable for BTreeMap<String, Tag> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let mut map = BTreeMap::new();
//         loop {
//             let id = u8::read_from(reader)?;
//             if id == 255 {
//                 break;
//             }
//             let name = String::read_from(reader)?;
//             let tag = Tag::read_with_id(id, reader)?;
//             map.insert(name, tag);
//         }
//         Ok(map)
//     }
// }

// impl Writeable for BTreeMap<String, Tag> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         let size = self.iter().try_fold(0u64, |mut size, (name, tag)| {
//             size += tag.id().write_to(writer)?;
//             size += name.write_to(writer)?;
//             size += tag.write_without_id(writer)?;
//             Result::Ok(size)
//         })?;
//         255u8.write_to(writer)?;
//         Ok(size + 1)
//     }
// }

// impl Readable for BTreeMap<String, Property> {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
//         let mut map = BTreeMap::new();
//         loop {
//             let id = u8::read_from(reader)?;
//             if id == 255 {
//                 break;
//             }
//             let name = String::read_from(reader)?;
//             let prop = Property::read_with_id(id, reader)?;
//             map.insert(name, prop);
//         }
//         Ok(map)
//     }
// }

// impl Writeable for BTreeMap<String, Property> {
//     fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
//         let size = self.iter().try_fold(0u64, |mut size, (name, prop)| {
//             size += prop.id().write_to(writer)?;
//             size += name.write_to(writer)?;
//             Result::Ok(size + prop.write_without_id(writer)?)
//         })?;
//         255u8.write_to(writer)?;
//         Ok(size + 1)
//     }
// }

macro_rules! read_tuple_impls {
    ($($tn:ident),+) => {
        impl<$($tn,)*> NonByte for ($($tn,)*) {}
        impl<$($tn: Readable),*> Readable for ($($tn,)*) {
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

macro_rules! write_tuple_impls {
    ($($tn:ident),+) => {
        impl<$($tn: Writeable,)*> Writeable for ($($tn,)*) {
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

macro_rules! deterministic_tuple_impls {
    ($($tn:ident),+) => {
        impl<$($tn: Deterministic),+> Deterministic for ($($tn,)*) {}
    };
}

macro_rules! tuple_impls {
    ($($tn:ident),+) => {
        read_tuple_impls!($($tn),*);
        write_tuple_impls!($($tn),*);
        deterministic_tuple_impls!($($tn),*);
    };
}

tuple_impls!(T0);
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
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);

mark!{trait = Deterministic;
    // primitive types
    bool; char;
    // integer types (isize and usize are not considered deterministic because they are not a fixed size across platforms)
    i8; i16; i32; i64; i128;
    u8; u16; u32; u64; u128;
    // String types
    String; &str;
    // Types with generics.
    <T: Deterministic> for &T;
    <T: Deterministic> for Box<T>;
    <T: Deterministic> for std::rc::Rc<T>;
    <T: Deterministic> for std::sync::Arc<T>;
    <T: Deterministic, const SIZE: usize> for [T; SIZE];
    <T: Deterministic> for &[T];
    <T: Deterministic> for Vec<T>;
    <T: Deterministic> for Box<[T]>;
    <T: Deterministic> for std::rc::Rc<[T]>;
    <T: Deterministic> for std::sync::Arc<[T]>;
    <K, V> for BTreeMap<K, V> where (K, V): Deterministic;
    <T: Deterministic> for BTreeSet<T>;
    <T: Deterministic> for std::ops::Range<T>;
    <T: Deterministic> for std::ops::RangeInclusive<T>;
    // hexahedron types.
    // Axis;
    // Orientation;
    // Flip;
    // Rotation;
    // Cardinal;
    // Direction;
    // Color;
    // Rgb;
    // Rgba;
    // BitFlags8;
    // BitFlags16;
    // BitFlags32;
    // BitFlags64;
    // BitFlags128;
    // FaceFlags;
    // AxisFlags;
    // Property;
    // PropertyArray;
    // glam
    IVec2;
    IVec3;
    IVec4;
    // rollgrid
    // Bounds2D;
    // Bounds3D;
}