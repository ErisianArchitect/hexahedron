

// I don't think this function is needed right now, but I'm keeping it as a reference to how the block size is calculated.
// pub const fn block_size_notation(block_count: u64, exponent: u32, bit_size: u32) -> u64 {
//     let max_block_size = 2u64.pow(bit_size)-1;
//     let spacer1 = (2u64.pow(exponent) - 1) * max_block_size;
//     let spacer2 = if exponent > 0 {
//         2u64.pow(exponent)
//     } else {
//         0
//     };
//     block_count * 2u64.pow(exponent) + spacer1 + spacer2
// }

/// 4KiB block size notation. This uses some special math to extend the size of a byte.
/// This allows you to use a byte to represent a higher range of values at the cost of not being able to represent some values.
/// This is used for block counts in region files.
/// This allows for small chunks to be stored in 4KiB sections while larger chunks might take up more space.
/// This allows for a maximum size of around 32MiB per chunk.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockSize(pub(super) u8);

impl BlockSize {
    pub const MAX_BLOCK_COUNT: u16 = 8033;
    const BLOCK_SIZE_TABLE: [u16; 256] = [
        // Column: Multiplier
        //    Row: 2.pow(Exponent)
        /*        0    1    2    3    4    5    6    7    8    9   10   11   12   13   14   15   16   17   18   19   20   21   22   23   24   25   26   27   28   29   30   31  */
        /* 0 */ 0000,0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026,0027,0028,0029,0030,0031,
        /* 1 */ 0033,0035,0037,0039,0041,0043,0045,0047,0049,0051,0053,0055,0057,0059,0061,0063,0065,0067,0069,0071,0073,0075,0077,0079,0081,0083,0085,0087,0089,0091,0093,0095,
        /* 2 */ 0097,0101,0105,0109,0113,0117,0121,0125,0129,0133,0137,0141,0145,0149,0153,0157,0161,0165,0169,0173,0177,0181,0185,0189,0193,0197,0201,0205,0209,0213,0217,0221,
        /* 3 */ 0225,0233,0241,0249,0257,0265,0273,0281,0289,0297,0305,0313,0321,0329,0337,0345,0353,0361,0369,0377,0385,0393,0401,0409,0417,0425,0433,0441,0449,0457,0465,0473,
        /* 4 */ 0481,0497,0513,0529,0545,0561,0577,0593,0609,0625,0641,0657,0673,0689,0705,0721,0737,0753,0769,0785,0801,0817,0833,0849,0865,0881,0897,0913,0929,0945,0961,0977,
        /* 5 */ 0993,1025,1057,1089,1121,1153,1185,1217,1249,1281,1313,1345,1377,1409,1441,1473,1505,1537,1569,1601,1633,1665,1697,1729,1761,1793,1825,1857,1889,1921,1953,1985,
        /* 6 */ 2017,2081,2145,2209,2273,2337,2401,2465,2529,2593,2657,2721,2785,2849,2913,2977,3041,3105,3169,3233,3297,3361,3425,3489,3553,3617,3681,3745,3809,3873,3937,4001,
        /* 7 */ 4065,4193,4321,4449,4577,4705,4833,4961,5089,5217,5345,5473,5601,5729,5857,5985,6113,6241,6369,6497,6625,6753,6881,7009,7137,7265,7393,7521,7649,7777,7905,8033,
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

impl std::fmt::Display for BlockSize {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockSize({})", self.block_count())
    }
}