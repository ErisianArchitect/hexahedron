use std::{borrow::Borrow, hash::{DefaultHasher, Hash, Hasher}};

use paste::paste;
use rand::{rngs::StdRng, SeedableRng};
use sha2::Digest;
use twox_hash::XxHash64;

pub fn make_rng<R: RngSource>(source: R) -> StdRng {
    source.make_rng()
}

pub fn make_rng_from_hash<T: Hash>(source: T) -> StdRng {
    let mut hasher = XxHash64::default();
    source.hash(&mut hasher);
    StdRng::seed_from_u64(hasher.finish())
}

pub trait SeedSource {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()>;
    fn seed(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        // let mut hasher = sha2::Sha256::default();
        self.write(&mut hasher).unwrap();
        let fin = hasher.finalize();
        let mut seed = [0u8; 32];
        seed.copy_from_slice(fin.as_bytes());
        seed
    }
}

pub trait RngSource {
    fn make_rng(&self) -> StdRng;
}

impl<T: SeedSource> RngSource for T {
    fn make_rng(&self) -> StdRng {
        StdRng::from_seed(self.seed())
    }
}

impl<T: SeedSource> SeedSource for &T {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        (*self).write(hasher)
    }
}

macro_rules! seed_source_int_impls {
    ($type:ty) => {
        impl SeedSource for $type {
            fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
                hasher.write_all(&self.to_be_bytes())
            }
        }
    };
}

hexahedron::macros::for_each_int_type!(seed_source_int_impls);

impl SeedSource for bool {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        hasher.write_all(&[if *self { 1u8 } else { 0u8 }])
    }
}

impl SeedSource for char {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        let mut buf = [0u8; 4];
        self.encode_utf8(&mut buf);
        hasher.write_all(&buf)
    }
}

impl SeedSource for f32 {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        hasher.write_all(&self.to_be_bytes())
    }
}

impl SeedSource for f64 {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        hasher.write_all(&self.to_be_bytes())
    }
}

impl SeedSource for &str {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        hasher.write_all(self.as_bytes())
    }
}

impl SeedSource for String {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        hasher.write_all(self.as_bytes())
    }
}

impl<T: SeedSource> SeedSource for &[T] {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        self.iter().try_for_each(|source| source.write(hasher))
    }
}

impl<T: SeedSource> SeedSource for Vec<T> {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        self.as_slice().write(hasher)
    }
}

impl SeedSource for glam::IVec2 {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        (
            self.x,
            self.y
        ).write(hasher)
    }
}

impl SeedSource for glam::IVec3 {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        (
            self.x,
            self.y,
            self.z,
        ).write(hasher)
    }
}

impl SeedSource for glam::IVec4 {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        (
            self.x,
            self.y,
            self.z,
            self.w,
        ).write(hasher)
    }
}

impl SeedSource for glam::Vec2 {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        (
            self.x,
            self.y
        ).write(hasher)
    }
}

impl SeedSource for glam::Vec3 {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        (
            self.x,
            self.y,
            self.z,
        ).write(hasher)
    }
}

impl SeedSource for glam::Vec4 {
    fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
        (
            self.x,
            self.y,
            self.z,
            self.w,
        ).write(hasher)
    }
}

macro_rules! seed_source_tuple_impls {
    ($($type_args:ident),*$(,)?) => {
        paste!{
            impl<$($type_args : SeedSource),*> SeedSource for ($($type_args,)*) {
                #[allow(non_snake_case)]
                fn write<W: std::io::Write>(&self, hasher: &mut W) -> std::io::Result<()> {
                    let (
                        $(
                            [<_ $type_args>],
                        )*
                    ) = self;
                    $(
                        [<_ $type_args>].write(hasher)?;
                    )*
                    Ok(())
                }
            }
        }
    };
    ($([$($type_args:ident),*$(,)?])*) => {
        $(
            seed_source_tuple_impls!{ $($type_args),* }
        )*
    };
}

seed_source_tuple_impls!{
    [T0]
    [T0, T1]
    [T0, T1, T2]
    [T0, T1, T2, T3]
    [T0, T1, T2, T3, T4]
    [T0, T1, T2, T3, T4, T5]
    [T0, T1, T2, T3, T4, T5, T6]
    [T0, T1, T2, T3, T4, T5, T6, T7]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31]
}
