mod private;
use std::{hash::{DefaultHasher, Hash, Hasher}, io::Write};
use hexmacros::mark;
use twox_hash::{XxHash32, XxHash64};
use sha2::digest::Digest;
use crate::private::*;

/// This function assumes that `bytes` has a length of 8.
#[inline]
fn u64_from_bytes(bytes: &[u8]) -> u64 {
    let mut buffer = [0u8; 8];
    buffer.copy_from_slice(bytes);
    u64::from_be_bytes(buffer)
}

/// This function assumes that `bytes` has a length of at least 32.
#[inline]
fn xor_256_to_64(bytes: &[u8]) -> u64 {
    (0..4).map(|i| {
        let range = i*8..i*8+8;
        u64_from_bytes(&bytes[range])
    }).reduce(|left, right| {
        left ^ right
    }).unwrap()
}

pub trait Pow2Array<T>: Sealed<()> {
    type HALF;

    fn xor_reduce(input: T) -> Self::HALF;
}

pub struct Pow2ArrayImpls;

seal!(Pow2ArrayImpls);

macro_rules! pow2_array {
    ($hi:literal -> $lo:literal -> $($rest:literal)->+) => {
        pow2_array!($hi -> $lo);
        pow2_array!($lo -> $($rest)->+);
    };
    ($hi:literal -> $lo:literal) => {
        impl Pow2Array<[u8; $hi]> for Pow2ArrayImpls {
            type HALF = [u8; $lo];

            #[inline]
            fn xor_reduce(input: [u8; $hi]) -> Self::HALF {
                let mut buffer = [0u8; $lo];
                let lo = &input[..$lo];
                let hi = &input[$lo..];
                std::iter::zip(lo, hi)
                    .map(|(lo, hi)| lo ^ hi)
                    .enumerate()
                    .for_each(|(i, fin)| buffer[i] = fin);
                buffer
            }
        }

        impl Pow2Array<Box<[u8; $hi]>> for Pow2ArrayImpls {
            type HALF = Box<[u8; $lo]>;

            #[inline]
            fn xor_reduce(input: Box<[u8; $hi]>) -> Self::HALF {
                let mut buffer = [0u8; $lo];
                let lo = &input[..$lo];
                let hi = &input[$lo..];
                std::iter::zip(lo, hi)
                    .map(|(lo, hi)| lo ^ hi)
                    .enumerate()
                    .for_each(|(i, fin)| buffer[i] = fin);
                Box::new(buffer)
            }
        }
    };
}

#[rustfmt::skip]
pow2_array!(
    0x8000 ->
    0x4000 ->
    0x2000 ->
    0x1000 ->
    0x0800 ->
    0x0400 ->
    0x0200 ->
    0x0100 ->
    0x0080 ->
    0x0040 ->
    0x0020 ->
    0x0010 ->
    0x0008 ->
    0x0004 ->
    0x0002 ->
    0x0001
);

/// Takes as input a u8 array with a length of a power of 2 and returns an
/// array with length of `input.len/2` where the lower and upper halves are
/// zipped and XORed together to make the final result.
#[inline]
pub fn xor_reduce<T>(array: T) -> <Pow2ArrayImpls as Pow2Array<T>>::HALF
where Pow2ArrayImpls: Pow2Array<T> {
    Pow2ArrayImpls::xor_reduce(array)
}

#[inline]
pub fn xor_bits(value: u64) -> bool {
    (value.count_ones() & 1) != 0
}

#[inline(always)]
fn hash_with<Hasher: std::hash::Hasher, T: Hash>(mut hasher: Hasher, value: T) -> u64 {
    value.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, Default)]
pub struct Sha256Hasher {
    hasher: sha2::Sha256,
}

impl Sha256Hasher {
    #[inline]
    pub fn finalize(self) -> [u8; 32] {
        self.hasher.finalize().into()
    }
}

impl std::hash::Hasher for Sha256Hasher {
    #[inline]
    fn finish(&self) -> u64 {
        let hash = self.hasher.clone().finalize();
        xor_256_to_64(&hash)
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }

    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_i128(&mut self, i: i128) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.write(&i.to_be_bytes());
    }
}

#[derive(Debug, Default)]
pub struct Blake3Hasher {
    hasher: blake3::Hasher,
}

impl Blake3Hasher {
    #[inline]
    pub fn finalize(&self) -> blake3::Hash {
        self.hasher.finalize()
    }
}

