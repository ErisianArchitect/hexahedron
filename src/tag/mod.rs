pub trait NonByte {}
pub trait Byte {}

#[rustfmt::skip]
macro_rules! tag_table {
    ($macro:path) => {
        $macro! {
            //  
            [ 1 Bool          Byte      unbox   <bool>                                                  ]
            [ 2 BitFlags8     Byte      unbox   <crate::math::bit::BitFlags8>                           ]
            [ 3 BitFlags16    NonByte   unbox   <crate::math::bit::BitFlags16>                          ]
            [ 4 BitFlags32    NonByte   unbox   <crate::math::bit::BitFlags32>                          ]
            [ 5 BitFlags64    NonByte   unbox   <crate::math::bit::BitFlags64>                          ]
            [ 6 BitFlags128   NonByte   box     <crate::math::bit::BitFlags128>                         ]
            [ 7 U8            Byte      unbox   <u8>                                                    ]
            [ 8 I8            Byte      unbox   <i8>                                                    ]
            [ 9 U16           NonByte   unbox   <u16>                                                   ]
            [10 I16           NonByte   unbox   <i16>                                                   ]
            [11 U32           NonByte   unbox   <u32>                                                   ]
            [12 I32           NonByte   unbox   <i32>                                                   ]
            [13 U64           NonByte   unbox   <u64>                                                   ]
            [14 I64           NonByte   unbox   <i64>                                                   ]
            [15 F32           NonByte   unbox   <f32>                                                   ]
            [16 F64           NonByte   unbox   <f64>                                                   ]
            [17 Direction     Byte      unbox   <crate::voxel::direction::Direction>                    ]
            [18 Cardinal      Byte      unbox   <crate::voxel::direction::Cardinal>                     ]
            [19 Rotation      Byte      unbox   <crate::voxel::orientation::rotation::Rotation>         ]
            [20 Flip          Byte      unbox   <crate::voxel::orientation::flip::Flip>                 ]
            [21 Orientation   Byte      unbox   <crate::voxel::orientation::orientation::Orientation>   ]
            [22 Axis          Byte      unbox   <crate::math::axis::Axis>                               ]
            [23 Rgb           NonByte   unbox   <crate::rendering::color::Rgb>                          ]
            [24 Rgba          NonByte   unbox   <crate::rendering::color::Rgba>                         ]
            [25 IVec2         NonByte   unbox   <glam::IVec2>                                           ]
            [26 IVec3         NonByte   unbox   <glam::IVec3>                                           ]
            [27 IVec4         NonByte   box     <glam::IVec4>                                           ]
            [28 Vec2          NonByte   unbox   <glam::Vec2>                                            ]
            [29 Vec3          NonByte   unbox   <glam::Vec3>                                            ]
            [30 Vec4          NonByte   box     <glam::Vec4>                                            ]
            [31 Mat2          NonByte   box     <glam::Mat2>                                            ]
            [32 Mat3          NonByte   box     <glam::Mat3>                                            ]
            [33 Mat4          NonByte   box     <glam::Mat4>                                            ]
            [34 Quat          NonByte   box     <glam::Quat>                                            ]
            [35 Bounds2       NonByte   box     <rollgrid::rollgrid2d::Bounds2D>                        ]
            [36 Bounds3       NonByte   box     <rollgrid::rollgrid3d::Bounds3D>                        ]
            [37 String        NonByte   box     <String>                                                ]
            [38 Array         NonByte   box     <crate::tag::Array>                                     ]
            [39 Map           NonByte   box     <hashbrown::HashMap<String, Tag>>                       ]
            /* This line should remain commented out. It is a representation of what I wrote manually
            [63 Tag           NonByte   box     <Tag>                                                   ]
            Continue writing new rows at index 39
            */
        }
    };
}