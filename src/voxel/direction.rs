use bytemuck::NoUninit;
use glam::Vec3;
use crate::math::axis::Axis;
use crate::voxel::orientation::{
    flip::Flip,
    rotation::Rotation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, NoUninit)]
#[repr(u8)]
// The ids are out of order so that they can have a certain order for orientations.
/// Represents each direction of a cube face.
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

    pub const LEFT: Direction = Direction::NegX;
    pub const DOWN: Direction = Direction::NegY;
    pub const FORWARD: Direction = Direction::NegZ;
    pub const RIGHT: Direction = Direction::PosX;
    pub const UP: Direction = Direction::PosY;
    pub const BACKWARD: Direction = Direction::PosZ;

    /// Invert the [Direction]. (`NegX` becomes `PosX`, `PosX` becomes `NegX`, etc.)
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
    pub fn rotate(self, rotation: Rotation) -> Self {
        rotation.reface(self)
    }

    /// Gets the [Axis] of the [Direction]
    pub const fn axis(self) -> Axis {
        use Direction::*;
        match self {
            NegX | PosX => Axis::X,
            NegY | PosY => Axis::Y,
            NegZ | PosZ => Axis::Z,
        }
    }

    /// Represents discriminant as single bit value.
    pub const fn bit(self) -> u8 {
        1 << self as u8
    }

    /// Gets the discriminant of the value.
    pub const fn discriminant(self) -> u8 {
        self as u8
    }

    /// Iterates in the order: `NegX`, `NegY`, `NegZ`, `PosX`, `PosY`, `PosZ`.
    pub fn iter() -> impl Iterator<Item = Direction> {
        Self::ALL.into_iter()
    }

    /// Iterates the [Direction] enum in the order of the variants' discriminants.
    pub fn iter_discriminant_order() -> impl Iterator<Item = Direction> {
        Self::INDEX_ORDER.into_iter()
    }

    /// On a non-oriented cube, each face has an "up" face. That's the face
    /// whose normal points to the top of the given face's UV plane.
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
}

impl std::ops::Neg for Direction {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        self.invert()
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