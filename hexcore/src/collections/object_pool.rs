use std::{iter::Map, marker::PhantomData, sync::atomic::AtomicU64, vec::Drain};
// use crate::util::extensions::Replace;
use std::sync::LazyLock;

#[derive(Debug, Default, Clone, Copy, Eq, Ord, Hash)]
pub struct PoolId<M: Copy>(u64, PhantomData<M>);

impl<M: Copy> PartialEq for PoolId<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl<M: Copy> PartialOrd for PoolId<M> {
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.0.ge(&other.0)
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.0.gt(&other.0)
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.0.le(&other.0)
    }
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.0.lt(&other.0)
    }
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<M: Copy> PoolId<M> {
    // 32 bits
    const           INDEX_BITS: u64 = 0b0000000000000000000000000000000011111111111111111111111111111111;
    // 22 bits
    const      GENERATION_BITS: u64 = 0b0000000000111111111111111111111100000000000000000000000000000000;
    // 10 bits
    const         POOL_ID_BITS: u64 = 0b1111111111000000000000000000000000000000000000000000000000000000;
    // This const isn't used right now, but it might be used in the future, so just leave it in.
    #[allow(unused)]
    const         INDEX_OFFSET: u32 = Self::INDEX_BITS.trailing_zeros();
    const       POOL_ID_OFFSET: u32 = Self::POOL_ID_BITS.trailing_zeros();
    const GENERATION_ID_OFFSET: u32 = Self::GENERATION_BITS.trailing_zeros();
    // This const isn't used right now, but it might be used in the future, so just leave it in.
    #[allow(unused)]
    const            INDEX_MAX: u64 = Self::INDEX_BITS >> Self::INDEX_OFFSET;
    const       GENERATION_MAX: u64 = Self::GENERATION_BITS >> Self::GENERATION_ID_OFFSET; 
    const          POOL_ID_MAX: u64 = Self::POOL_ID_BITS >> Self::POOL_ID_OFFSET;
    pub const NULL: Self = Self(0, PhantomData);

    #[must_use]
    #[inline]
    fn new(pool_id: u64, index: usize, generation: u64) -> Self {
        let index = index as u64 + 1;
        debug_assert!(index <= Self::INDEX_BITS, "Index out of bounds.");
        debug_assert!(generation <= Self::GENERATION_MAX, "Generation out of bounds.");
        debug_assert!(pool_id <= Self::POOL_ID_MAX, "Pool ID out of bounds.");
        Self(index | pool_id << Self::POOL_ID_OFFSET | generation << Self::GENERATION_ID_OFFSET, PhantomData)
    }
    
    #[must_use]
    #[inline]
    pub fn is_null(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    #[inline]
    pub fn is_non_null(self) -> bool {
        self.0 != 0
    }

    /// Swaps this [PoolId] with NULL and returns the old Id.
    #[inline]
    pub fn swap_null(&mut self) -> Self {
        std::mem::replace(self, PoolId::NULL)
    }
    
    #[must_use]
    #[inline]
    pub fn id(self) -> u64 {
        self.0
    }

    /// Do not call this function on a null ID.
    #[must_use]
    #[inline]
    pub fn index(self) -> usize {
        debug_assert!(self.is_non_null(), "index() on PoolId(null).");
        ((self.0 & Self::INDEX_BITS) as usize) - 1
    }

    
    #[must_use]
    #[inline]
    pub fn generation(self) -> u64 {
        self.0 >> Self::GENERATION_ID_OFFSET & Self::GENERATION_MAX
    }

    
    #[must_use]
    #[inline]
    pub fn pool_id(self) -> u64 {
        self.0 >> Self::POOL_ID_OFFSET & Self::POOL_ID_MAX
    }

    /// Increment Generation
    #[must_use]
    fn increment_generation(self) -> Self {
        let pool_id = self.pool_id();
        let index = self.index();
        let generation = self.generation()
            // Roll the generation around. It's unlikely for IDs to collide.
            .rem_euclid(const { Self::GENERATION_MAX + 1 });
        Self::new(pool_id, index, generation + 1)
    }
}

#[derive(Debug)]
pub struct PoolEntry<T, M: Copy = &'static T> {
    id: PoolId<M>,
    value: T,
}

impl<T, M: Copy> PoolEntry<T, M> {
    #[inline]
    fn new(id: PoolId<M>, value: T) -> Self {
        Self {
            id,
            value
        }
    }
}

/// An unordered object pool with O(1) lookup, insertion, deletion, and iteration.
/// Sounds too good to be true!
/// You can have 2^10 [ObjectPool]s before [PoolId]s between [ObjectPool]s start to collide.
/// You can store 2^32 elements.
#[derive(Debug)]
pub struct ObjectPool<T, M: Copy = &'static T> {
    pool: Vec<PoolEntry<T, M>>,
    indices: Vec<usize>,
    unused: Vec<PoolId<M>>,
    id: u64,
}

static OBJECT_POOL_ID_COUNTER: LazyLock<AtomicU64> = LazyLock::new(|| AtomicU64::new(0));

impl<T, M: Copy> ObjectPool<T, M> {
    
    #[must_use]
    pub fn new() -> Self {
        Self {
            pool: Vec::new(),
            indices: Vec::new(),
            unused: Vec::new(),
            id: Self::next_id(),
        }
    }

