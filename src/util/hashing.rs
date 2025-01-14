use std::hash::{Hash, Hasher, DefaultHasher};
use twox_hash::{XxHash32, XxHash64};
use crate::private::{Sealed, seal};

pub struct HashExtMarker;

/// An extension for Hashable types to get the hash of a value using various hashing algorithms.
pub trait HashExt: Hash + Sealed<HashExtMarker> {
    fn std_hash(&self) -> u64;
    fn xxhash32(&self) -> u32;
    fn xxhash32_64(&self) -> u64;
    fn xxhash64(&self) -> u64;
}

seal!(HashExtMarker; where: Hash);

impl<T: Hash> HashExt for T {
    /// Hashes `self` with [std::hash::DefaultHasher] and returns the result.
    fn std_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }

    /// Hashes `self` with [twox_hash::XxHash32] and returns the result.
    fn xxhash32(&self) -> u32 {
        let mut hasher = XxHash32::default();
        self.hash(&mut hasher);
        hasher.finish_32()
    }

    /// Hashes `self` with [twox_hash::XxHash32] and returns the 64-bit result.
    fn xxhash32_64(&self) -> u64 {
        let mut hasher = XxHash32::default();
        self.hash(&mut hasher);
        hasher.finish()
    }

    /// Hashes `self` with [twox_hash::XxHash64] and returns the result.
    fn xxhash64(&self) -> u64 {
        let mut hasher = XxHash64::default();
        self.hash(&mut hasher);
        hasher.finish()
    }
}