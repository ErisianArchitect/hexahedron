use std::{iter::Map, marker::PhantomData, sync::atomic::AtomicU64, vec::Drain};
use crate::util::extensions::SwapVal;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PoolId<M: Copy>(u64, PhantomData<M>);

impl<M: Copy> PoolId<M> {
    const           INDEX_BITS: u64 = 0b0000000000000000000000000000000011111111111111111111111111111111;
    const      GENERATION_BITS: u64 = 0b0000000000111111111111111111111100000000000000000000000000000000;
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
    pub fn is_null(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub fn is_non_null(self) -> bool {
        self.0 != 0
    }

    /// Swaps this [PoolId] with NULL and returns the old Id.
    pub fn swap_null(&mut self) -> Self {
        self.swap(PoolId::NULL)
    }
    
    #[must_use]
    fn new(pool_id: u64, index: usize, generation: u64) -> Self {
        let index = index as u64 + 1;
        if index > Self::INDEX_BITS {
            panic!("Index out of bounds.");
        }
        if generation > Self::GENERATION_MAX {
            panic!("Generation out of bounds.");
        }
        if pool_id > Self::POOL_ID_MAX {
            panic!("Pool Id out of bounds.");
        }
        Self(index | pool_id << Self::POOL_ID_OFFSET | generation << Self::GENERATION_ID_OFFSET, PhantomData)
    }

    
    #[must_use]
    pub fn id(self) -> u64 {
        self.0
    }

    /// Do not call this function on a null ID.
    #[must_use]
    pub fn index(self) -> usize {
        if self.is_null() {
            panic!("index() on null PollId.");
        }
        ((self.0 & Self::INDEX_BITS) as usize) - 1
    }

    
    #[must_use]
    pub fn generation(self) -> u64 {
        self.0 >> Self::GENERATION_ID_OFFSET & Self::GENERATION_MAX
    }

    
    #[must_use]
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
            .rem_euclid(Self::GENERATION_MAX);
        Self::new(pool_id, index, generation + 1)
    }
}

/// An unordered object pool with O(1) lookup, insertion, deletion, and iteration.
/// Sounds too good to be true!
/// You can have 2^10 [ObjectPool]s before [PoolId]s between [ObjectPool]s start to collide.
/// You can store 2^32 elements.
/// I
#[derive(Debug)]
pub struct ObjectPool<T, M: Copy = &'static T> {
    pool: Vec<(PoolId<M>, T)>,
    indices: Vec<usize>,
    unused: Vec<PoolId<M>>,
    id: u64,
}

impl<T,M: Copy> ObjectPool<T,M> {
    
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
    fn next_id() -> u64 {
        static mut ID: AtomicU64 = AtomicU64::new(0);
        unsafe {
            ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        }
    }

    /// Insertion order is not maintained.
    #[must_use]
    pub fn insert(&mut self, value: T) -> PoolId<M> {
        if let Some(unused_index) = self.unused.pop() {
            let new_id = unused_index.increment_generation();
            self.indices[new_id.index()] = self.pool.len();
            self.pool.push((new_id, value));
            new_id
        } else {
            let index = self.indices.len();
            let pool_index = self.pool.len();
            let id = PoolId::new(self.id, index, 0);
            self.pool.push((id, value));
            self.indices.push(pool_index);
            id
        }
    }
    
    pub fn remove(&mut self, id: PoolId<M>) {
        if id.is_null() {
            return;
        }
        if id.pool_id() != self.id {
            panic!("Id does not belong to this pool.");
        }
        if id.index() >= self.indices.len() {
            panic!("Out of bounds");
        }
        let pool_index = self.indices[id.index()];
        if self.pool[pool_index].0.0 != id.0 {
            panic!("Dead pool ID");
        }
        self.pool.swap_remove(pool_index);
        if pool_index == self.pool.len() {
            return;
        }
        let index_index = self.pool[pool_index].0;
        self.indices[index_index.index()] = pool_index;
        self.unused.push(id);
    }

    /// Removes the old id and then inserts the new value and replaces the id with the new id.
    pub fn swap_insert(&mut self, id: &mut PoolId<M>, insert: T) {
        let old = id.swap_null();
        self.remove(old);
        *id = self.insert(insert);
    }

    pub fn pop(&mut self) -> Option<T> {
        let (id, value) = self.pool.pop()?;
        self.unused.push(id);
        Some(value)
    }

    pub fn is_empty(&self) -> bool {
        self.pool.is_empty()
    }

    pub fn len(&self) -> usize {
        self.pool.len()
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    #[must_use]
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
        if self.pool[pool_index].0.0 != id.0 {
            return None;
        }
        Some(&self.pool[pool_index].1)
    }

    #[must_use]
    pub fn get_mut(&mut self, id: PoolId<M>) -> Option<&mut T> {
        if id.is_null() || id.pool_id() != self.id {
            return None;
        }
        let pool_index = self.indices[id.index()];
        if self.pool[pool_index].0.0 != id.0 {
            return None;
        }
        Some(&mut self.pool[pool_index].1)
    }

    #[must_use]
    pub fn reconstruct_id(&self, index: usize, generation: u64) -> PoolId<M> {
        PoolId::new(self.id, index, generation)
    }

    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = (PoolId<M>, &T)> {
        self.pool.iter().map(|(id, item)| (*id, item))
    }

    #[must_use]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (PoolId<M>, &mut T)> {
        self.pool.iter_mut().map(|(id, item)| (*id, item))
    }

    #[must_use]
    pub fn drain(&mut self) -> Map<Drain<'_, (PoolId<M>, T)>, fn((PoolId<M>, T)) -> T> {
        self.unused.clear();
        self.indices.clear();
        fn drain_helper<T,M: Copy>((_id, item): (PoolId<M>, T)) -> T {
            item
        }
        self.pool.drain(..).map(drain_helper::<T,M>)
    }

    pub fn clear(&mut self) {
        self.indices.clear();
        self.unused.clear();
        self.pool.clear();
    }
}

impl<T,M: Copy> IntoIterator for ObjectPool<T,M> {
    type IntoIter = ObjectPoolIterator<T,M>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        ObjectPoolIterator {
            iter: self.pool.into_iter()
        }
    }
}

pub struct ObjectPoolIterator<T,M: Copy> {
    iter: std::vec::IntoIter<(PoolId<M>, T)>,
}

impl<T,M: Copy> Iterator for ObjectPoolIterator<T,M> {
    type Item = T;
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, value)| value)
    }
}

impl<T,M: Copy> std::ops::Index<PoolId<M>> for ObjectPool<T,M> {
    type Output = T;
    fn index(&self, index: PoolId<M>) -> &Self::Output {
        self.get(index).expect("PoolId was invalid")
    }
}

impl<T, M: Copy> std::ops::IndexMut<PoolId<M>> for ObjectPool<T,M> {
    fn index_mut(&mut self, index: PoolId<M>) -> &mut Self::Output {
        self.get_mut(index).expect("PoolId was invalid")
    }
}

impl<M: Copy> std::fmt::Display for PoolId<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PoolId(pool_id={},index={},generation={})", self.pool_id(), self.index(), self.generation())
    }
}