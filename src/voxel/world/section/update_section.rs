use crate::{collections::update_queue::*, math::index3, prelude::{OptionExtension, Replace}, util::change::Change};

pub struct UpdateSection<const W: i32> {
    update_refs: Option<Box<[UpdateId]>>,
    enabled_count: u16,
}

impl<const W: i32> UpdateSection<W> {
    const BLOCK_COUNT: usize = (W as usize).pow(3);
    #[inline]
    pub const fn new() -> Self {
        Self {
            update_refs: None,
            enabled_count: 0,
        }
    }

    pub fn is_allocated(&self) -> bool {
        self.update_refs.is_some()
    }

    pub fn get<C: Into<(i32, i32, i32)>>(&self, coord: C) -> UpdateId {
        let Some(refs) = self.update_refs.as_ref() else {
            return UpdateId::NULL;
        };
        let (x, y, z) = coord.into();
        let index = index3::<W>(x, y, z);
        refs[index]
    }

    pub fn set<C: Into<(i32, i32, i32)>>(&mut self, coord: C, value: UpdateId) -> Change<UpdateId> {
        if self.update_refs.is_none() && value.is_null() {
            return Change::Unchanged;
        }
        let (x, y, z) = coord.into();
        let refs = self.update_refs.get_or_insert_with(|| (0..Self::BLOCK_COUNT).map(|_| UpdateId::NULL).collect());
        let index = index3::<W>(x, y, z);
        let old = refs[index].replace(value);
        match (old.is_null(), value.is_null()) {
            (true, true) => Change::Unchanged,
            (false, true) => {
                self.enabled_count -= 1;
                if self.enabled_count == 0 {
                    self.update_refs.drop();
                }
                Change::Changed(old)
            }
            (true, false) => {
                self.enabled_count += 1;
                Change::Changed(old)
            }
            (false, false) => if old != value {
                Change::Changed(old)
            } else {
                Change::Unchanged
            }
        }
    }
}