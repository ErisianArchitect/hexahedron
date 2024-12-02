use crate::{prelude::{OptionExtension, Replace}, util::change::Change, voxel::block::id::StateId};

use super::SectionIndex;

pub struct BlockSection<const W: i32> {
    blocks: Option<Box<[StateId]>>,
    /// Keeps track of how many non-air [StateId]s are in the section.
    /// Once this value becomes 0, the `blocks` field is dropped.
    non_air_count: u16,
}

impl<const W: i32> BlockSection<W> {
    const BLOCK_COUNT: usize = (W as usize).pow(3);
    pub const fn new() -> Self {
        Self {
            blocks: None,
            non_air_count: 0,
        }
    }

    /// Shorthand for `self.set(coord, StateId::AIR)`.
    #[inline]
    pub fn delete<I: SectionIndex<W>>(&mut self, coord: I) -> Change<StateId> {
        self.set(coord, StateId::AIR)
    }

    pub fn set<I: SectionIndex<W>>(&mut self, coord: I, id: StateId) -> Change<StateId> {
        if self.blocks.is_none() && id.is_air() {
            return Change::Unchanged;
        }
        let blocks = self.blocks.get_or_insert_with(|| (0..Self::BLOCK_COUNT).map(|_| StateId::AIR).collect());
        let index = coord.section_index();
        let old = blocks[index].replace(id);
        if old == id {
            Change::Unchanged
        } else {
            if id == StateId::AIR {
                self.non_air_count -= 1;
                if self.non_air_count == 0 {
                    self.blocks.drop();
                }
            } else if old == StateId::AIR {
                self.non_air_count += 1;
            }
            Change::Changed(old)
        }
    }

    pub fn get<I: SectionIndex<W>>(&mut self, coord: I) -> StateId {
        let Some(blocks) = &self.blocks else {
            return StateId::AIR;
        };
        let index = coord.section_index();
        blocks[index]
    }

    pub fn is_allocated(&self) -> bool {
        self.blocks.is_some()
    }
}