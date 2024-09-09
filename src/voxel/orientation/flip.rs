use crate::prelude::Direction;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Flip(pub u8);

impl Flip {
    pub const X: Flip = Flip(0b001);
    pub const XY: Flip = Flip(0b011);
    pub const XZ: Flip = Flip(0b101);
    pub const Y: Flip = Flip(0b010);
    pub const YZ: Flip = Flip(0b110);
    pub const Z: Flip = Flip(0b100);
    pub const XYZ: Flip = Flip(0b111);
    pub const ALL: Flip = Flip::XYZ;
    pub const NONE: Flip = Flip(0b000);

    pub const fn new(x: bool, y: bool, z: bool) -> Self {
        Self((x as u8) | ((y as u8) << 1) | ((z as u8) << 2))
    }
    
    pub const fn x(self) -> bool {
        self.0 & Flip::X.0 == Flip::X.0
    }

    pub const fn y(self) -> bool {
        self.0 & Flip::Y.0 == Flip::Y.0
    }

    pub const fn z(self) -> bool {
        self.0 & Flip::Z.0 == Flip::Z.0
    }
}