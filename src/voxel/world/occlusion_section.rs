use crate::{math::index3, prelude::{Direction, OptionExtension, Replace}, util::change::Change};

use super::occlusion::Occlusion;

pub struct OcclusionSection {
    occlusion_data: Option<Box<[Occlusion]>>,
    occluded_count: u16,
}

impl OcclusionSection {
    pub const fn new() -> Self {
        Self {
            occlusion_data: None,
            occluded_count: 0,
        }
    }

    pub fn show_face<C: Into<(i32, i32, i32)>>(&mut self, coord: C, face: Direction) -> Change<bool> {
        // Faces are shown by default, so we should return Unchanged
        // if the occlusion data is unallocated.
        let Some(occlusion_data) = self.occlusion_data.as_mut() else {
            return Change::Unchanged;
        };
        let (x, y, z) = coord.into();
        let index = index3::<32>(x, y, z);
        let mut occ = occlusion_data[index];
        let old = occ.show(face);
        if old {
            if occ.0 == 0 {
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

    pub fn hide_face<C: Into<(i32, i32, i32)>>(&mut self, coord: C, face: Direction) -> Change<bool> {
        let occlusion_data = self.occlusion_data.get_or_insert_with(|| (0..32768).map(|_| Occlusion::UNOCCLUDED).collect());
        let (x, y, z) = coord.into();
        let index = index3::<32>(x, y, z);
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

    pub fn show_all_faces<C: Into<(i32, i32, i32)>>(&mut self, coord: C) -> Change<Occlusion> {
        let Some(occlusion_data) = self.occlusion_data.as_mut() else {
            return Change::Unchanged;
        };
        let (x, y, z) = coord.into();
        let index = index3::<32>(x, y, z);
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

    pub fn hide_all_faces<C: Into<(i32, i32, i32)>>(&mut self, coord: C) -> Change<Occlusion> {
        let occlusion_data = self.occlusion_data.get_or_insert_with(|| (0..32768).map(|_| Occlusion::UNOCCLUDED).collect());
        let (x, y, z) = coord.into();
        let index = index3::<32>(x, y, z);
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
}