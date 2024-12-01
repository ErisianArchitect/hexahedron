use serde::{Serialize, Deserialize};
use crate::voxel::{
    orientation::{
        Flip,
        Rotation,
        orient_table,
        pack_flip_and_rotation,
    },
    direction::Direction,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Orientation(pub(crate) u8);

impl Orientation {
    pub const UNORIENTED: Orientation = Orientation::new(Rotation::new(Direction::PosY, 0), Flip::NONE);
    pub const ROTATE_X: Orientation = Rotation::ROTATE_X.orientation();
    pub const X_ROTATIONS: [Orientation; 4] = Self::ROTATE_X.angles();
    pub const ROTATE_Y: Orientation = Rotation::ROTATE_Y.orientation();
    pub const Y_ROTATIONS: [Orientation; 4] = Self::ROTATE_Y.angles();
    pub const ROTATE_Z: Orientation = Rotation::ROTATE_Z.orientation();
    pub const Z_ROTATIONS: [Orientation; 4] = Self::ROTATE_Z.angles();

    pub const CORNER_ORIENTATIONS_MATRIX: [[[[Orientation; 3]; 2]; 2]; 2] = [
        [
            [Rotation::new(Direction::PosZ, 3).orientation().corner_angles(), Rotation::new(Direction::NegX, 2).orientation().corner_angles()],
            [Rotation::new(Direction::PosX, 0).orientation().corner_angles(), Rotation::new(Direction::NegZ, 1).orientation().corner_angles()]
        ],
        [
            [Rotation::new(Direction::NegX, 0).orientation().corner_angles(), Rotation::new(Direction::NegZ, 3).orientation().corner_angles()],
            [Rotation::new(Direction::PosZ, 1).orientation().corner_angles(), Rotation::new(Direction::PosX, 2).orientation().corner_angles()]
        ],
    ];

    pub const FACE_ORIENTATIONS: [[Orientation; 4]; 6] = [
        Rotation::new(Direction::PosY, 1).orientation().angles(), // PosY
        Rotation::new(Direction::NegZ, 2).orientation().angles(), // PosX
        Rotation::new(Direction::PosX, 1).orientation().angles(), // PosZ
        Rotation::new(Direction::PosY, 3).orientation().angles(), // NegY
        Rotation::new(Direction::PosZ, 0).orientation().angles(), // NegX
        Rotation::new(Direction::NegX, 3).orientation().angles(), // NegZ
    ];
    
    /// An orientation that you can orient an orientation by to rotate around a face by angle. That was a mouthful.  
    /// If angle == 0, orientation is default orientation.
    #[inline]
    pub const fn face_orientation(face: Direction, angle: i32) -> Self {
        Self::FACE_ORIENTATIONS[face as usize][(angle & 3) as usize]
    }

    pub const fn corner_orientation(x: i32, y: i32, z: i32, angle: i32) -> Orientation {
        let x = if x <= 0 {
            0
        } else {
            1
        } as usize;
        let y = if y <= 0 {
            0
        } else {
            1
        } as usize;
        let z = if z <= 0 {
            0
        } else {
            1
        } as usize;
        let angle = angle.rem_euclid(3) as usize;
        Self::CORNER_ORIENTATIONS_MATRIX[y][z][x][angle]
    }

    #[inline]
    pub const fn new(rotation: Rotation, flip: Flip) -> Self {
        Self(pack_flip_and_rotation(flip, rotation))
    }

    /// A helper function to create 4 orientations for an orientation group.  
    /// An orientation group is a series of "contiguous" orientations. That is, the orientations are logically sequential.
    /// An example would be rotations around an axis, or around a face, where there are 4 orientations possible.
    /// The first orientation is unoriented, the second orientation is the target orientation
    /// applied once, the third orientation is the target orientation applied twice,
    /// and the fourth orientation is the target orientation applied three times.
    pub const fn angles(self) -> [Orientation; 4] {
        let angle1 = self;
        let angle2 = angle1.reorient(angle1);
        let angle3 = angle2.reorient(angle1);
        [
            Orientation::UNORIENTED,
            angle1,
            angle2,
            angle3,
        ]
    }

    /// A helper function to create 3 orientations for a corner orientation group.
    /// The first orientation is unoriented, the second orientation is the target orientation,
    /// and the third orientation is the target orientation applied to itself.
    pub const fn corner_angles(self) -> [Orientation; 3] {
        let angle1 = self;
        let angle2 = angle1.reorient(angle1);
        [
            Orientation::UNORIENTED,
            angle1,
            angle2,
        ]
    }

    #[inline]
    pub const fn flip(self) -> Flip {
        Flip(self.0 & 0b111)
    }

    #[inline]
    pub const fn rotation(self) -> Rotation {
        Rotation(self.0 >> 3)
    }

    #[inline]
    pub fn set_flip(&mut self, flip: Flip) {
        self.0 = (self.0 & 0b11111000) | flip.0
    }

    #[inline]
    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.0 = (self.0 & 0b111) | rotation.0 << 3;
    }

    #[inline]
    pub fn set_up(&mut self, up: Direction) {
        let mut rot = self.rotation();
        rot.set_up(up);
        self.set_rotation(rot);
    }

    #[inline]
    pub fn set_angle(&mut self, angle: i32) {
        let mut rot = self.rotation();
        rot.set_angle(angle);
        self.set_rotation(rot);
    }

    /// `reface` can be used to determine where a face will end up after orientation.
    /// First rotates and then flips the face.
    #[inline]
    pub const fn reface(self, face: Direction) -> Direction {
        let rotated = self.rotation().reface(face);
        rotated.flip(self.flip())
    }

    /// This determines which face was oriented to `face`.
    #[inline]
    pub const fn source_face(self, face: Direction) -> Direction {
        let flipped = face.flip(self.flip());
        self.rotation().source_face(flipped)
    }

    /// Gets the direction that [Direction::PosY] is pointing towards.
    #[inline]
    pub const fn up(self) -> Direction {
        self.reface(Direction::PosY)
    }

    /// Gets the direction that [Direction::NegY] is pointing towards.
    #[inline]
    pub const fn down(self) -> Direction {
        self.reface(Direction::NegY)
    }

    /// Gets the direction that [Direction::NegZ] is pointing towards.
    #[inline]
    pub const fn forward(self) -> Direction {
        self.reface(Direction::NegZ)
    }

    /// Gets the direction that [Direction::PosZ] is pointing towards.
    #[inline]
    pub const fn backward(self) -> Direction {
        self.reface(Direction::PosZ)
    }

    /// Gets the direction that [Direction::NegX] is pointing towards.
    #[inline]
    pub const fn left(self) -> Direction {
        self.reface(Direction::NegX)
    }

    /// Gets the direction that [Direction::PosX] is pointing towards.
    #[inline]
    pub const fn right(self) -> Direction {
        self.reface(Direction::PosX)
    }

    /// If you're using this method to transform mesh vertices, make sure that you 
    /// reverse your indices if the face will be flipped (for backface culling). To
    /// determine if your indices need to be inverted, simply XOR each axis of the [Orientation]'s [Flip].
    /// This method will rotate and then flip the coordinate.
    #[inline]
    pub fn transform<T: Copy + std::ops::Neg<Output = T>, C: Into<(T, T, T)> + From<(T, T, T)>>(self, point: C) -> C {
        let rotated = self.rotation().rotate_coord(point);
        self.flip().flip_coord(rotated)
    }

    /// This method can tell you where on the target face a source UV is.
    /// To get the most benefit out of this, it is advised that you center your coords around (0, 0).
    /// So if you're trying to map a coord within a rect of size (16, 16), you would subtract 8 from the
    /// x and y of the coord, then pass that offset coord to this function, then add 8 back to the x and y
    /// to get your final coord.
    #[inline]
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
    #[inline]
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
    #[inline]
    pub const fn invert(self) -> Self {
        Orientation::UNORIENTED.deorient(self)
    }

    #[inline]
    pub const fn flip_x(self) -> Self {
        Orientation::new(self.rotation(), self.flip().flip_x())
    }

    #[inline]
    pub const fn flip_y(self) -> Self {
        Orientation::new(self.rotation(), self.flip().flip_y())
    }

    #[inline]
    pub const fn flip_z(self) -> Self {
        Orientation::new(self.rotation(), self.flip().flip_z())
    }

    #[inline]
    pub const fn rotate_x(self, angle: i32) -> Self {
        self.reorient(Orientation::X_ROTATIONS[(angle & 3) as usize])
    }

    #[inline]
    pub const fn rotate_y(self, angle: i32) -> Self {
        self.reorient(Orientation::Y_ROTATIONS[(angle & 3) as usize])
    }

    #[inline]
    pub const fn rotate_z(self, angle: i32) -> Self {
        self.reorient(Orientation::Z_ROTATIONS[(angle & 3) as usize])
    }

    /// Rotate `face` clockwise by `angle`. Use a negative `angle` to rotate counter-clockwise.
    #[inline]
    pub const fn rotate_face(self, face: Direction, angle: i32) -> Self {
        let orient = Self::face_orientation(face, angle);
        self.reorient(orient)
    }

    #[inline]
    pub const fn rotate_corner(self, x: i32, y: i32, z: i32, angle: i32) -> Self {
        let orient = Self::corner_orientation(x, y, z, angle);
        self.reorient(orient)
    }
}

impl From<u8> for Orientation {
    #[inline]
    fn from(value: u8) -> Self {
        Orientation(value)
    }
}

impl Into<u8> for Orientation {
    #[inline]
    fn into(self) -> u8 {
        self.0
    }
}

impl From<Rotation> for Orientation {
    #[inline]
    fn from(value: Rotation) -> Self {
        Orientation::new(value, Flip::NONE)
    }
}

impl From<Flip> for Orientation {
    #[inline]
    fn from(value: Flip) -> Self {
        Orientation::new(Rotation::default(), value)
    }
}

impl std::fmt::Display for Orientation {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Orientation({},{})", self.flip(), self.rotation())
    }
}