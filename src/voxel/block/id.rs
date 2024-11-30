use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BlockId(pub(super) u32);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StateId(pub(super) u32);

impl BlockId {
    pub const AIR: Self = Self(0);
    
    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn is_air(self) -> bool {
        self.0 == 0
    }
}

impl StateId {
    pub const AIR: Self = Self(0);

    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn is_air(self) -> bool {
        self.0 == 0
    }
}