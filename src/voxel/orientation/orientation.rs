use crate::voxel::{
    orientation::{
        Flip,
        Rotation,
        orient_table,
        pack_flip_and_rotation,
    },
    direction::Direction,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Orientation(u8);

impl Orientation {
    pub const UNORIENTED: Orientation = Orientation::new(Rotation::new(Direction::PosY, 0), Flip::NONE);
    pub const ROTATE_X: Rotation = Rotation::new(Direction::NegZ, 2);
    pub const X_ROTATIONS: [Rotation; 4] = [
        Rotation::new(Direction::PosY, 0),
        Rotation::new(Direction::NegZ, 2),
        Rotation::new(Direction::NegY, 0),
        Rotation::new(Direction::PosZ, 0),
    ];
    pub const ROTATE_Y: Rotation = Rotation::new(Direction::PosY, 1);
    pub const Y_ROTATIONS: [Rotation; 4] = [
        Rotation::new(Direction::PosY, 0),
        Rotation::new(Direction::PosY, 1),
        Rotation::new(Direction::PosY, 2),
        Rotation::new(Direction::PosY, 3),
    ];
    pub const ROTATE_Z: Rotation = Rotation::new(Direction::PosX, 1);
    pub const Z_ROTATIONS: [Rotation; 4] = [
        Rotation::new(Direction::PosY, 0),
        Rotation::new(Direction::PosX, 1),
        Rotation::new(Direction::NegY, 2),
        Rotation::new(Direction::NegX, 3),
    ];
    pub const CORNER_ROTATIONS_MATRIX: [[[Rotation; 2]; 2]; 2] = [
        [
            [Rotation::new(Direction::PosZ, 3), Rotation::new(Direction::NegX, 2)],
            [Rotation::new(Direction::PosX, 0), Rotation::new(Direction::NegZ, 1)]
        ],
        [
            [Rotation::new(Direction::NegX, 0), Rotation::new(Direction::NegZ, 3)],
            [Rotation::new(Direction::PosZ, 1), Rotation::new(Direction::PosX, 2)]
        ],
    ];

    pub const fn new(rotation: Rotation, flip: Flip) -> Self {
        Self(pack_flip_and_rotation(flip, rotation))
    }

    pub const fn flip(self) -> Flip {
        Flip(self.0 & 0b111)
    }

    pub fn set_flip(&mut self, flip: Flip) {
        self.0 = (self.0 & 0b11111000) | flip.0
    }

    pub const fn rotation(self) -> Rotation {
        Rotation(self.0 >> 3)
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.0 = (self.0 & 0b111) | rotation.0 << 3;
    }

    /// `reface` can be used to determine where a face will end up after orientation.
    /// First rotates and then flips the face.
    pub const fn reface(self, face: Direction) -> Direction {
        let rotated = self.rotation().reface(face);
        rotated.flip(self.flip())
    }

    /// This determines which face was oriented to `face`. I hope that makes sense.
    pub const fn source_face(self, face: Direction) -> Direction {
        let flipped = face.flip(self.flip());
        self.rotation().source_face(flipped)
    }

    /// Gets the direction that [Direction::PosY] is pointing towards.
    pub const fn up(self) -> Direction {
        self.reface(Direction::PosY)
    }

    /// Gets the direction that [Direction::NegY] is pointing towards.
    pub const fn down(self) -> Direction {
        self.reface(Direction::NegY)
    }

    /// Gets the direction that [Direction::NegZ] is pointing towards.
    pub const fn forward(self) -> Direction {
        self.reface(Direction::NegZ)
    }

    /// Gets the direction that [Direction::PosZ] is pointing towards.
    pub const fn backward(self) -> Direction {
        self.reface(Direction::PosZ)
    }

    /// Gets the direction that [Direction::NegX] is pointing towards.
    pub const fn left(self) -> Direction {
        self.reface(Direction::NegX)
    }

    /// Gets the direction that [Direction::PosX] is pointing towards.
    pub const fn right(self) -> Direction {
        self.reface(Direction::PosX)
    }

    /// If you're using this method to transform mesh vertices, make sure that you 
    /// reverse your indices if the face will be flipped (for backface culling). To
    /// determine if your indices need to be inverted, simply XOR each axis of the [Orientation]'s [Flip].
    /// This method will rotate and then flip the coordinate.
    pub fn transform<T: Copy + std::ops::Neg<Output = T>, C: Into<(T, T, T)> + From<(T, T, T)>>(self, point: C) -> C {
        let rotated = self.rotation().rotate_coord(point);
        self.flip().flip_coord(rotated)
    }

    /// This method can tell you where on the target face a source UV is.
    /// To get the most benefit out of this, it is advised that you center your coords around (0, 0).
    /// So if you're trying to map a coord within a rect of size (16, 16), you would subtract 8 from the
    /// x and y of the coord, then pass that offset coord to this function, then add 8 back to the x and y
    /// to get your final coord.
    pub fn map_face_coord<T: Copy + std::ops::Neg<Output = T>, C: Into<(T, T)> + From<(T, T)>>(self, face: Direction, uv: C) -> C {
        let table_index = orient_table::table_index(self.rotation(), self.flip(), face);
        let coordmap = orient_table::MAP_COORD_TABLE[table_index];
        coordmap.map(uv)
    }

    /// This method can tell you where on the source face a target UV is.
    /// To get the most benefit out of this, it is advised that you center your coords around (0, 0).
    /// So if you're trying to map a coord within a rect of size (16, 16), you would subtract 8 from the
    /// x and y of the coord, then pass that offset coord to this function, then add 8 back to the x and y
    /// to get your final coord.
    pub fn source_face_coord<T: Copy + std::ops::Neg<Output = T>, C: Into<(T, T)> + From<(T, T)>>(self, face: Direction, uv: C) -> C {
        let table_index = orient_table::table_index(self.rotation(), self.flip(), face);
        let coordmap = orient_table::SOURCE_FACE_COORD_TABLE[table_index];
        coordmap.map(uv)
    }

    /// Apply an orientation to an orientation.
    pub const fn reorient(self, orientation: Orientation) -> Self {
        let up = self.up();
        let fwd = self.forward();
        let reup = orientation.reface(up);
        let refwd = orientation.reface(fwd);
        let flip = self.flip().flip(orientation.flip());
        let flipup = reup.flip(flip);
        let flipfwd = refwd.flip(flip);
        let Some(rot) = Rotation::from_up_and_forward(flipup, flipfwd) else {
            unreachable!()
        };
        Orientation::new(rot, flip)
    }

    /// Remove an orientation from an orientation.
    /// This is the inverse operation to [Orientation::reorient].
    pub const fn deorient(self, orientation: Orientation) -> Self {
        let up = self.up();
        let fwd = self.forward();
        let reup = orientation.source_face(up);
        let refwd = orientation.source_face(fwd);
        let flip = self.flip().flip(orientation.flip());
        let flipup = reup.flip(flip);
        let flipfwd = refwd.flip(flip);
        let Some(rot) = Rotation::from_up_and_forward(flipup, flipfwd) else {
            unreachable!()
        };
        Orientation::new(rot, flip)
    }
    
    /// Returns the orientation that can be applied to deorient by [self].
    pub const fn invert(self) -> Self {
        Orientation::UNORIENTED.deorient(self)
    }

    pub const fn flip_x(self) -> Self {
        Orientation::new(self.rotation(), self.flip().flip_x())
    }

    pub const fn flip_y(self) -> Self {
        Orientation::new(self.rotation(), self.flip().flip_y())
    }

    pub const fn flip_z(self) -> Self {
        Orientation::new(self.rotation(), self.flip().flip_z())
    }

    pub const fn rotate_x(self, angle: i32) -> Self {
        self.reorient(Orientation::new(
            Orientation::X_ROTATIONS[angle.rem_euclid(4) as usize],
            Flip::NONE
        ))
    }

    pub const fn rotate_y(self, angle: i32) -> Self {
        self.reorient(Orientation::new(
            Orientation::Y_ROTATIONS[angle.rem_euclid(4) as usize],
            Flip::NONE
        ))
    }

    pub const fn rotate_z(self, angle: i32) -> Self {
        self.reorient(Orientation::new(
            Orientation::Z_ROTATIONS[angle.rem_euclid(4) as usize],
            Flip::NONE
        ))
    }
}

impl From<Rotation> for Orientation {
    fn from(value: Rotation) -> Self {
        Orientation::new(value, Flip::NONE)
    }
}

impl From<Flip> for Orientation {
    fn from(value: Flip) -> Self {
        Orientation::new(Rotation::default(), value)
    }
}

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Orientation({},{})", self.flip(), self.rotation())
    }
}