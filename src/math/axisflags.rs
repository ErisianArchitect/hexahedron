use bytemuck::NoUninit;

use super::axis::Axis;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, NoUninit)]
#[repr(C)]
pub struct AxisFlags(u8);

impl AxisFlags {
    pub const X: AxisFlags = AxisFlags(0b001);
    pub const Y: AxisFlags = AxisFlags(0b010);
    pub const Z: AxisFlags = AxisFlags(0b100);

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
}

impl std::ops::BitOr<AxisFlags> for AxisFlags {
    type Output = AxisFlags;
    #[inline]
    fn bitor(self, rhs: AxisFlags) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign<AxisFlags> for AxisFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: AxisFlags) {
        self.0 = self.0 | rhs.0;
    }
}

impl std::ops::BitAnd<AxisFlags> for AxisFlags {
    type Output = AxisFlags;
    #[inline]
    fn bitand(self, rhs: AxisFlags) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign<AxisFlags> for AxisFlags {
    #[inline]
    fn bitand_assign(&mut self, rhs: AxisFlags) {
        self.0 = self.0 & rhs.0;
    }
}

impl std::ops::BitXor<AxisFlags> for AxisFlags {
    type Output = AxisFlags;
    #[inline]
    fn bitxor(self, rhs: AxisFlags) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXorAssign<AxisFlags> for AxisFlags {
    #[inline]
    fn bitxor_assign(&mut self, rhs: AxisFlags) {
        self.0 = self.0 ^ rhs.0;
    }
}

impl std::ops::Sub<AxisFlags> for AxisFlags {
    type Output = AxisFlags;
    #[inline]
    fn sub(self, rhs: AxisFlags) -> Self::Output {
        Self(self.0 & !rhs.0)
    }
}

impl std::ops::SubAssign<AxisFlags> for AxisFlags {
    #[inline]
    fn sub_assign(&mut self, rhs: AxisFlags) {
        self.0 = self.0 & !rhs.0;
    }
}