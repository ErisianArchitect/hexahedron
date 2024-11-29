use crate::tag::*;

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
    pub const fn new() -> Self {
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
            let id = TagId::new(self.data.len() as u16);
            self.data.push(Some(value));
            id
        }
    }

    pub fn remove(&mut self, id: TagId) -> Tag {
        if id.is_null() {
            panic!("id is null.");
        }
        let tag = self.data[id.index()].take().expect("TagContainer slot was empty.");
        self.unused.push(id.index() as u16);
        tag
    }

    pub fn replace<T: Into<Tag>>(&mut self, id: TagId, tag: T) -> Tag {
        if id.is_null() {
            panic!("id is null.");
        }
        let tag: Tag = tag.into();
        let data = &mut self.data[id.index()];
        assert!(data.is_some(), "TagContainer slot was empty while replacing; this likely indicates program corruption.");
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