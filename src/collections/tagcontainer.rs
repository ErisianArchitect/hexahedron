use crate::tag::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagId(u16);

impl TagId {
    pub const NULL: TagId = TagId(0);

    fn new(index: usize) -> Self {
        if index > 0x7fff {
            panic!("Index out of range.");
        }
        Self(index as u16 | 0x8000)
    }

    #[inline]
    pub fn is_null(self) -> bool {
        self == Self::NULL
    }

    #[inline]
    fn id(self) -> u16 {
        self.0 & 0x7fff
    }

    #[inline]
    pub fn index(self) -> usize {
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
            TagId::new(index as usize)
        } else {
            // Max length for 15 bits.
            if self.data.len() >= 32768 {
                panic!("Container overflow.");
            }
            let index = TagId::new(self.data.len());
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