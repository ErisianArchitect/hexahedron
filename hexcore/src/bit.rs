use serde::{Serialize, Deserialize};
use bytemuck::NoUninit;

use std::ops::Range;

pub trait BitSize {
    const BIT_SIZE: u32;
}

pub trait BitLength: BitSize {
    fn bit_length(self) -> u32;
}

pub trait ShiftIndex: Copy {
    /// A `u32` value that represents an index that a `1` bit can be shifted to.
    /// This simply converts the value to u32.
    
    fn shift_index(self) -> u32;
}

pub trait SetBit {
    #[must_use]
    fn set_bit<I: ShiftIndex>(self, index: I, on: bool) -> Self;
    #[must_use]
    fn add_bit<I: ShiftIndex>(self, index: I) -> Self;
    #[must_use]
    fn remove_bit<I: ShiftIndex>(self, index: I) -> Self;
    #[must_use]
    fn set_bitmask(self, mask: Range<u32>, value: Self) -> Self;
    #[must_use]
    fn delete_bitmask(self, mask: Range<u32>) -> Self;
}

pub trait GetBit {
    
    #[must_use]
    fn get_bit<I: ShiftIndex>(self, index: I) -> bool;
    
    #[must_use]
    fn get_bitmask(self, mask: Range<u32>) -> Self;

    #[must_use]
    fn bitmask_range(mask: Range<u32>) -> Self;
}

pub trait InvertBit {
    #[must_use]
    fn invert_bit<I: ShiftIndex>(self, index: I) -> Self;
}

/// To allow polymorphism for iterators of different integer types or references to integer types.
pub trait MoveBitsIteratorItem {
    fn translate(self) -> usize;
}

pub trait MoveBits: Sized {
    fn move_bits<T: MoveBitsIteratorItem, It: IntoIterator<Item = T>>(self, new_indices: It) -> Self;
    /// Much like move_bits, but takes indices in reverse order. This is useful if you want to have the
    /// indices laid out more naturally from right to left.
    fn move_bits_rev<T: MoveBitsIteratorItem, It: IntoIterator<Item = T>>(self, new_indices: It) -> Self
    where It::IntoIter: DoubleEndedIterator {
        self.move_bits(new_indices.into_iter().rev())
    }
}

impl<T: BitSize + GetBit + SetBit + Copy> MoveBits for T {
    fn move_bits<I: MoveBitsIteratorItem, It: IntoIterator<Item = I>>(self, source_indices: It) -> Self {
        source_indices.into_iter()
            .map(I::translate)
            .enumerate()
            .take(Self::BIT_SIZE as usize)
            .fold(self, |value, (index, swap_index)| {
                let on = value.get_bit(swap_index);
                value.set_bit(index, on)
            })
    }
}

