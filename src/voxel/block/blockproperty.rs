use crate::{io::*, prelude::FaceFlags};

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
            [12     FaceFlags(crate::voxel::faceflags::FaceFlags)       ]
            [13     AxisFlags(crate::math::axisflags::AxisFlags)        ]
            [14     BitFlags8(crate::math::bit::BitFlags8)              ]
            [15     BitFlags16(crate::math::bit::BitFlags16)            ]
            [16     BitFlags32(crate::math::bit::BitFlags32)            ]
            [17     BitFlags64(crate::math::bit::BitFlags64)            ]
            [18     BitFlags128(crate::math::bit::BitFlags128)          ]
            [19     Rgb(crate::rendering::color::Rgb)                   ]
            [20     Rgba(crate::rendering::color::Rgba)                 ]
            [21     Range(std::ops::Range<i64>)                         ]
            [22     RangeInclusive(std::ops::RangeInclusive<i64>)       ]
        }
    };
}

macro_rules! build_property_enum {
    ($([$id:literal $name:ident($type:ty)])+) => {
        #[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum Property {
            #[default]
            Null = 0,
            $(
                $name($type) = $id,
            )+
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
                Ok(match id {
                    0 => Property::Null,
                    $(
                        $id => Property::$name(<$type>::read_from(reader)?),
                    )*
                    id => return Err(super::error::Error::InvalidPropertyId(id).into()),
                })
            }
        }

        impl Writeable for Property {
            fn write_to<W: std::io::Write>(&self, writer: &mut W) -> crate::prelude::VoxelResult<u64> {
                Ok(1 + match self {
                    Property::Null => {
                        0u8.write_to(writer)?;
                        0
                    },
                    $(
                        Property::$name(inner) => {
                            ($id as u8).write_to(writer)?;
                            inner.write_to(writer)?
                        },
                    )*
                })
            }
        }
    };
}

property_table!(build_property_enum);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockProperty {
    name: String,
    value: Property
}

impl PartialOrd for BlockProperty {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for BlockProperty {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}