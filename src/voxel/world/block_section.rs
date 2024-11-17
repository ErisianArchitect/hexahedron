use crate::{math::index3, prelude::{OptionExtension, Replace}, util::change::Change, voxel::block::id::StateId};

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
    pub fn delete<C: Into<(i32, i32, i32)>>(&mut self, coord: C) -> Change<StateId> {
        self.set(coord, StateId::AIR)
    }

    pub fn set<C: Into<(i32, i32, i32)>>(&mut self, coord: C, id: StateId) -> Change<StateId> {
        let (x, y, z): (i32, i32, i32) = coord.into();
        if self.blocks.is_none() && id.is_air() {
            return Change::Unchanged;
        }
        let blocks = self.blocks.get_or_insert_with(|| (0..Self::BLOCK_COUNT).map(|_| StateId::AIR).collect());
        let index = index3::<W>(x, y, z);
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

    pub fn get<C: Into<(i32, i32, i32)>>(&mut self, coord: C) -> StateId {
        let (x, y, z): (i32, i32, i32) = coord.into();
        let Some(blocks) = &self.blocks else {
            return StateId::AIR;
        };
        let index = index3::<W>(x, y, z);
        blocks[index]
    }

    pub fn is_allocated(&self) -> bool {
        self.blocks.is_some()
    }
}