    #[must_use]
    #[inline]
    fn next_id() -> u64 {
        OBJECT_POOL_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed).rem_euclid(PoolId::<M>::POOL_ID_MAX)
    }

    /// Insertion order is not maintained.
    #[must_use]
    pub fn insert(&mut self, value: T) -> PoolId<M> {
        if let Some(unused_index) = self.unused.pop() {
            let new_id = unused_index.increment_generation();
            self.indices[new_id.index()] = self.pool.len();
            self.pool.push(PoolEntry::new(new_id, value));
            new_id
        } else {
            let index = self.indices.len();
            let pool_index = self.pool.len();
            let id = PoolId::new(self.id, index, 0);
            self.pool.push(PoolEntry::new(id, value));
            self.indices.push(pool_index);
            id
        }
    }
    
    pub fn remove(&mut self, id: PoolId<M>) -> T {
        debug_assert!(id.is_non_null(), "ID was null.");
        debug_assert!(id.pool_id() == self.id, "ID does not belong to this pool.");
        debug_assert!(id.index() < self.indices.len(), "Out of bounds.");
        let pool_index = self.indices[id.index()];
        debug_assert!(self.pool[pool_index].id == id, "Dead pool ID.");
        let old = self.pool.swap_remove(pool_index).value;
        // If we didn't just swap_remove the last element, then
        // we need to adjust the index in the ObjectPool.
        if pool_index != self.pool.len() {
            let index_index = self.pool[pool_index].id;
            self.indices[index_index.index()] = pool_index;
            self.unused.push(id);
        }
        old
    }

    /// Replaces the value with the given ID with a new value.
    pub fn replace(&mut self, id: PoolId<M>, value: T) -> T {
        debug_assert!(id.is_non_null(), "Provided a null ID.");
        debug_assert!(id.pool_id() == self.id, "ID does not belong to this pool.");
        debug_assert!(id.index() < self.indices.len(), "Out of bounds.");
        let pool_index = self.indices[id.index()];
        std::mem::replace(&mut self.pool[pool_index].value, value)
    }

    pub fn pop(&mut self) -> Option<T> {
        let PoolEntry {id, value} = self.pool.pop()?;
        self.unused.push(id);
        Some(value)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pool.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.pool.len()
    }

    #[inline]
    pub fn id(&self) -> u64 {
        self.id
    }

    #[must_use]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            pool: Vec::with_capacity(capacity),
            indices: Vec::with_capacity(capacity),
            unused: Vec::new(),
            id: Self::next_id(),
        }
    }

    #[must_use]
    pub fn get(&self, id: PoolId<M>) -> Option<&T> {
        if id.is_null() || id.pool_id() != self.id {
            return None;
        }
        let pool_index = self.indices[id.index()];
        debug_assert!(self.pool[pool_index].id == id, "Corrupted ObjectPool: internal pool id does not match");
        Some(&self.pool[pool_index].value)
    }

    #[must_use]
    pub fn get_mut(&mut self, id: PoolId<M>) -> Option<&mut T> {
        if id.is_null() || id.pool_id() != self.id {
            return None;
        }
        let pool_index = self.indices[id.index()];
        debug_assert!(self.pool[pool_index].id == id, "Corrupted ObjectPool: internal pool id does not match");
        Some(&mut self.pool[pool_index].value)
    }

    #[must_use]
    pub fn reconstruct_id(&self, index: usize, generation: u64) -> PoolId<M> {
        PoolId::new(self.id, index, generation)
    }

    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = (PoolId<M>, &T)> {
        self.pool.iter().map(|PoolEntry {id, value}| (*id, value))
    }

    #[must_use]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (PoolId<M>, &mut T)> {
        self.pool.iter_mut().map(|PoolEntry {id, value}| (*id, value))
    }

    #[must_use]
    pub fn drain(&mut self) -> Map<Drain<'_, PoolEntry<T, M>>, fn(PoolEntry<T, M>) -> T> {
        self.unused.clear();
        self.indices.clear();
        fn drain_helper<T,M: Copy>(entry: PoolEntry<T, M>) -> T {
            entry.value
        }
        self.pool.drain(..).map(drain_helper::<T,M>)
    }

    /// Clears the [ObjectPool].
    pub fn clear(&mut self) {
        self.indices.clear();
        self.unused.clear();
        self.pool.clear();
    }
}

impl<T,M: Copy> IntoIterator for ObjectPool<T,M> {
    type IntoIter = ObjectPoolIterator<T,M>;
    type Item = T;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ObjectPoolIterator {
            iter: self.pool.into_iter()
        }
    }
}

pub struct ObjectPoolIterator<T,M: Copy> {
    iter: std::vec::IntoIter<PoolEntry<T, M>>,
}

impl<T,M: Copy> Iterator for ObjectPoolIterator<T,M> {
    type Item = T;
    
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|PoolEntry { value, .. }| value)
    }
}

impl<T,M: Copy> std::ops::Index<PoolId<M>> for ObjectPool<T,M> {
    type Output = T;
    #[inline]
    fn index(&self, index: PoolId<M>) -> &Self::Output {
        self.get(index).expect("PoolId was invalid")
    }
}

impl<T, M: Copy> std::ops::IndexMut<PoolId<M>> for ObjectPool<T,M> {
    #[inline]
    fn index_mut(&mut self, index: PoolId<M>) -> &mut Self::Output {
        self.get_mut(index).expect("PoolId was invalid")
    }
}

impl<M: Copy> std::fmt::Display for PoolId<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            write!(f, "PoolId(null)")
        } else {
            write!(f, "PoolId(pool_id={},index={},generation={})", self.pool_id(), self.index(), self.generation())
        }
    }
}