macro_rules! __bit_impls {
    ($type:ty) => {
        impl BitSize for $type {
            const BIT_SIZE: u32 = std::mem::size_of::<$type>() as u32 * 8;
        }

        impl BitLength for $type {
            fn bit_length(self) -> u32 {
                Self::BIT_SIZE - self.leading_zeros()
            }
        }

        impl ShiftIndex for $type {
            fn shift_index(self) -> u32 {
                self as u32
            }
        }

        impl InvertBit for $type {
            #[must_use]
            fn invert_bit<I: ShiftIndex>(self, index: I) -> Self {
                let mask = (1 as Self).overflowing_shl(index.shift_index()).0;
                self ^ mask
            }
        }

        impl SetBit for $type {
            #[must_use]
            fn set_bit<I: ShiftIndex>(self, index: I, on: bool) -> Self {
                if let (mask, false) = (1 as $type).overflowing_shl(index.shift_index()) {
                    if on {
                        self | mask
                    } else {
                        self & !mask
                    }
                } else {
                    self
                }
            }

            #[must_use]
            fn add_bit<I: ShiftIndex>(self, index: I) -> Self {
                if let (mask, false) = (1 as $type).overflowing_shl(index.shift_index()) {
                    self | mask
                } else {
                    self
                }
            }

            #[must_use]
            fn remove_bit<I: ShiftIndex>(self, index: I) -> Self {
                if let (mask, false) = (1 as $type).overflowing_shl(index.shift_index()) {
                    self & !mask
                } else {
                    self
                }
            }

            #[must_use]
            fn set_bitmask(self, mask: Range<u32>, value: Self) -> Self {
                let mask_len = mask.len();
                let (bitmask, size_mask) = if mask_len as u32 >= Self::BIT_SIZE {
                    (
                        Self::MAX.overflowing_shl(mask.start).0,
                        Self::MAX
                    )
                } else {
                    (
                        ((1 as Self).overflowing_shl(mask_len as u32).0 - 1).overflowing_shl(mask.start).0,
                        (1 as Self).overflowing_shl(mask_len as u32).0 - 1
                    )
                };
                let delete = self & !bitmask;
                let value = value & size_mask;
                delete | value << mask.start
            }

            #[must_use]
            fn delete_bitmask(self, mask: Range<u32>) -> Self {
                let mask_len = mask.len();
                let bitmask = if mask_len as u32 >= Self::BIT_SIZE {
                    Self::MAX.overflowing_shl(mask.start).0
                } else {
                    ((1 as Self).overflowing_shl(mask_len as u32).0 - 1).overflowing_shl(mask.start).0
                };
                self & !bitmask
            }
        
        }

        impl GetBit for $type {

            #[must_use]
            fn bitmask_range(range: Range<u32>) -> Self {
                let range_len = range.len() as u32;
                if range_len == Self::BIT_SIZE {
                    Self::MAX.overflowing_shl(range.start).0
                } else {
                    ((1 as Self).overflowing_shl(range_len).0 - 1).overflowing_shl(range.start).0
                }
            }

            #[must_use]
            fn get_bit<I: ShiftIndex>(self, index: I) -> bool {
                if let (mask, false) = (1 as $type).overflowing_shl(index.shift_index()) {
                    (self & mask) != 0
                } else {
                    false
                }
            }

            #[must_use]
            fn get_bitmask(self, mask: Range<u32>) -> Self {
                let mask_len = mask.len();
                let bitmask = if mask_len as u32 >= Self::BIT_SIZE {
                    Self::MAX.overflowing_shl(mask.start).0
                } else {
                    ((1 as Self).overflowing_shl(mask_len as u32).0 - 1).overflowing_shl(mask.start).0
                };
                (self & bitmask).overflowing_shr(mask.start as u32).0
            }
        }

        impl MoveBitsIteratorItem for $type {
            fn translate(self) -> usize {
                self as usize
            }
        }

        impl MoveBitsIteratorItem for &$type {
            fn translate(self) -> usize {
                *self as usize
            }
        }
    };
}

crate::macros::for_each_int_type!(__bit_impls);

mod private {
    pub trait BitFlagsSealed: Sized + Default + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash {}
}

pub trait BitFlags: private::BitFlagsSealed {
    fn get(self, index: u32) -> bool;
    fn set(&mut self, index: u32, value: bool) -> bool;
    fn iter(self) -> impl Iterator<Item = bool>;
    const BIT_SIZE: u32;
}

macro_rules! bitflags_impls {
    ($type:ident($inner_type:ty)) => {
        #[repr(C)]
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, NoUninit, Serialize, Deserialize)]
        pub struct $type(pub $inner_type);

        impl private::BitFlagsSealed for $type {}

        impl std::ops::BitOr<$type> for $type {
            type Output = Self;
            
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }
        
        impl std::ops::BitOr<$inner_type> for $type {
            type Output = Self;
            
            fn bitor(self, rhs: $inner_type) -> Self::Output {
                Self(self.0 | rhs)
            }
        }
        
        impl std::ops::BitAnd<$type> for $type {
            type Output = Self;
            
            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }
        
        impl std::ops::BitAnd<$inner_type> for $type {
            type Output = Self;
            
            fn bitand(self, rhs: $inner_type) -> Self::Output {
                Self(self.0 & rhs)
            }
        }

        impl std::ops::BitXor<$type> for $type {
            type Output = Self;

            fn bitxor(self, rhs: $type) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl std::ops::BitXor<$inner_type> for $type {
            type Output = Self;

            fn bitxor(self, rhs: $inner_type) -> Self::Output {
                Self(self.0 ^ rhs)
            }
        }

        impl std::ops::Not for $type {
            type Output = Self;
            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }
        
        impl std::ops::Sub<$type> for $type {
            type Output = Self;
            
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 & !rhs.0)
            }
        }
        
        impl std::ops::Sub<$inner_type> for $type {
            type Output = Self;
            
            fn sub(self, rhs: $inner_type) -> Self::Output {
                Self(self.0 & !rhs)
            }
        }

        impl std::ops::Index<u32> for $type {
            type Output = bool;
            
            fn index(&self, index: u32) -> &Self::Output {
                const FALSE_TRUE: [bool; 2] = [false, true];
                let index = ((self.0 & (1 << index)) != 0) as usize;
                &FALSE_TRUE[index]
            }
        }

        impl std::ops::Deref for $type {
            type Target = $inner_type;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl BitFlags for $type {
            const BIT_SIZE: u32 = (std::mem::size_of::<Self>() * 8) as u32;
            
            fn get(self, index: u32) -> bool {
                (self.0 & (1 << index)) != 0
            }
        
            
            fn set(&mut self, index: u32, value: bool) -> bool {
                let old = (self.0 & (1 << index)) != 0;
                if value {
                    self.0 = self.0 | (1 << index);
                } else {
                    self.0 = self.0 & !(1 << index);
                }
                old
            }
            
            
            fn iter(self) -> impl Iterator<Item = bool> {
                (0..Self::BIT_SIZE).map(move |i| self.get(i))
            }
        }
    };
    ($($type:ident($inner_type:ty);)*) => {
        $(
            bitflags_impls!{$type($inner_type)}
        )*
    };
}

