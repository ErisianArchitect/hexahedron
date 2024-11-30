use serde::{Serialize, Deserialize};
use std::{borrow::Cow, rc::Rc, sync::Arc};

use crate::io::*;

#[rustfmt::skip]
macro_rules! property_table {
    ($macro:path) => {
        $macro! {
            [01     Bool(bool)                                          ]
            [02     Int(i64)                                            ]
            [03     String(String)                                      ]
            [04     Direction(crate::voxel::direction::Direction)       ]
            [05     Cardinal(crate::voxel::cardinal::Cardinal)          ]
            [06     Rotation(crate::voxel::orientation::Rotation)       ]
            [07     Flip(crate::voxel::orientation::Flip)               ]
            [08     Orientation(crate::voxel::orientation::Orientation) ]
            [09     Axis(crate::math::axis::Axis)                       ]
            [10     IVec2(glam::IVec2)                                  ]
            [11     IVec3(glam::IVec3)                                  ]
            [12     IVec4(glam::IVec4)                                  ]
            [13     FaceFlags(crate::voxel::face_flags::FaceFlags)      ]
            [14     AxisFlags(crate::math::axis_flags::AxisFlags)       ]
            [15     BitFlags8(crate::math::bit::BitFlags8)              ]
            [16     BitFlags16(crate::math::bit::BitFlags16)            ]
            [17     BitFlags32(crate::math::bit::BitFlags32)            ]
            [18     BitFlags64(crate::math::bit::BitFlags64)            ]
            [19     BitFlags128(crate::math::bit::BitFlags128)          ]
            [20     Color(crate::rendering::color::Color)               ]
            [21     Rgb(crate::rendering::color::Rgb)                   ]
            [22     Rgba(crate::rendering::color::Rgba)                 ]
            [23     Range(std::ops::Range<i64>)                         ]
            [24     RangeInclusive(std::ops::RangeInclusive<i64>)       ]
            [25     Bytes(Vec<u8>)                                      ]
            [26     Map(std::collections::BTreeMap<String, Property>)   ]
            // [27     Array(PropertyArray)                                ]
            /* 28   PropertyArray::Any(Vec<Property>) */
        }
    };
}

