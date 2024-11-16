use crate::{collections::tag_container::{TagContainer, TagId}, math, prelude::{OptionExtension, Replace}, tag::Tag};



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
        self.0.drop();
    }
}

#[derive(Debug, Default)]
pub struct TagSection {
    ids: IdContainer,
    container: TagContainer,
    non_null_count: u16,
}

impl TagSection {
    pub const fn new() -> Self {
        Self {
            ids: IdContainer(None),
            container: TagContainer::new(),
            non_null_count: 0,
        }
    }

    pub fn insert<C: Into<(i32, i32, i32)>, T: Into<Tag>>(&mut self, coord: C, value: T) -> Option<Tag> {
        let coord: (i32, i32, i32) = coord.into();
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

    pub fn remove<C: Into<(i32, i32, i32)>>(&mut self, coord: C) -> Option<Tag> {
        let coord: (i32, i32, i32) = coord.into();
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

    pub fn get<C: Into<(i32, i32, i32)>>(&self, coord: C) -> Option<&Tag> {
        let coord: (i32, i32, i32) = coord.into();
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

    pub fn get_mut<C: Into<(i32, i32, i32)>>(&mut self, coord: C) -> Option<&mut Tag> {
        let coord: (i32, i32, i32) = coord.into();
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

    pub fn get_or_insert_with<C: Into<(i32, i32, i32)>, T: Into<Tag>, F: FnOnce() -> T>(&mut self, coord: C, insert: F) -> &mut Tag {
        let coord: (i32, i32, i32) = coord.into();
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

    pub fn get_or_insert<C: Into<(i32, i32, i32)>, T: Into<Tag>>(&mut self, coord: C, insert: T) -> &mut Tag {
        self.get_or_insert_with(coord, move || insert)
    }
}