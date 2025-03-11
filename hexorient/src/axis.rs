use serde::{Serialize, Deserialize};
use bytemuck::NoUninit;

use crate::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, NoUninit, Serialize, Deserialize)]
#[repr(u8)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2
}

impl Axis {
    #[inline]
    pub const fn pos(self) -> Direction {
        match self {
            Axis::X => Direction::PosX,
            Axis::Y => Direction::PosY,
            Axis::Z => Direction::PosZ,
        }
    }

    #[inline]
    pub const fn neg(self) -> Direction {
        match self {
            Axis::X => Direction::NegX,
            Axis::Y => Direction::NegY,
            Axis::Z => Direction::NegZ,
        }
    }
}