macro_rules! build_property_enum {
    ($([$id:literal $name:ident($type:ty)])+) => {
        #[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[repr(u8)]
        pub enum Property {
            #[default]
            Null = 0,
            $(
                $name($type) = $id,
            )+
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[repr(u8)]
        pub enum PropertyArray {
            #[default]
            Empty = 0,
            $(
                $name(Vec<$type>) = $id,
            )+
            Any(Vec<Property>) = 28,
        }

        impl Property {
            #[inline]
            pub const fn id(&self) -> u8 {
                match self {
                    Property::Null => 0,
                    $(
                        Property::$name(_) => $id,
                    )+
                }
            }

            pub(crate) fn read_with_id<R: std::io::Read>(id: u8, reader: &mut R) -> crate::prelude::VoxelResult<Self> {
                Ok(match id {
                    0 => Property::Null,
                    $(
                        $id => Property::$name(<$type>::read_from(reader)?),
                    )*
                    id => return Err(super::error::Error::InvalidPropertyId(id).into()),
                })
            }

            pub(crate) fn write_without_id<W: std::io::Write>(&self, writer: &mut W) -> crate::prelude::VoxelResult<u64> {
                Ok(match self {
                    Property::Null => 0,
                    $(
                        Property::$name(inner) => inner.write_to(writer)?,
                    )*
                })
            }
        }

        $(
            impl From<$type> for Property {
                #[inline]
                fn from(value: $type) -> Property {
                    Property::$name(value)
                }
            }

            impl TryInto<$type> for Property {
                type Error = super::error::Error;
                #[inline]
                fn try_into(self) -> Result<$type, Self::Error> {
                    if let Property::$name(value) = self {
                        Ok(value)
                    } else {
                        Err(super::error::Error::InvalidConversion)
                    }
                }
            }
        )+

        impl Readable for Property {
            fn read_from<R: std::io::Read>(reader: &mut R) -> crate::prelude::VoxelResult<Self> {
                let id = u8::read_from(reader)?;
                Self::read_with_id(id, reader)
            }
        }

        impl Writeable for Property {
            fn write_to<W: std::io::Write>(&self, writer: &mut W) -> crate::prelude::VoxelResult<u64> {
                self.id().write_to(writer)?;
                Ok(1 + self.write_without_id(writer)?)
            }
        }
    };
}

property_table!(build_property_enum);

impl Property {
    pub const NULL: Property = Property::Null;

    pub fn get_property(&self, name: &str) -> &Property {
        if let Self::Map(map) = self {
            map.get(name).unwrap_or_else(|| &Property::NULL)
        } else {
            &Property::NULL
        }
    }
}

impl<S: AsRef<str>> std::ops::Index<S> for Property {
    type Output = Property;
    fn index(&self, index: S) -> &Self::Output {
        self.get_property(index.as_ref())
    }
}

// impl From<(i32, i32)> for Property {
//     #[inline]
//     fn from(coord: (i32, i32)) -> Self {
//         Property::IVec2(coord)
//     }
// }

// impl From<(i32, i32, i32)> for Property {
//     #[inline]
//     fn from(coord: (i32, i32, i32)) -> Self {
//         Property::IVec3(coord)
//     }
// }

// impl From<(i32, i32, i32, i32)> for Property {
//     #[inline]
//     fn from(coord: (i32, i32, i32, i32)) -> Self {
//         Property::IVec4(coord)
//     }
// }

impl From<&str> for Property {
    #[inline]
    fn from(value: &str) -> Self {
        Property::String(value.to_owned())
    }
}

impl From<&String> for Property {
    #[inline]
    fn from(value: &String) -> Self {
        Property::String(value.clone())
    }
}

impl<'a> From<Cow<'a, str>> for Property {
    #[inline]
    fn from(value: Cow<'a, str>) -> Self {
        Property::String(value.into())
    }
}

impl From<Arc<str>> for Property {
    #[inline]
    fn from(value: Arc<str>) -> Self {
        Property::String(value.as_ref().to_owned())
    }
}

impl From<Rc<str>> for Property {
    #[inline]
    fn from(value: Rc<str>) -> Self {
        Property::String(value.as_ref().to_owned())
    }
}

impl From<Box<str>> for Property {
    #[inline]
    fn from(value: Box<str>) -> Self {
        Property::String(value.as_ref().to_owned())
    }
}

impl From<Box<[u8]>> for Property {
    #[inline]
    fn from(value: Box<[u8]>) -> Self {
        Property::Bytes(value.into_vec())
    }
}

impl<const SIZE: usize> From<[u8; SIZE]> for Property {
    #[inline]
    fn from(value: [u8; SIZE]) -> Self {
        Property::Bytes(value.into())
    }
}

impl<const SIZE: usize> From<&[u8; SIZE]> for Property {
    #[inline]
    fn from(value: &[u8; SIZE]) -> Self {
        Property::Bytes(value.into())
    }
}

impl From<&[u8]> for Property {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Property::Bytes(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockProperty {
    pub(in super) name: String,
    pub(in super) value: Property
}

impl BlockProperty {
    #[inline]
    pub fn new<S: Into<String>, P: Into<Property>>(name: S, value: P) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn value(&self) -> &Property {
        &self.value
    }
}

impl Readable for BlockProperty {
    fn read_from<R: std::io::Read>(reader: &mut R) -> crate::prelude::VoxelResult<Self> {
        let name = String::read_from(reader)?;
        let value = Property::read_from(reader)?;
        Ok(BlockProperty {
            name,
            value
        })
    }
}

impl Writeable for BlockProperty {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> crate::prelude::VoxelResult<u64> {
        Ok(
            self.name.write_to(writer)? +
            self.value.write_to(writer)?
        )
    }
}

impl PartialOrd for BlockProperty {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.name().partial_cmp(self.name())
    }
}

impl Ord for BlockProperty {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}