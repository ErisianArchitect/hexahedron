#![allow(unused)]
use std::num::{NonZeroU32, NonZeroUsize};

use hexahedron::util::extensions::*;
use hexahedron::io::region::block_size::*;


struct Seven0(([u8; 7]));
struct Seven1([u8; 7]);
struct Seven2([u8; 7]);

enum Seven {
    Seven0(Seven0),
    Seven1(Seven1),
    Seven2(Seven2),
}

fn main() {
    println!("{}", 256i32.rem_euclid(256));
}

#[allow(unused)]
fn gen_bsn_table() -> std::fmt::Result {
    use std::fmt::Write;
    let mut table = String::new();
    writeln!(table, "// Column: Multiplier")?;
    writeln!(table, "// Row: 2.pow(Exponent)")?;
    write!(table, "//     ")?;
    for mult in 32.iter() {
        write!(table, "  {mult:2} ")?;
    }
    writeln!(table)?;
    for exp in 8.iter() {
        write!(table, "/* {exp} */ ")?;
        for mult in 32.iter() {
            let size = block_size_notation::<5>(mult, exp);
            write!(table, "{size:04},")?;
        }
        writeln!(table)?;
    }
    println!("{table}");
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NullableIndex {
    Null,
    NonNull(NonZeroU32),
}

impl NullableIndex {
    pub const fn new(value: u32) -> Self {
        match value {
            0 => Self::Null,
            value => Self::NonNull(unsafe { NonZeroU32::new_unchecked(value) })
        }
    }

    pub const fn value(self) -> u32 {
        match self {
            Self::Null => 0,
            Self::NonNull(value) => value.get(),
        }
    }
}

struct AlignedToU128 {
    big: u128,
    small: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn null_index_test() {
        let index = NullableIndex::new(0);
        assert!(index.value() == 0);
        let index = NullableIndex::new(u32::MAX);
        assert!(index.value() == u32::MAX);
    }
}