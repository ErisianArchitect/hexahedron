use crate::{prelude::{Direction, OptionExtension, Replace}, util::change::Change};

use super::{occlusion::Occlusion, SectionIndex};

pub struct OcclusionSection<const W: i32> {
    occlusion_data: Option<Box<[Occlusion]>>,
    occluded_count: u16,
}

impl<const W: i32> OcclusionSection<W> {
    const BLOCK_COUNT: usize = (W as usize).pow(3);
    pub const fn new() -> Self {
        Self {
            occlusion_data: None,
            occluded_count: 0,
        }
    }

    pub fn show_face<I: SectionIndex<W>>(&mut self, coord: I, face: Direction) -> Change<bool> {
        // Faces are shown by default, so we should return Unchanged
        // if the occlusion data is unallocated.
        let Some(occlusion_data) = self.occlusion_data.as_mut() else {
            return Change::Unchanged;
        };
        let index = coord.section_index();
        let mut occ = occlusion_data[index];
        let old = occ.show(face);
        if old {
            if occ.is_fully_unoccluded() {
                self.occluded_count -= 1;
                if self.occluded_count == 0 {
                    self.occlusion_data.drop();
                    return Change::Changed(old);
                }
            }
            occlusion_data[index] = occ;
            Change::Changed(old)
        } else {
            Change::Unchanged
        }
    }

    pub fn hide_face<I: SectionIndex<W>>(&mut self, coord: I, face: Direction) -> Change<bool> {
        let occlusion_data = self.occlusion_data.get_or_insert_with(|| (0..Self::BLOCK_COUNT).map(|_| Occlusion::UNOCCLUDED).collect());
        let index = coord.section_index();
        let mut occ = occlusion_data[index];
        let was_unoccluded = occ.is_fully_unoccluded();
        let old = occ.hide(face);
        if !old {
            if was_unoccluded {
                self.occluded_count += 1;
            }
            occlusion_data[index] = occ;
            Change::Changed(old)
        } else {
            Change::Unchanged
        }
    }

    pub fn show_all_faces<I: SectionIndex<W>>(&mut self, coord: I) -> Change<Occlusion> {
        let Some(occlusion_data) = self.occlusion_data.as_mut() else {
            return Change::Unchanged;
        };
        let index = coord.section_index();
        let old = occlusion_data[index].replace(Occlusion::UNOCCLUDED);
        if old != Occlusion::UNOCCLUDED {
            self.occluded_count -= 1;
            if self.occluded_count == 0 {
                self.occlusion_data.drop();
                return Change::Changed(old);
            }
            Change::Changed(old)
        } else {
            Change::Unchanged
        }
    }

    pub fn hide_all_faces<I: SectionIndex<W>>(&mut self, coord: I) -> Change<Occlusion> {
        let occlusion_data = self.occlusion_data.get_or_insert_with(|| (0..Self::BLOCK_COUNT).map(|_| Occlusion::UNOCCLUDED).collect());
        let index = coord.section_index();
        let old = occlusion_data[index].replace(Occlusion::OCCLUDED);
        match old {
            Occlusion::OCCLUDED => Change::Unchanged,
            Occlusion::UNOCCLUDED => {
                self.occluded_count += 1;
                Change::Changed(old)
            },
            old => Change::Changed(old),
        }
    }

    pub fn get<I: SectionIndex<W>>(&self, coord: I) -> Occlusion {
        let Some(occlusion_data) = self.occlusion_data.as_ref() else {
            return Occlusion::UNOCCLUDED;
        };
        let index = coord.section_index();
        occlusion_data[index]
    }

    pub fn set<I: SectionIndex<W>>(&mut self, coord: I, occlusion: Occlusion) -> Change<Occlusion> {
        if self.occlusion_data.is_none() && occlusion.is_fully_unoccluded() {
            return Change::Unchanged;
        }
        let occlusion_data = self.occlusion_data.get_or_insert_with(|| (0..Self::BLOCK_COUNT).map(|_| Occlusion::UNOCCLUDED).collect());
        let index = coord.section_index();
        let old = occlusion_data[index].replace(occlusion);
        match (old.is_fully_unoccluded(), occlusion.is_fully_unoccluded()) {
            (true, true) => Change::Unchanged,
            (false, true) => {
                self.occluded_count -= 1;
                if self.occluded_count == 0 {
                    self.occlusion_data.drop();
                }
                Change::Changed(old)
            }
            (true, false) => {
                self.occluded_count += 1;
                Change::Changed(old)
            }
            (false, false) => if old != occlusion {
                Change::Changed(old)
            } else {
                Change::Unchanged
            }
        }
    }

    pub fn is_allocated(&self) -> bool {
        self.occlusion_data.is_some()
    }
}