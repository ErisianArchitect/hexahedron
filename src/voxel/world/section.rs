// use crate::voxel::block::id::StateId;
use super::block_section::BlockSection;
use super::light_section::LightSection;
use super::tag_section::TagSection;

/// Holds all the block data within a 32x32x32 volume.
pub struct Section {
    pub blocks: BlockSection,
    pub block_light: LightSection<0>,
    pub sky_light: LightSection<15>,
    pub tags: TagSection,
    // TODO:
    // UpdateSection
    // OcclusionSection
    // 
}