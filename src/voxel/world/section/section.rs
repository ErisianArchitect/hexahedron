use super::block_section::BlockSection;
use super::light_section::LightSection;
use super::occlusion_section::OcclusionSection;
use super::tag_section::TagSection;
use super::update_section::UpdateSection;

/// Holds all the block data within a 32x32x32 volume.
pub struct Section<const WIDTH: i32> {
    pub blocks: BlockSection<WIDTH>,
    pub block_light: LightSection<WIDTH, 0>,
    pub sky_light: LightSection<WIDTH, 15>,
    pub tags: TagSection<WIDTH>,
    pub occlusion_data: OcclusionSection<WIDTH>,
    pub update_ids: UpdateSection<WIDTH>,
}

impl<const WIDTH: i32> Section<WIDTH> {
    /// The size of the Width, Height, and Depth.
    pub const SIZE: i32 = WIDTH;
    /// This is the number of blocks that exist in a [Section].  
    /// This value is dependent on [Section]`::SIZE` (SIZE*SIZE*SIZE).
    pub const BLOCK_COUNT: usize = (Self::SIZE as usize).pow(3);

    #[inline]
    pub const fn new() -> Self {
        Self {
            blocks: BlockSection::new(),
            block_light: LightSection::new(),
            sky_light: LightSection::new(),
            tags: TagSection::new(),
            occlusion_data: OcclusionSection::new(),
            update_ids: UpdateSection::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{blockstate, voxel::block::{block::BlockBehavior, block_registry::BlockRegistry, id::StateId}};

    use super::*;
    #[allow(unused)]
    #[test]
    fn section_test() {
        let reg = BlockRegistry::new();
        struct DebugBlock(&'static str);
        impl BlockBehavior for DebugBlock {
            fn name(&self) -> &str {
                self.0
            }
        }
        reg.register_block(DebugBlock("dirt")).unwrap();
        reg.register_block(DebugBlock("stone")).unwrap();
        reg.register_block(DebugBlock("sand")).unwrap();
        reg.register_block(DebugBlock("grass")).unwrap();
        let dirt = reg.register_state(blockstate!(dirt)).unwrap();
        let stone = reg.register_state(blockstate!(stone)).unwrap();
        let sand = reg.register_state(blockstate!(sand)).unwrap();
        let grass = reg.register_state(blockstate!(grass)).unwrap();
        let mut section = Section::<8>::new();
        assert!(!section.blocks.is_allocated());
        assert!(!section.occlusion_data.is_allocated());
        assert!(!section.block_light.is_allocated());
        assert!(!section.sky_light.is_allocated());
        assert!(!section.tags.is_allocated());
        assert!(!section.update_ids.is_allocated());
        section.blocks.set((7, 7, 7), dirt);
        assert!(section.blocks.is_allocated());
        section.blocks.set((7, 7, 7), StateId::AIR);
        assert!(!section.blocks.is_allocated());
    }
}