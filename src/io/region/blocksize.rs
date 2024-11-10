

// I don't think this function is needed right now, but I'm keeping it as a reference to how the block size is calculated.
// pub const fn block_size_notation(block_count: u64, exponent: u32, bit_size: u32) -> u64 {
//     let max_block_size = 2u64.pow(bit_size)-1;
//     let spacer1 = (2u64.pow(exponent) - 1) * max_block_size;
//     let spacer2 = if exponent > 0 {
//         2u64.pow(exponent)
//     } else {
//         0
//     };
//     block_count * 2u64.pow(exponent) + spacer1 + spacer2 + 1
// }

/// 4KiB block size notation. This uses some special math to extend the size of a byte.
/// This allows you to use a byte to represent a higher range of values at the cost of not being able to represent some values.
/// This is used for block counts in region files.
/// This allows for small chunks to be stored in 4KiB sections while larger chunks might take up more space.
/// This allows for a maximum size of around 32MiB per chunk.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockSize(pub(super) u8);

impl BlockSize {
    pub const MAX_BLOCK_COUNT: u16 = 8034;
    // It's impossible to represent a size of 0 with BlockSize.
    pub const BLOCK_SIZE_TABLE: [u16; 256] = [
        // Column: Multiplier
        // Row: 2.pow(Exponent)
        //        0    1    2    3    4    5    6    7    8    9   10   11   12   13   14   15   16   17   18   19   20   21   22   23   24   25   26   27   28   29   30   31
        /* 0 */ 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026,0027,0028,0029,0030,0031,0032,
        /* 1 */ 0034,0036,0038,0040,0042,0044,0046,0048,0050,0052,0054,0056,0058,0060,0062,0064,0066,0068,0070,0072,0074,0076,0078,0080,0082,0084,0086,0088,0090,0092,0094,0096,
        /* 2 */ 0098,0102,0106,0110,0114,0118,0122,0126,0130,0134,0138,0142,0146,0150,0154,0158,0162,0166,0170,0174,0178,0182,0186,0190,0194,0198,0202,0206,0210,0214,0218,0222,
        /* 3 */ 0226,0234,0242,0250,0258,0266,0274,0282,0290,0298,0306,0314,0322,0330,0338,0346,0354,0362,0370,0378,0386,0394,0402,0410,0418,0426,0434,0442,0450,0458,0466,0474,
        /* 4 */ 0482,0498,0514,0530,0546,0562,0578,0594,0610,0626,0642,0658,0674,0690,0706,0722,0738,0754,0770,0786,0802,0818,0834,0850,0866,0882,0898,0914,0930,0946,0962,0978,
        /* 5 */ 0994,1026,1058,1090,1122,1154,1186,1218,1250,1282,1314,1346,1378,1410,1442,1474,1506,1538,1570,1602,1634,1666,1698,1730,1762,1794,1826,1858,1890,1922,1954,1986,
        /* 6 */ 2018,2082,2146,2210,2274,2338,2402,2466,2530,2594,2658,2722,2786,2850,2914,2978,3042,3106,3170,3234,3298,3362,3426,3490,3554,3618,3682,3746,3810,3874,3938,4002,
        /* 7 */ 4066,4194,4322,4450,4578,4706,4834,4962,5090,5218,5346,5474,5602,5730,5858,5986,6114,6242,6370,6498,6626,6754,6882,7010,7138,7266,7394,7522,7650,7778,7906,8034,
    ];
    
    #[inline]
    pub const fn new(multiplier: u8, exponent: u8) -> Self {
        assert!(multiplier <= 0b11111, "Multiplier greater than 31");
        assert!(exponent <= 0b111, "Exponent greater than 7");
        Self(multiplier | exponent << 5)
    }

    /// This isn't really a "multiplier", but instead 
    #[inline]
    pub const fn multiplier(self) -> u8 {
        self.0 & 0b11111
    }

    /// Exponent of 2 which is multiplied by the multiplier.
    #[inline]
    pub const fn exponent(self) -> u8 {
        self.0 >> 5
    }

    /// The 4KiB block count. (multiply this by 4096 to get the size)
    #[inline]
    pub const fn block_count(self) -> u16 {
        Self::BLOCK_SIZE_TABLE[self.0 as usize]
    }

    /// The block size in bytes.
    #[inline]
    pub const fn file_size(self) -> u64 {
        self.block_count() as u64 * 4096
    }

    /// If the `size` represents an exact block size, then it will return that block size. Otherwise returns `None`.
    pub fn reverse(size: u16) -> Option<Self> {
        assert!(size != 0, "Size is 0.");
        assert!(size <= Self::MAX_BLOCK_COUNT, "Size greater than {}", Self::MAX_BLOCK_COUNT);
        let mut low = 0;
        let mut hi = 256;
        while low < hi {
            let mid = low + (hi - low) / 2;
            let bs = BlockSize::BLOCK_SIZE_TABLE[mid];
            match bs.cmp(&size) {
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Equal => return Some(BlockSize(mid as u8)),
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        None
    }

    /// Gets the [BlockSize] required to contain `size` in bytes.
    pub fn required(size: u16) -> Self {
        assert!(size <= Self::MAX_BLOCK_COUNT, "Size greater than {}", Self::MAX_BLOCK_COUNT);
        let mut low = 0;
        let mut hi = 256;
        while low < hi {
            let mid = low + (hi - low) / 2;
            let bs = BlockSize::BLOCK_SIZE_TABLE[mid];
            match bs.cmp(&size) {
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Equal => return BlockSize(mid as u8),
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        BlockSize(low as u8)
    }
}

pub const fn block_size_notation<const BIT_SIZE: u32>(block_count: u64, exponent: u32) -> u64 {
    let max_block_size = const { 2u64.pow(BIT_SIZE)-1 };
    let spacer1 = (2u64.pow(exponent) - 1) * max_block_size;
    let spacer2 = if exponent > 0 {
        2u64.pow(exponent)
    } else {
        0
    };
    block_count * 2u64.pow(exponent) + spacer1 + spacer2 + 1
}

impl std::fmt::Display for BlockSize {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockSize({})", self.block_count())
    }
}