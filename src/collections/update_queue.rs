use std::{iter::Map, slice::Iter, vec::IntoIter};

use glam::IVec3;

use crate::prelude::Replace;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UpdateId(u32);

impl UpdateId {
    pub const NULL: UpdateId = UpdateId(0);
    pub const MAX_ID: u32 = u32::MAX - 1;
    #[inline]
    fn new(id: u32) -> Self {
        if id == u32::MAX {
            panic!("id out of bounds.");
        }
        Self(id + 1)
    }

    #[inline]
    fn id(self) -> u32 {
        self.0 - 1
    }

    #[inline]
    fn index(self) -> usize {
        self.id() as usize
    }

    #[inline]
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub const fn is_non_null(self) -> bool {
        self.0 != 0
    }
}

pub struct UpdateEntry {
    coord: IVec3,
    id: UpdateId,
}

impl UpdateEntry {
    pub fn new(coord: IVec3, id: UpdateId) -> Self {
        Self {
            coord,
            id,
        }
    }
}

/// Represents an unordered queue of coordinates for updates in a Voxel world.
pub struct UpdateQueue {
    queue: Vec<UpdateEntry>,
    indices: Vec<u32>,
    unused: Vec<u32>,
}

impl UpdateQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            indices: Vec::new(),
            unused: Vec::new(),
        }
    }

    pub fn insert(&mut self, coord: IVec3) -> UpdateId {
        // index points to unused index in indices
        if let Some(id) = self.unused.pop() {
            let index = id as usize;
            let queue_index = self.queue.len();
            self.indices[index] = queue_index as u32;
            let id = UpdateId::new(id);
            self.queue.push(UpdateEntry::new(coord, id));
            id
        } else {
            if self.queue.len() >= u32::MAX as usize {
                panic!("Queue overflow.");
            }
            let id = UpdateId::new(self.indices.len() as u32);
            let index = self.queue.len() as u32;
            self.indices.push(index);
            self.queue.push(UpdateEntry::new(coord, id));
            id
        }
    }

    // I just realized that it doesn't really make sense to have this.
    pub fn replace(&mut self, id: UpdateId, coord: IVec3) -> IVec3 {
        let index = self.indices[id.index()] as usize;
        self.queue[index].coord.replace(coord)
    }

    pub fn remove(&mut self, id: UpdateId) -> IVec3 {
        let index = self.indices[id.index()] as usize;
        let old = self.queue.swap_remove(index);
        // Check if it wasn't the last element that was removed.
        // If it was the last element, the index in self.indices
        // needs to be updated.
        if index != self.queue.len() {
            let swap_id = self.queue[index].id;
            let swap_index = swap_id.index();
            self.indices[swap_index] = index as u32;
        }
        old.coord
    }

    pub fn iter<'a>(&'a self) -> Map<Iter<'a, UpdateEntry>, fn(&'a UpdateEntry) -> IVec3> {
        #[inline]
        fn iter_helper<'a>(entry: &'a UpdateEntry) -> IVec3 {
            entry.coord
        }
        self.queue.iter().map(iter_helper)
    }

    pub fn clone_queue(&self) -> Vec<IVec3> {
        #[inline]
        fn iter_helper<'a>(entry: &'a UpdateEntry) -> IVec3 {
            entry.coord
        }
        self.queue.iter().map(iter_helper).collect::<Vec<_>>()
    }

    /// If you want to be able to mutate the [UpdateQueue] while iterating over elements in it,
    /// you'll want to use this method.
    #[inline]
    pub fn clone_iter(&self) -> IntoIter<IVec3> {
        self.clone_queue().into_iter()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn update_queue_test() {
        let mut queue = UpdateQueue::new();
        let a = queue.insert(IVec3::new(1,2,3));
        let b = queue.insert(IVec3::new(3,2,1));
        for coord in queue.iter() {
            println!("Coord: {:#?}", coord);
        }
        println!("Remove b");
        queue.remove(b);
        for coord in queue.iter() {
            println!("Coord: {:#?}", coord);
        }
        println!("Remove a");
        queue.remove(a);
        for coord in queue.iter() {
            println!("Coord: {:#?}", coord);
        }
    }
}