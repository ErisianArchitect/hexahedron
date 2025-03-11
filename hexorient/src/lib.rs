pub mod axis;
pub mod cardinal;
pub mod direction;
pub mod flip;
pub mod orient_table;
pub mod orientation;
pub mod rotation;

pub use axis::Axis;
pub use direction::Direction;
pub use flip::Flip;
pub use orientation::Orientation;
pub use rotation::Rotation;

#[inline]
pub const fn pack_flip_and_rotation(flip: Flip, rotation: Rotation) -> u8 {
    flip.0 | rotation.0 << 3
}

#[inline]
pub const fn unpack_flip_and_rotation(packed: u8) -> (Flip, Rotation) {
    let flip = packed & 0b111;
    let rotation = packed >> 3;
    (Flip(flip), Rotation(rotation))
}