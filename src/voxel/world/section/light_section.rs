// use bytemuck::NoUninit;

use crate::{math::index3, prelude::OptionExtension, util::change::Change};


/// Returns (low, high) where low is bits 0..=3 and high is bits 4..=7.
#[inline]
const fn get_nibble(nibble: u8) -> (u8, u8) {
    let low = nibble & 0xf;
    let high = nibble >> 4;
    (low, high)
}

/// Sets (low, high) where low is bits 0..=3 and high is bits 4..=7.
#[inline]
const fn set_nibble(low: u8, high: u8) -> u8 {
    low & 0xf | (high << 4)
}

#[derive(Debug, Default, Clone)]
pub struct LightSection<const W: i32, const DEFAULT: u8> {
    light_data: Option<Box<[u8]>>,
    instance_count: u16,
}

impl<const W: i32, const DEFAULT: u8> LightSection<W, DEFAULT> {
    /// This is the number of bytes that are used to get two 4-bit nibbles per byte..
    const NIBBLE_COUNT: usize = (W as usize).pow(3) / 2;
    const DEFAULT_NIBBLE: u8 = set_nibble(DEFAULT, DEFAULT);
    pub const fn new() -> Self {
        Self {
            light_data: None,
            instance_count: 0,
        }
    }

    pub fn get<C: Into<(i32, i32, i32)>>(&self, coord: C) -> u8 {
        let (x, y, z): (i32, i32, i32) = coord.into();
        self.light_data.as_ref().and_then(|data| {
            let index = index3::<W, W, W>(x, y, z);
            let subindex = index / 2;
            let lights = data[subindex];
            if (index & 1) == 1 {
                Some(lights >> 4)
            } else {
                Some(lights & 0xf)
            }
        }).unwrap_or(DEFAULT)
    }

    pub fn set<C: Into<(i32, i32, i32)>>(&mut self, coord: C, level: u8) -> Change<u8> {
        let (x, y, z): (i32, i32, i32) = coord.into();
        if self.light_data.is_none() && level == DEFAULT {
            return Change::Unchanged;
        }
        let data = self.light_data.get_or_insert_with(|| (0..Self::NIBBLE_COUNT).map(|_| Self::DEFAULT_NIBBLE).collect());
        let index = index3::<W, W, W>(x, y, z);
        let subindex = index / 2;
        let lights = data[subindex];
        let (low, high) = get_nibble(lights);
        let (old, injected) = if (index & 1) == 1 {
            (
                high,
                set_nibble(low, level),
            )
        } else {
            (
                low,
                set_nibble(level, high),
            )
        };
        data[subindex] = injected;
        if level != old {
            if level == DEFAULT {
                self.instance_count -= 1;
                if self.instance_count == 0 {
                    self.light_data.drop();
                }
            } else {
                self.instance_count += 1;
            }
        }
        Change::cmp_new(&level, old)
    }

    pub fn clear(&mut self) {
        self.light_data.drop();
        self.instance_count = 0;
    }

    #[inline]
    pub fn is_allocated(&self) -> bool {
        self.light_data.is_some()
    }
}

// #[repr(C)]
// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, NoUninit)]
// pub struct Nibble2(pub u8);

// impl Nibble2 {
//     #[inline]
//     pub const fn get_0(self) -> u8 {
//         self.0 & 0xf
//     }

//     #[inline]
//     pub const fn get_1(self) -> u8 {
//         self.0 >> 4
//     }

//     #[inline]
//     pub fn set_0(&mut self, value: u8) -> u8 {
//         let value = value & 0xf;
//         let old = self.get_0();
//         self.0 = (self.0 & 0xf0) | value;
//         old
//     }

//     #[inline]
//     pub fn set_1(&mut self, value: u8) -> u8 {
//         let value = value & 0xf;
//         let old = self.get_1();
//         self.0 = (self.0 & 0x0f) | (value << 4);
//         old
//     }

//     #[inline]
//     pub const fn get(self, index: usize) -> u8 {
//         match index {
//             0 => self.get_0(),
//             1 => self.get_1(),
//             _ => panic!("index out of range. (expected value in range of 0 <= index <= 1)"),
//         }
//     }

//     #[inline]
//     pub fn set(&mut self, index: usize, value: u8) -> u8 {
//         match index {
//             0 => self.set_0(value),
//             1 => self.set_1(value),
//             _ => panic!("index out of range. (expected value in range of 0 <= index <= 1)"),
//         }
//     }
// }