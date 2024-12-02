use crate::{collections::tag_container::{TagContainer, TagId}, prelude::{OptionExtension, Replace}, tag::Tag};

use super::SectionIndex;


#[derive(Debug, Default)]
struct IdContainer<const W: i32>(Option<Box<[TagId]>>);

impl<const W: i32> IdContainer<W> {
    const BLOCK_COUNT: usize = (W as usize).pow(3);
    /// return mut ref to inner value if it exists, otherwise allocate inner value and return mut ref.
    #[inline]
    fn get_or_init(&mut self) -> &mut Box<[TagId]> {
        self.0.get_or_insert_with(|| {
            (0..Self::BLOCK_COUNT).map(|_| TagId::NULL).collect()
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

    fn is_allocated(&self) -> bool {
        self.0.is_some()
    }
}

#[derive(Debug, Default)]
pub struct TagSection<const W: i32> {
    ids: IdContainer<W>,
    container: TagContainer,
    non_null_count: u16,
}

impl<const W: i32> TagSection<W> {
    pub const fn new() -> Self {
        Self {
            ids: IdContainer(None),
            container: TagContainer::new(),
            non_null_count: 0,
        }
    }

    pub fn insert<I: SectionIndex<W>, T: Into<Tag>>(&mut self, coord: I, value: T) -> Option<Tag> {
        let ids = self.ids.get_or_init();
        let index = coord.section_index();
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

    pub fn remove<I: SectionIndex<W>>(&mut self, coord: I) -> Option<Tag> {
        if self.ids.0.is_none() {
            return None;
        }
        // This will always succeed because we've early returned if None.
        let ids = self.ids.unwrap_mut();
        let index = coord.section_index();
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

    pub fn get<I: SectionIndex<W>>(&self, coord: I) -> Option<&Tag> {
        self.ids.0.as_ref().and_then(|ids| {
            let index = coord.section_index();
            let id = ids[index];
            if id.is_null() {
                None
            } else {
                Some(self.container.get(id))
            }
        })
    }

    pub fn get_mut<I: SectionIndex<W>>(&mut self, coord: I) -> Option<&mut Tag> {
        self.ids.0.as_mut().and_then(|ids| {
            let index = coord.section_index();
            let id = ids[index];
            if id.is_null() {
                None
            } else {
                Some(self.container.get_mut(id))
            }
        })
    }

    pub fn get_or_insert_with<I: SectionIndex<W>, T: Into<Tag>, F: FnOnce() -> T>(&mut self, coord: I, insert: F) -> &mut Tag {
        let ids = self.ids.get_or_init();
        let index = coord.section_index();
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

    #[inline]
    pub fn get_or_insert<I: SectionIndex<W>, T: Into<Tag>>(&mut self, coord: I, insert: T) -> &mut Tag {
        self.get_or_insert_with(coord, move || insert)
    }

    pub fn is_allocated(&self) -> bool {
        self.ids.is_allocated()
    }
}