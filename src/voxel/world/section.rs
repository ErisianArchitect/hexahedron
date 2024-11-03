use crate::{collections::tagcontainer::{TagContainer, TagId, TagSection}, voxel::block::id::StateId};

// Holds all the block data within a 32x32x32 volume.
pub struct Section {
    pub blocks: Option<Box<[StateId]>>,
    pub block_light: Option<Box<[u8]>>,
    pub sky_light: Option<Box<[u8]>>,
    pub tags: TagSection,
}