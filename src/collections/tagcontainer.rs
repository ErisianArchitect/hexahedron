use glam::IVec3;

use crate::prelude::SwapVal;
use crate::tag::*;
use crate::math;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagId(u16);

impl TagId {
    pub const NULL: TagId = TagId(0);
    pub const MAX: u16 = 0xffff-1;

    fn new(index: u16) -> Self {
        if index == 0xffff {
            panic!("Index out of range.");
        }
        Self(index + 1)
    }

    #[inline]
    pub fn is_null(self) -> bool {
        self == Self::NULL
    }

    #[inline]
    fn id(self) -> u16 {
        self.0 - 1
    }

    #[inline]
    fn index(self) -> usize {
        self.id() as usize
    }
}

pub struct TagContainer {
    data: Vec<Option<Tag>>,
    unused: Vec<u16>,
}

impl TagContainer {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            unused: Vec::new(),
        }
    }

    pub fn insert<T: Into<Tag>>(&mut self, value: T) -> TagId {
        let value: Tag = value.into();
        if let Some(index) = self.unused.pop() {
            self.data[index as usize].replace(value);
            TagId::new(index)
        } else {
            if self.data.len() >= u16::MAX as usize {
                panic!("Container overflow.");
            }
            let index = TagId::new(self.data.len() as u16);
            self.data.push(Some(value));
            index
        }
    }

    pub fn remove(&mut self, id: TagId) -> Tag {
        if id.is_null() {
            panic!("id is null.");
        }
        let tag = self.data[id.index()].take().expect("Failed to remove Tag.");
        self.unused.push(id.index() as u16);
        tag
    }

    pub fn replace<T: Into<Tag>>(&mut self, id: TagId, tag: T) -> Tag {
        if id.is_null() {
            panic!("id is null.");
        }
        let tag: Tag = tag.into();
        let data = &mut self.data[id.index()];
        assert!(data.is_some(), "TagContainer slot was empty while replacing; Likely program corruption.");
        data.replace(tag).unwrap()
    }

    /// This method can panic if id is invalid.
    pub fn get(&self, id: TagId) -> &Tag {
        if id.is_null() {
            panic!("id is null.");
        }
        self.data[id.index()].as_ref().unwrap()
    }

    /// This method can panic if id is invalid.
    pub fn get_mut(&mut self, id: TagId) -> &mut Tag {
        if id.is_null() {
            panic!("id is null.");
        }
        self.data[id.index()].as_mut().unwrap()
    }

    /// Clears the container and optionally shrinks the buffers by calling shrink_to_fit on them.
    pub fn clear(&mut self, shrink: bool) {
        self.data.clear();
        self.unused.clear();
        if shrink {
            self.data.shrink_to_fit();
            self.unused.shrink_to_fit();
        }
    }
}

impl std::ops::Index<TagId> for TagContainer {
    type Output = Tag;

    fn index(&self, index: TagId) -> &Self::Output {
        self.get(index)
    }
}

impl std::ops::IndexMut<TagId> for TagContainer {
    fn index_mut(&mut self, index: TagId) -> &mut Self::Output {
        self.get_mut(index)
    }
}

#[derive(Default)]
struct IdContainer(Option<Box<[TagId]>>);

impl IdContainer {
    #[inline]
    fn force_ids(&mut self) -> &mut Box<[TagId]> {
        self.0.get_or_insert_with(|| {
            (0..32768).map(|_| TagId::NULL).collect()
        })
    }

    #[inline]
    fn unwrap_mut(&mut self) -> &mut Box<[TagId]> {
        self.0.as_mut().unwrap()
    }

    #[inline]
    fn clear(&mut self) {
        self.0.take();
    }
}

pub struct TagSection {
    ids: IdContainer,
    container: TagContainer,
    non_null_count: u16,
}

impl TagSection {
    pub fn new() -> Self {
        Self {
            ids: IdContainer::default(),
            container: TagContainer::new(),
            non_null_count: 0,
        }
    }

    pub fn insert<C: Into<IVec3>, T: Into<Tag>>(&mut self, coord: C, value: T) -> Tag {
        let coord: IVec3 = coord.into();
        let ids = self.ids.force_ids();
        let index = math::index3::<32>(coord.x, coord.y, coord.z);
        let id = ids[index];
        if !id.is_null() {
            self.container.replace(id, value)
        } else {
            let id = self.container.insert(value);
            ids[index] = id;
            self.non_null_count += 1;
            Tag::Null
        }
    }

    pub fn remove<C: Into<IVec3>>(&mut self, coord: C) -> Tag {
        let coord: IVec3 = coord.into();
        if self.ids.0.is_none() {
            return Tag::Null;
        }
        let ids = self.ids.unwrap_mut();
        let index = math::index3::<32>(coord.x, coord.y, coord.z);
        let id = ids[index].swap(TagId::NULL);
        if id.is_null() {
            Tag::Null
        } else {
            let old = self.container.remove(id);
            self.non_null_count -= 1;
            if self.non_null_count == 0 {
                self.ids.clear();
                self.container.clear(true);
            }
            old
        }
    }

    pub fn get<C: Into<IVec3>>(&self, coord: C) -> Option<&Tag> {
        let coord: IVec3 = coord.into();
        self.ids.0.as_ref().and_then(|ids| {
            let index = math::index3::<32>(coord.x, coord.y, coord.z);
            let id = ids[index];
            if id.is_null() {
                None
            } else {
                Some(self.container.get(id))
            }
        })
    }

    pub fn get_mut<C: Into<IVec3>>(&mut self, coord: C) -> Option<&mut Tag> {
        let coord: IVec3 = coord.into();
        self.ids.0.as_mut().and_then(|ids| {
            let index = math::index3::<32>(coord.x, coord.y, coord.z);
            let id = ids[index];
            if id.is_null() {
                None
            } else {
                Some(self.container.get_mut(id))
            }
        })
    }

    pub fn get_or_insert_with<C: Into<IVec3>, T: Into<Tag>, F: FnOnce() -> T>(&mut self, coord: C, insert: F) -> &mut Tag {
        let coord: IVec3 = coord.into();
        let ids = self.ids.force_ids();
        let index = math::index3::<32>(coord.x, coord.y, coord.z);
        let id = ids[index];
        if id.is_null() {
            let id = self.container.insert(insert());
            ids[index] = id;
            self.non_null_count += 1;
            self.container.get_mut(id)
        } else {
            self.container.get_mut(id)
        }
    }

    pub fn get_or_insert<C: Into<IVec3>, T: Into<Tag>>(&mut self, coord: C, insert: T) -> &mut Tag {
        self.get_or_insert_with(coord, move || insert)
    }
}