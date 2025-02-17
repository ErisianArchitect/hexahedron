use serde::{Serialize, Deserialize};
use crate::error::{Error, Result};
use crate::io::{
    Readable,
    Writeable,
};
use std::io::{
    Read, Write,
};
use hexmacros::table;

pub trait NonByte {}
pub trait Byte {}

#[rustfmt::skip]
table!(macro tag_table {
    //  ID | Name | Byte-Size or Non-Byte-Size | Boxed | Type
    [ 1 Bool            Byte        unbox   <bool>                                  ]
    [ 2 BitFlags8       Byte        unbox   <crate::math::bit::BitFlags8>           ]
    [ 3 BitFlags16      NonByte     unbox   <crate::math::bit::BitFlags16>          ]
    [ 4 BitFlags32      NonByte     unbox   <crate::math::bit::BitFlags32>          ]
    [ 5 BitFlags64      NonByte     unbox   <crate::math::bit::BitFlags64>          ]
    [ 6 BitFlags128     NonByte     box     <crate::math::bit::BitFlags128>         ]
    [ 7 U8              Byte        unbox   <u8>                                    ]
    [ 8 I8              Byte        unbox   <i8>                                    ]
    [ 9 U16             NonByte     unbox   <u16>                                   ]
    [10 I16             NonByte     unbox   <i16>                                   ]
    [11 U32             NonByte     unbox   <u32>                                   ]
    [12 I32             NonByte     unbox   <i32>                                   ]
    [13 U64             NonByte     unbox   <u64>                                   ]
    [14 I64             NonByte     unbox   <i64>                                   ]
    [15 F32             NonByte     unbox   <f32>                                   ]
    [16 F64             NonByte     unbox   <f64>                                   ]
    [17 Direction       Byte        unbox   <crate::voxel::direction::Direction>    ]
    [18 Cardinal        Byte        unbox   <crate::voxel::cardinal::Cardinal>      ]
    [19 Rotation        Byte        unbox   <crate::voxel::orientation::Rotation>   ]
    [20 Flip            Byte        unbox   <crate::voxel::orientation::Flip>       ]
    [21 Orientation     Byte        unbox   <crate::voxel::orientation::Orientation>]
    [22 Axis            Byte        unbox   <crate::math::axis::Axis>               ]
    [23 AxisFlags       Byte        unbox   <crate::math::axis_flags::AxisFlags>    ]
    [24 FaceFlags       Byte        unbox   <crate::voxel::face_flags::FaceFlags>   ]
    [25 Color           Byte        unbox   <crate::rendering::color::Color>        ]
    [26 Rgb             NonByte     unbox   <crate::rendering::color::Rgb>          ]
    [27 Rgba            NonByte     unbox   <crate::rendering::color::Rgba>         ]
    [28 IVec2           NonByte     unbox   <glam::IVec2>                           ]
    [29 IVec3           NonByte     unbox   <glam::IVec3>                           ]
    [30 IVec4           NonByte     box     <glam::IVec4>                           ]
    [31 Vec2            NonByte     unbox   <glam::Vec2>                            ]
    [32 Vec3            NonByte     unbox   <glam::Vec3>                            ]
    [33 Vec4            NonByte     box     <glam::Vec4>                            ]
    [34 Mat2            NonByte     box     <glam::Mat2>                            ]
    [35 Mat3            NonByte     box     <glam::Mat3>                            ]
    [36 Mat4            NonByte     box     <glam::Mat4>                            ]
    [37 Quat            NonByte     box     <glam::Quat>                            ]
    [38 Bounds2         NonByte     box     <rollgrid::bounds2d::Bounds2D>          ]
    [39 Bounds3         NonByte     box     <rollgrid::bounds3d::Bounds3D>          ]
    [40 Range           NonByte     box     <std::ops::Range<i64>>                  ]
    [41 RangeInclusive  NonByte     box     <std::ops::RangeInclusive<i64>>         ]
    [42 String          NonByte     box     <String>                                ]
    [43 Array           NonByte     box     <crate::tag::Array>                     ]
    [44 Map             NonByte     box     <hashbrown::HashMap<String, Tag>>       ]
    /* This line should remain commented out.
    This represents a tag ID that I inserted manually into the generation code.
    (See: TAG_ARRAY_ID)
    [63 Tag           NonByte   box     <Tag>                                                   ]
    Continue writing new rows at index 41
    */
});

