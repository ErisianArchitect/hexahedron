use bytemuck::NoUninit;
use glam::{IVec3, Vec3};
use serde::{Serialize, Deserialize};

use crate::{
    axis::Axis,
    flip::Flip,
    rotation::Rotation,
};

// The ids are out of order so that they can have a certain order for orientations.
// If you change the discriminants, then some code might break.
/// Represents each direction of a cube face.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, NoUninit, Serialize, Deserialize)]
pub enum Direction {
    /// Left
    NegX = 4,
    /// Down
    NegY = 3,
    /// Forward
    NegZ = 5,
    /// Right
    PosX = 1,
    /// Up
    PosY = 0,
    /// Back
    PosZ = 2,
}

impl Direction {
    /// All directions, ordered logically (`NegX`, `NegY`, `NegZ`, `PosX`, `PosY`, `PosZ`).
    pub const ALL: [Direction; 6] = [
        Direction::NegX,
        Direction::NegY,
        Direction::NegZ,
        Direction::PosX,
        Direction::PosY,
        Direction::PosZ
    ];

    /// All directions, ordered by discriminant.
    pub const INDEX_ORDER: [Direction; 6] = [
        Direction::PosY,
        Direction::PosX,
        Direction::PosZ,
        Direction::NegY,
        Direction::NegX,
        Direction::NegZ,
    ];

    /// All directions, ordered for a flood fill algorithm.
    /// ```no_run
    /// [PosY, NegY, PosX, NegX, PosZ, NegZ]
    /// ```
    pub const FLOOD: [Direction; 6] = [
        Direction::PosY,
        Direction::NegY,
        Direction::PosX,
        Direction::NegX,
        Direction::PosZ,
        Direction::NegZ,
    ];

    pub const LEFT: Direction = Direction::NegX;
    pub const DOWN: Direction = Direction::NegY;
    pub const FORWARD: Direction = Direction::NegZ;
    pub const RIGHT: Direction = Direction::PosX;
    pub const UP: Direction = Direction::PosY;
    pub const BACKWARD: Direction = Direction::PosZ;

    /// Invert the [Direction]. (`NegX` becomes `PosX`, `PosX` becomes `NegX`, etc.)
    #[inline]
    pub const fn invert(self) -> Self {
        match self {
            Direction::NegX => Direction::PosX,
            Direction::NegY => Direction::PosY,
            Direction::NegZ => Direction::PosZ,
            Direction::PosX => Direction::NegX,
            Direction::PosY => Direction::NegY,
            Direction::PosZ => Direction::NegZ,
        }
    }

    /// Flips the [Direction] based on [Flip].
    #[inline]
    pub const fn flip(self, flip: Flip) -> Self {
        use Direction::*;
        match self {
            NegX if flip.x() => PosX,
            NegY if flip.y() => PosY,
            NegZ if flip.z() => PosZ,
            PosX if flip.x() => NegX,
            PosY if flip.y() => NegY,
            PosZ if flip.z() => NegZ,
            _ => self
        }
    }

    /// Rotates the [Direction] by [Rotation].
    #[inline]
    pub fn rotate(self, rotation: Rotation) -> Self {
        rotation.reface(self)
    }

    /// Gets the [Axis] of the [Direction]
    #[inline]
    pub const fn axis(self) -> Axis {
        use Direction::*;
        match self {
            NegX | PosX => Axis::X,
            NegY | PosY => Axis::Y,
            NegZ | PosZ => Axis::Z,
        }
    }

    /// Represents discriminant as single bit value.
    #[inline]
    pub const fn bit(self) -> u8 {
        1 << self.discriminant()
    }

    /// Gets the discriminant of the value.
    #[inline]
    pub const fn discriminant(self) -> u8 {
        self as u8
    }

    #[inline]
    pub const fn rotation_discriminant(self) -> u8 {
        match self {
            Direction::PosY => 0,
            Direction::PosX => 1,
            Direction::PosZ => 2,
            Direction::NegY => 3,
            Direction::NegX => 4,
            Direction::NegZ => 5,
        }
    }

    /// Iterates in the order: `NegX`, `NegY`, `NegZ`, `PosX`, `PosY`, `PosZ`.
    #[inline]
    pub fn iter() -> impl Iterator<Item = Direction> {
        Self::ALL.into_iter()
    }

    /// Iterates the [Direction] enum in the order of the variants' discriminants.
    #[inline]
    pub fn iter_discriminant_order() -> impl Iterator<Item = Direction> {
        Self::INDEX_ORDER.into_iter()
    }

    /// On a non-oriented cube, each face has an "up" face. That's the face
    /// whose normal points to the top of the given face's UV plane.
    #[inline]
    pub const fn up(self) -> Direction {
        use Direction::*;
        match self {
            NegX => PosY,
            NegY => PosZ,
            NegZ => PosY,
            PosX => PosY,
            PosY => NegZ,
            PosZ => PosY,
        }
    }