bitflags_impls!(
    BitFlags8(u8);
    BitFlags16(u16);
    BitFlags32(u32);
    BitFlags64(u64);
    BitFlags128(u128);
);

impl std::fmt::Display for BitFlags8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BitFlags8({:08b})", self.0)
    }
}

impl std::fmt::Display for BitFlags16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let low = self.0 & 0xFF;
        let high = self.0 >> 8;
        write!(f, "BitFlags16({high:08b} {low:08b})")
    }
}

impl std::fmt::Display for BitFlags32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _0 = self.0 & 0xFF;
        let _1 = self.0 >> 8 & 0xFF;
        let _2 = self.0 >> 16 & 0xFF;
        let _3 = self.0 >> 24 & 0xFF;
        write!(f, "BitFlags32({_3:08b} {_2:08b} {_1:08b} {_0:08b})")
    }
}

impl std::fmt::Display for BitFlags64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _0 = self.0 & 0xFF;
        let _1 = self.0 >> 8 & 0xFF;
        let _2 = self.0 >> 16 & 0xFF;
        let _3 = self.0 >> 24 & 0xFF;
        let _4 = self.0 >> 32 & 0xFF;
        let _5 = self.0 >> 40 & 0xFF;
        let _6 = self.0 >> 48 & 0xFF;
        let _7 = self.0 >> 56 & 0xFF;
        write!(f, "BitFlags64({_7:08b} {_6:08b} {_5:08b} {_4:08b} {_3:08b} {_2:08b} {_1:08b} {_0:08b})")
    }
}

impl std::fmt::Display for BitFlags128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _0  = self.0 >> ( 0 * 8) & 0xFF;
        let _1  = self.0 >> ( 1 * 8) & 0xFF;
        let _2  = self.0 >> ( 2 * 8) & 0xFF;
        let _3  = self.0 >> ( 3 * 8) & 0xFF;

        let _4  = self.0 >> ( 4 * 8) & 0xFF;
        let _5  = self.0 >> ( 5 * 8) & 0xFF;
        let _6  = self.0 >> ( 6 * 8) & 0xFF;
        let _7  = self.0 >> ( 7 * 8) & 0xFF;

        let _8  = self.0 >> ( 8 * 8) & 0xFF;
        let _9  = self.0 >> ( 9 * 8) & 0xFF;
        let _10 = self.0 >> (10 * 8) & 0xFF;
        let _11 = self.0 >> (11 * 8) & 0xFF;

        let _12 = self.0 >> (12 * 8) & 0xFF;
        let _13 = self.0 >> (13 * 8) & 0xFF;
        let _14 = self.0 >> (14 * 8) & 0xFF;
        let _15 = self.0 >> (15 * 8) & 0xFF;

        write!(f, "BitFlags128({_15:08b} {_14:08b} {_13:08b} {_12:08b} {_11:08b} {_10:08b} {_9:08b} {_8:08b} {_7:08b} {_6:08b} {_5:08b} {_4:08b} {_3:08b} {_2:08b} {_1:08b} {_0:08b})")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn delete_bitmask_test() {
        debug_assert_eq!(0b11000011, u8::MAX.delete_bitmask(2..6));
    }
}