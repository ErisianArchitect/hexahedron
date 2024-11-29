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
    use glam::IVec3;

    use crate::{blockstate, collections::update_queue::{UpdateId, UpdateQueue}, tag::Tag, util::change::Change, voxel::{block::{block::BlockBehavior, block_registry::BlockRegistry, id::StateId}, world::section::occlusion::Occlusion}};

    use super::*;
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
        let dirt = reg.register_state(blockstate!(dirt)).unwrap();
        let mut section = Section::<8>::new();
        let mut update_queue = UpdateQueue::new();

        assert!(!section.blocks.is_allocated());
        assert!(!section.occlusion_data.is_allocated());
        assert!(!section.block_light.is_allocated());
        assert!(!section.sky_light.is_allocated());
        assert!(!section.tags.is_allocated());
        assert!(!section.update_ids.is_allocated());
        assert!(update_queue.is_empty());

        let c = (7, 7, 7);
        assert_eq!(
            section.blocks.set(c, dirt),
            Change::Changed(StateId::AIR)
        );
        assert_eq!(
            section.occlusion_data.set(c, Occlusion::OCCLUDED),
            Change::Changed(Occlusion::UNOCCLUDED)
        );
        assert_eq!(
            section.block_light.set(c, 15),
            Change::Changed(0)
        );
        assert_eq!(
            section.sky_light.set(c, 0),
            Change::Changed(15)
        );
        assert_eq!(
            section.tags.insert(c, Tag::from("Hello, world!")),
            None
        );
        assert_eq!(
            section.update_ids.set(c, update_queue.insert(IVec3::new(c.0, c.1, c.2))),
            Change::Changed(UpdateId::NULL)
        );

        assert!(section.blocks.is_allocated());
        assert!(section.occlusion_data.is_allocated());
        assert!(section.block_light.is_allocated());
        assert!(section.sky_light.is_allocated());
        assert!(section.tags.is_allocated());
        assert!(section.update_ids.is_allocated());
        assert!(!update_queue.is_empty());

        assert_eq!(
            section.blocks.set(c, StateId::AIR),
            Change::Changed(dirt)
        );
        assert_eq!(
            section.occlusion_data.set(c, Occlusion::UNOCCLUDED),
            Change::Changed(Occlusion::OCCLUDED)
        );
        assert_eq!(
            section.block_light.set(c, 0),
            Change::Changed(15)
        );
        assert_eq!(
            section.sky_light.set(c, 15),
            Change::Changed(0)
        );
        section.tags.remove(c).and_then(|tag| {
            assert_eq!(tag, Tag::from("Hello, world!"));
            Some(tag)
        }).or_else(|| {
            panic!("Did not match tag.");
        });
        let mut changed = false;
        section.update_ids.set(c, UpdateId::NULL).if_changed(|id| {
            changed = true;
            update_queue.remove(id);
        });
        assert!(changed);

        assert!(!section.blocks.is_allocated());
        assert!(!section.occlusion_data.is_allocated());
        assert!(!section.block_light.is_allocated());
        assert!(!section.sky_light.is_allocated());
        assert!(!section.tags.is_allocated());
        assert!(!section.update_ids.is_allocated());
        assert!(update_queue.is_empty());
    }
}