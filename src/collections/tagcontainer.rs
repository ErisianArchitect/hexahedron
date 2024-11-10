use crate::prelude::Replace;
use crate::tag::*;
use crate::math;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    #[inline]
    pub fn inner(self) -> u16 {
        self.0
    }
}

// TODO: Tags should be in some kind of synchronization primitive such as Arc<RwLock<Tag>>.
//       This will allow sharing across threads.
#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
struct IdContainer(Option<Box<[TagId]>>);

impl IdContainer {
    /// return mut ref to inner value if it exists, otherwise allocate inner value and return mut ref.
    #[inline]
    fn get_or_init(&mut self) -> &mut Box<[TagId]> {
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

#[derive(Debug, Default)]
pub struct TagSection {
    ids: IdContainer,
    container: TagContainer,
    non_null_count: u16,
}

pub type Coord = (i32, i32, i32);

impl TagSection {
    pub fn new() -> Self {
        Self {
            ids: IdContainer::default(),
            container: TagContainer::new(),
            non_null_count: 0,
        }
    }

    ///
    pub fn insert<C: Into<Coord>, T: Into<Tag>>(&mut self, coord: C, value: T) -> Option<Tag> {
        let coord: Coord = coord.into();
        let ids = self.ids.get_or_init();
        let index = math::index3::<32>(coord.0, coord.1, coord.2);
        let id = ids[index];
        if id.is_null() {
            let id = self.container.insert(value);
            ids[index] = id;
            self.non_null_count += 1;
            None
        } else {
            Some(self.container.replace(id, value))
        }
    }

    pub fn remove<C: Into<Coord>>(&mut self, coord: C) -> Option<Tag> {
        let coord: Coord = coord.into();
        if self.ids.0.is_none() {
            return None;
        }
        // This will always succeed because we've early returned if None.
        let ids = self.ids.unwrap_mut();
        let index = math::index3::<32>(coord.0, coord.1, coord.2);
        let id = ids[index].replace(TagId::NULL);
        if id.is_null() {
            None
        } else {
            let old = self.container.remove(id);
            self.non_null_count -= 1;
            if self.non_null_count == 0 {
                self.ids.clear();
                self.container.clear(true);
            }
            Some(old)
        }
    }

    pub fn get<C: Into<Coord>>(&self, coord: C) -> Option<&Tag> {
        let coord: Coord = coord.into();
        self.ids.0.as_ref().and_then(|ids| {
            let index = math::index3::<32>(coord.0, coord.1, coord.2);
            let id = ids[index];
            if id.is_null() {
                None
            } else {
                Some(self.container.get(id))
            }
        })
    }

    pub fn get_mut<C: Into<Coord>>(&mut self, coord: C) -> Option<&mut Tag> {
        let coord: Coord = coord.into();
        self.ids.0.as_mut().and_then(|ids| {
            let index = math::index3::<32>(coord.0, coord.1, coord.2);
            let id = ids[index];
            if id.is_null() {
                None
            } else {
                Some(self.container.get_mut(id))
            }
        })
    }

    pub fn get_or_insert_with<C: Into<Coord>, T: Into<Tag>, F: FnOnce() -> T>(&mut self, coord: C, insert: F) -> &mut Tag {
        let coord: Coord = coord.into();
        let ids = self.ids.get_or_init();
        let index = math::index3::<32>(coord.0, coord.1, coord.2);
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

    pub fn get_or_insert<C: Into<Coord>, T: Into<Tag>>(&mut self, coord: C, insert: T) -> &mut Tag {
        self.get_or_insert_with(coord, move || insert)
    }
}