impl std::hash::Hasher for Blake3Hasher {
    #[inline]
    fn finish(&self) -> u64 {
        let hash = self.hasher.finalize();
        xor_256_to_64(hash.as_bytes())
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write_all(bytes).unwrap();
    }

    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_i128(&mut self, i: i128) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.write(&i.to_be_bytes());
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.write(&i.to_be_bytes());
    }
}

pub struct HashExtMarker;

/// An extension for Hashable types to get the hash of a value using various hashing algorithms.
pub trait HashExt: Hash + Sealed<HashExtMarker> {
    fn std_hash(&self) -> u64;
    fn xxhash32(&self) -> u32;
    fn xxhash32_64(&self) -> u64;
    fn xxhash64(&self) -> u64;
    fn blake3_64(&self) -> u64;
    fn blake3_256(&self) -> blake3::Hash;
}

#[inline]
pub fn stdhash<T: Hash>(source: T) -> u64 {
    hash_with(DefaultHasher::default(), source)
}

#[inline]
pub fn xxhash32<T: Hash>(source: T) -> u32 {
    let mut hasher = XxHash32::default();
    source.hash(&mut hasher);
    hasher.finish_32()
}

#[inline]
pub fn xxhash32_64<T: Hash>(source: T) -> u64 {
    hash_with(XxHash32::default(), source)
}

#[inline]
pub fn xxhash64<T: Hash>(source: T) -> u64 {
    hash_with(XxHash64::default(), source)
}

#[inline]
pub fn blake3_64<T: Hash>(source: T) -> u64 {
    hash_with(Blake3Hasher::default(), source)
}

#[inline]
pub fn blake3<T: Hash>(source: T) -> blake3::Hash {
    let mut hasher = Blake3Hasher::default();
    source.hash(&mut hasher);
    hasher.finalize()
}

#[inline]
pub fn sha256_64<T: Hash>(source: T) -> u64 {
    hash_with(Sha256Hasher::default(), source)
}

#[inline]
pub fn sha256<T: Hash>(source: T) -> [u8; 32] {
    let mut hasher = Sha256Hasher::default();
    source.hash(&mut hasher);
    hasher.finalize()
}

pub mod deterministic {

    use super::*;

    use hexio::{Deterministic, Writeable};


    struct HasherWriter<T: Hasher>(T);

    impl<T: Hasher> Write for HasherWriter<T> {
        #[inline]
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.write(buf);
            Ok(buf.len())
        }
        
        #[inline]
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<T: Hasher> std::ops::Deref for HasherWriter<T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Hasher> std::ops::DerefMut for HasherWriter<T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    pub trait DeterministicHash: Deterministic + Writeable {}
    mark!(trait = DeterministicHash; <T> for T where T: Deterministic + Writeable);

    #[inline]
    pub fn xxhash32<T: DeterministicHash>(source: T) -> u32 {
        let hasher = XxHash32::default();
        let mut writer = HasherWriter(hasher);
        source.write_to(&mut writer).unwrap();
        writer.0.finish_32()
    }

    #[inline]
    pub fn xxhash32_64<T: DeterministicHash>(source: T) -> u64 {
        let hasher = XxHash32::default();
        let mut writer = HasherWriter(hasher);
        source.write_to(&mut writer).unwrap();
        writer.finish()
    }

    #[inline]
    pub fn xxhash64<T: DeterministicHash>(source: T) -> u64 {
        let hasher = XxHash64::default();
        let mut writer = HasherWriter(hasher);
        source.write_to(&mut writer).unwrap();
        writer.finish()
    }

    #[inline]
    pub fn blake3_64<T: DeterministicHash>(source: T) -> u64 {
        let hasher = Blake3Hasher::default();
        let mut writer = HasherWriter(hasher);
        source.write_to(&mut writer).unwrap();
        writer.finish()
    }

    #[inline]
    pub fn blake3<T: DeterministicHash>(source: T) -> blake3::Hash {
        let hasher = Blake3Hasher::default();
        let mut writer = HasherWriter(hasher);
        source.write_to(&mut writer).unwrap();
        writer.0.finalize()
    }

    #[inline]
    pub fn sha256_64<T: DeterministicHash>(source: T) -> u64 {
        let hasher = Sha256Hasher::default();
        let mut writer = HasherWriter(hasher);
        source.write_to(&mut writer).unwrap();
        writer.finish()
    }

    #[inline]
    pub fn sha256<T: DeterministicHash>(source: T) -> [u8; 32] {
        let hasher = Sha256Hasher::default();
        let mut writer = HasherWriter(hasher);
        source.write_to(&mut writer).unwrap();
        writer.0.finalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn hash_test() {
        let hash = deterministic::sha256("hello, world!");
        println!("{hash:?}");
    }
}