const TAG_ARRAY_ID: u8 = 63;

macro_rules! table_impls {
    ($({$id:literal $name:ident $impl:ident $box:ident <$type:ty> $($end:tt)*})*) => {
        // Blanket impls
        $(
            impl $impl for $type {}
        )*

        #[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
        #[repr(u8)]
        pub enum Tag {
            #[default]
            Null = 0,
            $(
                $name(table_impls!(@box_unbox: $box $type)) = $id,
            )*
        }

        impl Tag {
            pub const MAX_ID: u8 = {
                let mut max = 0u8;
                $(
                    if $id > max {
                        max = $id;
                    }
                )*
                max
            };

            pub fn id(&self) -> u8 {
                match self {
                    Tag::Null => 0,
                    $(
                        Tag::$name(_) => $id,
                    )*
                }
            }

            pub(crate) fn read_with_id<R: Read>(id: u8, reader: &mut R) -> Result<Self> {
                Ok(match id {
                    0 => Tag::Null,
                    $(
                        $id => Tag::$name(<table_impls!(@box_unbox: $box $type)>::read_from(reader)?),
                    )*
                    _ => return Err(Error::InvalidBinaryFormat),
                })
            }

            pub(crate) fn write_without_id<W: Write>(&self, writer: &mut W) -> Result<u64> {
                Ok(match self {
                    Tag::Null => 0,
                    $(
                        Tag::$name(value) => value.write_to(writer)?,
                    )*
                })
            }
        }

        impl Readable for Tag {
            fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
                let id: u8 = u8::read_from(reader)?;
                Tag::read_with_id(id, reader)
            }
        }

        impl Writeable for Tag {
            fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
                self.id().write_to(writer)?;
                Ok(self.write_without_id(writer)? + 1)
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
        #[repr(u8)]
        pub enum Array {
            #[default]
            Empty = 0,
            $(
                $name(Vec<$type>) = $id,
            )*
            Tag(Vec<Tag>) = TAG_ARRAY_ID,
        }

        impl Array {
            pub fn id(&self) -> u8 {
                match self {
                    Array::Empty => 0,
                    $(
                        Array::$name(_) => $id,
                    )*
                    Array::Tag(_) => TAG_ARRAY_ID,
                }
            }

            pub fn len(&self) -> usize {
                match self {
                    Array::Empty => 0,
                    $(
                        Array::$name(array) => array.len(),
                    )*
                    Array::Tag(array) => array.len(),
                }
            }
        }

        $(
            impl From<Vec<$type>> for Array {
                fn from(value: Vec<$type>) -> Self {
                    Array::$name(value)
                }
            }

            impl<const SIZE: usize> From<[$type; SIZE]> for Array {
                fn from(value: [$type; SIZE]) -> Self {
                    Array::$name(value.into())
                }
            }

            impl From<Vec<$type>> for Tag {
                fn from(value: Vec<$type>) -> Self {
                    Tag::Array(Box::new(value.into()))
                }
            }

            impl<const SIZE: usize> From<[$type; SIZE]> for Tag {
                fn from(value: [$type; SIZE]) -> Self {
                    Tag::Array(Box::new(value.into()))
                }
            }
        )*

        impl Readable for Array {
            fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
                let id: u8 = u8::read_from(reader)?;
                Ok(match id {
                    0 => Array::Empty,
                    $(
                        $id => Array::$name(Vec::<$type>::read_from(reader)?),
                    )*
                    TAG_ARRAY_ID => Array::Tag(Vec::<Tag>::read_from(reader)?),
                    _ => return Err(Error::InvalidBinaryFormat),
                })
            }
        }

        impl Writeable for Array {
            fn write_to<W: Write>(&self, writer: &mut W) -> Result<u64> {
                self.id().write_to(writer)?;
                Ok(match self {
                    Array::Empty => 0,
                    $(
                        Array::$name(array) => array.write_to(writer)?,
                    )*
                    Array::Tag(array) => array.write_to(writer)?,
                } + 1)
            }
        }
    };
    (@box_unbox: box $type:ty) => {
        Box<$type>
    };
    (@box_unbox: unbox $type:ty) => {
        $type
    };
}