    /// On a non-oriented cube, each face has a "down" face. That's the face
    /// whose normal points to the bottom of the given face's UV plane.
    #[inline]
    pub const fn down(self) -> Direction {
        use Direction::*;
        match self {
            NegX => NegY,
            NegY => NegZ,
            NegZ => NegY,
            PosX => NegY,
            PosY => PosZ,
            PosZ => NegY,
        }
    }

    /// On a non-oriented cube, each face has a "left" face. That's the face
    /// whose normal points to the left of the given face's UV plane.
    #[inline]
    pub const fn left(self) -> Direction {
        use Direction::*;
        match self {
            NegX => NegZ,
            NegY => NegX,
            NegZ => PosX,
            PosX => PosZ,
            PosY => NegX,
            PosZ => NegX,
        }
    }

    /// On a non-oriented cube, each face has a "right" face. That's the face
    /// whose normal points to the right of the given face's UV plane.
    #[inline]
    pub const fn right(self) -> Direction {
        use Direction::*;
        match self {
            NegX => PosZ,
            NegY => PosX,
            NegZ => NegX,
            PosX => NegZ,
            PosY => PosX,
            PosZ => PosX,
        }
    }

    /// Converts the [Direction] into a unit vector.
    #[inline]
    pub const fn to_vec3(self) -> Vec3 {
        use Direction::*;
        match self {
            NegX => Vec3::new(-1.0,  0.0,  0.0),
            NegY => Vec3::new( 0.0, -1.0,  0.0),
            NegZ => Vec3::new( 0.0,  0.0, -1.0),
            PosX => Vec3::new( 1.0,  0.0,  0.0),
            PosY => Vec3::new( 0.0,  1.0,  0.0),
            PosZ => Vec3::new( 0.0,  0.0,  1.0),
        }
    }

    /// Converts the [Direction] into a unit integer vector.
    #[inline]
    pub const fn to_ivec3(self) -> IVec3 {
        use Direction::*;
        match self {
            NegX => IVec3::new(-1,  0,  0),
            NegY => IVec3::new( 0, -1,  0),
            NegZ => IVec3::new( 0,  0, -1),
            PosX => IVec3::new( 1,  0,  0),
            PosY => IVec3::new( 0,  1,  0),
            PosZ => IVec3::new( 0,  0,  1),
        }
    }

    #[inline]
    pub const fn to_ftuple(self) -> (f32, f32, f32) {
        use Direction::*;
        match self {
            NegX => (-1.0,  0.0,  0.0),
            NegY => ( 0.0, -1.0,  0.0),
            NegZ => ( 0.0,  0.0, -1.0),
            PosX => ( 1.0,  0.0,  0.0),
            PosY => ( 0.0,  1.0,  0.0),
            PosZ => ( 0.0,  0.0,  1.0),
        }
    }

    #[inline]
    pub const fn to_ituple(self) -> (i32, i32, i32) {
        use Direction::*;
        match self {
            NegX => (-1,  0,  0),
            NegY => ( 0, -1,  0),
            NegZ => ( 0,  0, -1),
            PosX => ( 1,  0,  0),
            PosY => ( 0,  1,  0),
            PosZ => ( 0,  0,  1),
        }
    }

    #[inline]
    pub const fn to_farray(self) -> [f32; 3] {
        let (x, y, z) = self.to_ftuple();
        [x, y, z]
    }

    #[inline]
    pub const fn to_iarray(self) -> [i32; 3] {
        let (x, y, z) = self.to_ituple();
        [x, y, z]
    }
}

impl std::ops::Neg for Direction {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        self.invert()
    }
}

impl Into<Vec3> for Direction {
    #[inline]
    fn into(self) -> Vec3 {
        self.to_vec3()
    }
}

impl Into<IVec3> for Direction {
    #[inline]
    fn into(self) -> IVec3 {
        self.to_ivec3()
    }
}

impl Into<(i32, i32, i32)> for Direction {
    #[inline]
    fn into(self) -> (i32, i32, i32) {
        self.to_ituple()
    }
}

impl Into<(f32, f32, f32)> for Direction {
    #[inline]
    fn into(self) -> (f32, f32, f32) {
        self.to_ftuple()
    }
}

impl Into<[i32; 3]> for Direction {
    #[inline]
    fn into(self) -> [i32; 3] {
        self.to_iarray()
    }
}

impl Into<[f32; 3]> for Direction {
    #[inline]
    fn into(self) -> [f32; 3] {
        self.to_farray()
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::NegX => write!(f, "NegX"),
            Direction::NegY => write!(f, "NegY"),
            Direction::NegZ => write!(f, "NegZ"),
            Direction::PosX => write!(f, "PosX"),
            Direction::PosY => write!(f, "PosY"),
            Direction::PosZ => write!(f, "PosZ"),
        }
    }
}