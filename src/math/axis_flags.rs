use serde::{Serialize, Deserialize};
use bytemuck::NoUninit;

use super::axis::Axis;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, NoUninit, Serialize, Deserialize)]
#[repr(C)]
pub struct AxisFlags(u8);

impl AxisFlags {
    pub const NONE: AxisFlags = AxisFlags(0);
    pub const X: AxisFlags = AxisFlags(0b001);
    pub const Y: AxisFlags = AxisFlags(0b010);
    pub const Z: AxisFlags = AxisFlags(0b100);
    pub const XY: AxisFlags = AxisFlags(0b011);
    pub const XZ: AxisFlags = AxisFlags(0b101);
    pub const YZ: AxisFlags = AxisFlags(0b110);
    pub const XYZ: AxisFlags = AxisFlags(0b111);

    #[inline]
    pub(crate) const fn from_u8(value: u8) -> Self {
        Self(value)
    }

    #[inline]
    pub(crate) const fn inner(self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn get_axis(self, axis: Axis) -> bool {
        match axis {
            Axis::X => (self.0 & AxisFlags::X.0) != 0,
            Axis::Y => (self.0 & AxisFlags::Y.0) != 0,
            Axis::Z => (self.0 & AxisFlags::Z.0) != 0,
        }
    }

    #[inline]
    pub fn set_axis(&mut self, axis: Axis, value: bool) {
        match axis {
            Axis::X if value => self.0 |= 0b001,
            Axis::X => self.0 &= 0b110,
            Axis::Y if value => self.0 |= 0b010,
            Axis::Y => self.0 &= 0b101,
            Axis::Z if value => self.0 |= 0b100,
            Axis::Z => self.0 &= 0b011,
        }
    }

    #[inline]
    pub fn tuple(self) -> (bool, bool, bool) {
        (
            (self.0 & AxisFlags::X.0) != 0,
            (self.0 & AxisFlags::Y.0) != 0,
            (self.0 & AxisFlags::Z.0) != 0,
        )
    }

    #[inline]
    pub fn array(self) -> [bool; 3] {
        [
            (self.0 & AxisFlags::X.0) != 0,
            (self.0 & AxisFlags::Y.0) != 0,
            (self.0 & AxisFlags::Z.0) != 0,
        ]
    }
}

impl Into<(bool, bool, bool)> for AxisFlags {
    #[inline]
    fn into(self) -> (bool, bool, bool) {
        self.tuple()
    }
}

impl From<(bool, bool, bool)> for AxisFlags {
    #[inline]
    fn from(value: (bool, bool, bool)) -> Self {
        AxisFlags(value.0 as u8 | (value.1 as u8) << 1 | (value.2 as u8) << 2)
    }
}

impl Into<[bool; 3]> for AxisFlags {
    #[inline]
    fn into(self) -> [bool; 3] {
        self.array()
    }
}

impl From<[bool; 3]> for AxisFlags {
    #[inline]
    fn from(value: [bool; 3]) -> Self {
        let [x, y, z] = value;
        AxisFlags(x as u8 | (y as u8) << 1 | (z as u8) << 2)
    }
}

impl From<Axis> for AxisFlags {
    #[inline]
    fn from(value: Axis) -> Self {
        match value {
            Axis::X => AxisFlags::X,
            Axis::Y => AxisFlags::Y,
            Axis::Z => AxisFlags::Z,
        }
    }
}

impl std::ops::BitOr<AxisFlags> for AxisFlags {
    type Output = AxisFlags;
    
    #[inline]
    fn bitor(self, rhs: AxisFlags) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOr<Axis> for AxisFlags {
    type Output = AxisFlags;

    #[inline]
    fn bitor(self, rhs: Axis) -> Self::Output {
        self | AxisFlags::from(rhs)
    }
}

impl std::ops::BitOrAssign<AxisFlags> for AxisFlags {

    #[inline]
    fn bitor_assign(&mut self, rhs: AxisFlags) {
        self.0 = self.0 | rhs.0;
    }
}

impl std::ops::BitOrAssign<Axis> for AxisFlags {

    #[inline]
    fn bitor_assign(&mut self, rhs: Axis) {
        *self |= AxisFlags::from(rhs);
    }
}

impl std::ops::BitAnd<AxisFlags> for AxisFlags {
    type Output = AxisFlags;

    #[inline]
    fn bitand(self, rhs: AxisFlags) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAnd<Axis> for AxisFlags {
    type Output = AxisFlags;
    
    #[inline]
    fn bitand(self, rhs: Axis) -> Self::Output {
        self & AxisFlags::from(rhs)
    }
}

impl std::ops::BitAndAssign<AxisFlags> for AxisFlags {
    
    #[inline]
    fn bitand_assign(&mut self, rhs: AxisFlags) {
        self.0 = self.0 & rhs.0;
    }
}

impl std::ops::BitAndAssign<Axis> for AxisFlags {

    #[inline]
    fn bitand_assign(&mut self, rhs: Axis) {
        *self &= AxisFlags::from(rhs);
    }
}

impl std::ops::BitXor<AxisFlags> for AxisFlags {
    type Output = AxisFlags;

    #[inline]
    fn bitxor(self, rhs: AxisFlags) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXor<Axis> for AxisFlags {
    type Output = AxisFlags;
    
    #[inline]
    fn bitxor(self, rhs: Axis) -> Self::Output {
        self ^ AxisFlags::from(rhs)
    }
}

impl std::ops::BitXorAssign<AxisFlags> for AxisFlags {
    
    #[inline]
    fn bitxor_assign(&mut self, rhs: AxisFlags) {
        self.0 = self.0 ^ rhs.0;
    }
}

impl std::ops::BitXorAssign<Axis> for AxisFlags {

    #[inline]
    fn bitxor_assign(&mut self, rhs: Axis) {
        *self ^= AxisFlags::from(rhs);
    }
}

impl std::ops::Sub<AxisFlags> for AxisFlags {
    type Output = AxisFlags;
    
    #[inline]
    fn sub(self, rhs: AxisFlags) -> Self::Output {
        Self(self.0 & !rhs.0)
    }
}

impl std::ops::Sub<Axis> for AxisFlags {
    type Output = AxisFlags;

    #[inline]
    fn sub(self, rhs: Axis) -> Self::Output {
        self - AxisFlags::from(rhs)
    }
}

impl std::ops::SubAssign<AxisFlags> for AxisFlags {
    
    #[inline]
    fn sub_assign(&mut self, rhs: AxisFlags) {
        self.0 = self.0 & !rhs.0;
    }
}

impl std::ops::SubAssign<Axis> for AxisFlags {
    
    #[inline]
    fn sub_assign(&mut self, rhs: Axis) {
        *self -= AxisFlags::from(rhs);
    }
}