tag_table!(table_impls);

impl NonByte for Tag {}

impl From<Vec<Tag>> for Array {
    fn from(value: Vec<Tag>) -> Self {
        Array::Tag(value)
    }
}

impl From<Vec<Tag>> for Tag {
    fn from(value: Vec<Tag>) -> Self {
        Tag::Array(Box::new(value.into()))
    }
}

impl<const SIZE: usize> From<[Tag; SIZE]> for Array {
    fn from(value: [Tag; SIZE]) -> Self {
        Array::Tag(value.into())
    }
}

impl<const SIZE: usize> From<[Tag; SIZE]> for Tag {
    fn from(value: [Tag; SIZE]) -> Self {
        Tag::Array(Box::new(value.into()))
    }
}

impl<'a> From<Vec<&'a str>> for Array {
    fn from(value: Vec<&'a str>) -> Self {
        Array::String(value.into_iter().map(str::to_owned).collect())
    }
}

impl<'a, const SIZE: usize> From<[&'a str; SIZE]> for Array {
    fn from(value: [&'a str; SIZE]) -> Self {
        Array::String(value.into_iter().map(str::to_owned).collect())
    }
}

impl<'a> From<Vec<&'a str>> for Tag {
    fn from(value: Vec<&'a str>) -> Self {
        Tag::Array(Box::new(Array::String(
            value.into_iter().map(str::to_owned).collect(),
        )))
    }
}

impl<'a, const SIZE: usize> From<[&'a str; SIZE]> for Tag {
    fn from(value: [&'a str; SIZE]) -> Self {
        Tag::Array(Box::new(Array::String(
            value.into_iter().map(str::to_owned).collect(),
        )))
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        Tag::String(Box::new(value.to_owned()))
    }
}

macro_rules! from_impls {
    ($({$id:literal $name:ident $impl:ident $box:ident <$type:ty> $($end:tt)*})*) => {
        $(
            from_impls!($box $name $type);
        )*
    };
    (box $name:ident $type:ty) => {
        impl From<Box<$type>> for Tag {
            fn from(value: Box<$type>) -> Tag {
                Tag::$name(value)
            }
        }
        impl From<$type> for Tag {
            fn from(value: $type) -> Tag {
                Tag::$name(Box::new(value))
            }
        }
    };
    (unbox $name:ident $type:ty) => {
        impl From<$type> for Tag {
            fn from(value: $type) -> Tag {
                Tag::$name(value)
            }
        }
    };
}

tag_table!(from_impls);

impl Tag {
    pub const NULL: Tag = Tag::Null;

    pub fn is_null(&self) -> bool {
        matches!(self, Tag::Null)
    }
}

impl<S: AsRef<str>> std::ops::Index<S> for Tag {
    type Output = Tag;
    fn index(&self, index: S) -> &Self::Output {
        const NULL: Tag = Tag::Null;
        let Tag::Map(map) = self else {
            return &NULL;
        };
        map.get(index.as_ref()).unwrap_or(&NULL)
    }
}

impl<S: AsRef<str>> std::ops::IndexMut<S> for Tag {
    fn index_mut(&mut self, index: S) -> &mut Self::Output {
        if let Tag::Map(map) = self {
            map.entry(index.as_ref().to_owned()).or_insert(Tag::Null)
        } else {
            panic!("Not a map");
        }
    }
}