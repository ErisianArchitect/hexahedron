pub mod light_section;
pub mod block_section;
pub mod tag_section;
pub mod occlusion;
pub mod occlusion_section;
pub mod update_section;
mod section;

use glam::IVec3;
pub use section::*;

use crate::{for_each_int_type, math::index3};

pub trait SectionIndex<const W: i32>: Copy {
    fn section_index(self) -> usize;
}

impl<const W: i32> SectionIndex<W> for IVec3 {
    fn section_index(self) -> usize {
        let IVec3 { x, y, z } = self;
        index3::<W, W, W>(x, y, z)
    }
}

impl<const W: i32> SectionIndex<W> for [i32; 3] {
    fn section_index(self) -> usize {
        let [x, y, z] = self;
        index3::<W, W, W>(x, y, z)
    }
}

impl<const W: i32> SectionIndex<W> for (i32, i32, i32) {
    fn section_index(self) -> usize {
        let (x, y, z) = self;
        index3::<W, W, W>(x, y, z)
    }
}

macro_rules! section_index_impls {
    ($type:ty) => {
        impl<const W: i32> SectionIndex<W> for $type {
            fn section_index(self) -> usize {
                self as usize
            }
        }
    };
}

impl<const W: i32> SectionIndex<W> for usize {
    fn section_index(self) -> usize {
        self
    }
}

// for_each_int_type!(unsigned; section_index_impls);