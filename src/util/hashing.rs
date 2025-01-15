use std::{hash::{DefaultHasher, Hash, Hasher}, io::Write};
use hexmacros::mark;
use twox_hash::{XxHash32, XxHash64};
use sha2::digest::Digest;
use crate::private::Sealed;

/// This function assumes that `bytes` has a length of 8.
fn u64_from_bytes(bytes: &[u8]) -> u64 {
    let mut buffer = [0u8; 8];
    buffer.copy_from_slice(bytes);
    u64::from_be_bytes(buffer)
}

/// This function assumes that `bytes` has a length of at least 32.
fn xor_256_to_64(bytes: &[u8]) -> u64 {
    (0..4).map(|i| {
        let range = i*8..i*8+8;
        u64_from_bytes(&bytes[range])
    }).reduce(|left, right| {
        left ^ right
    }).unwrap()
}

#[inline(always)]
fn hash_with<Hasher: std::hash::Hasher, T: Hash>(mut hasher: Hasher, value: T) -> u64 {
    value.hash(&mut hasher);
    hasher.finish()
}

pub struct Blake3Hasher {
    hasher: blake3::Hasher,
}

pub struct Sha256Hasher {
    hasher: sha2::Sha256,
}

impl Sha256Hasher {
    pub fn finalize(self) -> [u8; 32] {
        self.hasher.finalize().into()
    }
}

impl Default for Sha256Hasher {
    fn default() -> Self {
        Self {
            hasher: sha2::Sha256::default()
        }
    }
}

impl std::hash::Hasher for Sha256Hasher {
    fn finish(&self) -> u64 {
        let hash = self.hasher.clone().finalize();
        xor_256_to_64(&hash)
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }

    fn write_i8(&mut self, i: i8) {
        self.write(&i.to_be_bytes());
    }

    fn write_i16(&mut self, i: i16) {
        self.write(&i.to_be_bytes());
    }

    fn write_i32(&mut self, i: i32) {
        self.write(&i.to_be_bytes());
    }

    fn write_i64(&mut self, i: i64) {
        self.write(&i.to_be_bytes());
    }

    fn write_i128(&mut self, i: i128) {
        self.write(&i.to_be_bytes());
    }

    fn write_isize(&mut self, i: isize) {
        self.write(&i.to_be_bytes());
    }

    fn write_u8(&mut self, i: u8) {
        self.write(&i.to_be_bytes());
    }

    fn write_u16(&mut self, i: u16) {
        self.write(&i.to_be_bytes());
    }

    fn write_u32(&mut self, i: u32) {
        self.write(&i.to_be_bytes());
    }

    fn write_u64(&mut self, i: u64) {
        self.write(&i.to_be_bytes());
    }

    fn write_u128(&mut self, i: u128) {
        self.write(&i.to_be_bytes());
    }

    fn write_usize(&mut self, i: usize) {
        self.write(&i.to_be_bytes());
    }
}

impl Default for Blake3Hasher {
    fn default() -> Self {
        Self {
            hasher: blake3::Hasher::default()
        }
    }
}

impl std::hash::Hasher for Blake3Hasher {
    #[inline]
    fn finish(&self) -> u64 {
        let hash = self.hasher.finalize();
        let final_hash = (0..4).map(|i| {
            let range = i*8..i*8+8;
            u64_from_bytes(&hash.as_bytes()[range])
        }).reduce(|left, right| {
            left ^ right
        }).unwrap();
        final_hash
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write_all(bytes).unwrap();
    }

    fn write_i8(&mut self, i: i8) {
        self.write(&i.to_be_bytes());
    }

    fn write_i16(&mut self, i: i16) {
        self.write(&i.to_be_bytes());
    }

    fn write_i32(&mut self, i: i32) {
        self.write(&i.to_be_bytes());
    }

    fn write_i64(&mut self, i: i64) {
        self.write(&i.to_be_bytes());
    }

    fn write_i128(&mut self, i: i128) {
        self.write(&i.to_be_bytes());
    }

    fn write_isize(&mut self, i: isize) {
        self.write(&i.to_be_bytes());
    }

    fn write_u8(&mut self, i: u8) {
        self.write(&i.to_be_bytes());
    }

    fn write_u16(&mut self, i: u16) {
        self.write(&i.to_be_bytes());
    }

    fn write_u32(&mut self, i: u32) {
        self.write(&i.to_be_bytes());
    }

    fn write_u64(&mut self, i: u64) {
        self.write(&i.to_be_bytes());
    }

    fn write_u128(&mut self, i: u128) {
        self.write(&i.to_be_bytes());
    }

    fn write_usize(&mut self, i: usize) {
        self.write(&i.to_be_bytes());
    }
}

impl Blake3Hasher {
    #[inline]
    pub fn finalize(&self) -> blake3::Hash {
        self.hasher.finalize()
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

    use crate::io::{Deterministic, Writeable};


    struct HasherWriter<T: Hasher>(T);

    impl<T: Hasher> Write for HasherWriter<T> {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.write(buf);
            Ok(buf.len())
        }
    
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<T: Hasher> HasherWriter<T> {
        fn finish(self) -> u64 {
            self.0.finish()
        }
    }

    impl<T: Hasher> std::ops::Deref for HasherWriter<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Hasher> std::ops::DerefMut for HasherWriter<T> {
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

// seal!(HashExtMarker; where: Hash);
// mark!(
//     trait = crate::private::Sealed<HashExtMarker>;
//     <T> for T where T: std::hash::Hash
// );

// impl<T: Hash> HashExt for T {
//     /// Hashes `self` with [std::hash::DefaultHasher] and returns the result.
//     fn std_hash(&self) -> u64 {
//         let mut hasher = DefaultHasher::default();
//         self.hash(&mut hasher);
//         hasher.finish()
//     }

//     /// Hashes `self` with [twox_hash::XxHash32] and returns the result.
//     fn xxhash32(&self) -> u32 {
//         let mut hasher = XxHash32::default();
//         self.hash(&mut hasher);
//         hasher.finish_32()
//     }

//     /// Hashes `self` with [twox_hash::XxHash32] and returns the 64-bit result.
//     fn xxhash32_64(&self) -> u64 {
//         let mut hasher = XxHash32::default();
//         self.hash(&mut hasher);
//         hasher.finish()
//     }

//     /// Hashes `self` with [twox_hash::XxHash64] and returns the result.
//     fn xxhash64(&self) -> u64 {
//         let mut hasher = XxHash64::default();
//         self.hash(&mut hasher);
//         hasher.finish()
//     }

//     /// Hashes `self` with [blake3::Hasher] and returns the 64-bit result.
//     fn blake3_64(&self) -> u64 {
//         let mut hasher = Blake3Hasher::default();
//         self.hash(&mut hasher);
//         hasher.finish()
//     }

//     /// Hashes `self` with [blake3::Hasher] and returns the 256-bit [blake3::Hash].
//     fn blake3_256(&self) -> blake3::Hash {
//         let mut hasher = Blake3Hasher::default();
//         self.hash(&mut hasher);
//         hasher.finalize()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn hash_test() {
        let hash = deterministic::sha256("hello, world!");
        println!("{hash:?}");
    }
}