pub mod rotation;
pub mod flip;
pub mod orientation;
mod orient_table;

use flip::Flip;
use rotation::Rotation;

pub const fn pack_flip_and_rotation(flip: Flip, rotation: Rotation) -> u8 {
    flip.0 | rotation.0 << 3
}

pub const fn unpack_flip_and_rotation(packed: u8) -> (Flip, Rotation) {
    let flip = packed & 0b111;
    let rotation = packed >> 3;
    (Flip(flip), Rotation(rotation))
}