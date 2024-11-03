use glam::IVec3;

use crate::math::index3;

#[derive(Debug, Default, Clone)]
pub struct LightSection<const DEFAULT: u8 = 0> {
    light_data: Option<Box<[u8]>>,
    instance_count: u16,
}

impl<const DEFAULT: u8> LightSection<DEFAULT> {
    pub fn new() -> Self {
        Self {
            light_data: None,
            instance_count: 0,
        }
    }

    pub fn get<C: Into<IVec3>>(&self, coord: C) -> u8 {
        let coord: IVec3 = coord.into();
        self.light_data.as_ref().and_then(|data| {
            let index = index3::<32>(coord.x, coord.y, coord.z);
            let subindex = index / 2;
            let lights = data[subindex];
            if (index & 1) == 1 {
                Some(lights >> 4)
            } else {
                Some(lights & 0xf)
            }
        }).unwrap_or(DEFAULT)
    }

    pub fn set<C: Into<IVec3>>(&mut self, coord: C, level: u8) -> u8 {
        let coord: IVec3 = coord.into();
        if self.light_data.is_none() && level == DEFAULT {
            return DEFAULT;
        }
        let data = self.light_data.get_or_insert_with(|| (0..32768).map(|_| DEFAULT).collect());
        let index = index3::<32>(coord.x, coord.y, coord.z);
        let subindex = index / 2;
        let lights = data[subindex];
        let (injected, old) = if (index & 1) == 1 {
            (
                (lights & 0xf) | (level << 4),
                lights >> 4
            )
        } else {
            (
                (lights & 0xf0) | (level & 0xf),
                lights & 0xf
            )
        };
        data[subindex] = injected;
        if level != old {
            if level == DEFAULT {
                self.instance_count -= 1;
                if self.instance_count == 0 {
                    self.light_data.take();
                }
            } else {
                self.instance_count += 1;
            }
        }
        old
    }

    pub fn clear(&mut self) {
        self.light_data.take();
        self.instance_count = 0;
    }
}