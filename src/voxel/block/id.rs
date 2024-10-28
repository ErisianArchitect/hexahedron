#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(pub(in super) u32);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateId(pub(in super) u32);

impl BlockId {
    pub const AIR: Self = Self(0);
    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl StateId {
    pub const AIR: Self = Self(0);

    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}