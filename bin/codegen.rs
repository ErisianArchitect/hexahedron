use hexahedron::prelude::*;
use std::fmt::Write;

#[allow(unused)]
fn iter_angle() -> impl Iterator<Item = i32> {
    (0..4).into_iter()
}

pub fn rotate_coord<T: Copy + std::ops::Neg<Output = T>, C: Into<(T, T, T)> + From<(T, T, T)>>(rotation: Rotation, coord: C) -> C {
    let (x, y, z): (T, T, T) = coord.into();
    use Direction::*;
    C::from(match (rotation.angle(), rotation.up()) {
        (0, PosY) => (x, y, z), // Default rotation, no change.
        (0, PosX) => (y, -z, -x),
        (0, PosZ) => (x, -z, y),
        (0, NegY) => (x, -y, -z),
        (0, NegX) => (-y, -z, x),
        (0, NegZ) => (-x, -z, -y),
        (1, PosY) => (-z, y, x),
        (1, PosX) => (y, -x, z),
        (1, PosZ) => (-z, -x, y),
        (1, NegY) => (-z, -y, -x),
        (1, NegX) => (-y, -x, -z),
        (1, NegZ) => (z, -x, -y),
        (2, PosY) => (-x, y, -z),
        (2, PosX) => (y, z, x),
        (2, PosZ) => (-x, z, y),
        (2, NegY) => (-x, -y, z),
        (2, NegX) => (-y, z, -x),
        (2, NegZ) => (x, z, -y),
        (3, PosY) => (z, y, -x),
        (3, PosX) => (y, x, -z),
        (3, PosZ) => (z, x, y),
        (3, NegY) => (z, -y, x),
        (3, NegX) => (-y, x, z),
        (3, NegZ) => (-z, x, -y),
        _  => unreachable!(),
    })
}

fn main() -> std::fmt::Result {
    let mut code = String::new();
    writeln!(code, "match ((self.angle(), self.up()), face) {{")?;
    for angle in iter_angle() {
        for up in Direction::iter_discriminant_order() {
            for face in Direction::iter_discriminant_order() {
                
                writeln!(code, "    ({up}, {face}) => {angle},")?;
            }
        }
    }
    writeln!(code, "    _ => return None,")?;
    write!(code, "}}")?;
    std::fs::write("./codegen.rs", code).expect("Failed to write to file.");
    Ok(())
}

fn up_and_forward_angle(up: Direction, forward: Direction) -> Option<i32> {
    Some(match up {
        Direction::NegX => match forward {
            Direction::NegX => return None,
            Direction::NegY => 2,
            Direction::NegZ => 3,
            Direction::PosX => return None,
            Direction::PosY => 0,
            Direction::PosZ => 1
        },
        Direction::NegY => match forward {
            Direction::NegX => 3,
            Direction::NegY => return None,
            Direction::NegZ => 2,
            Direction::PosX => 1,
            Direction::PosY => return None,
            Direction::PosZ => 0
        },
        Direction::NegZ => match forward {
            Direction::NegX => 1,
            Direction::NegY => 2,
            Direction::NegZ => return None,
            Direction::PosX => 3,
            Direction::PosY => 0,
            Direction::PosZ => return None
        },
        Direction::PosX => match forward {
            Direction::NegX => return None,
            Direction::NegY => 2,
            Direction::NegZ => 1,
            Direction::PosX => return None,
            Direction::PosY => 0,
            Direction::PosZ => 3
        },
        Direction::PosY => match forward {
            Direction::NegX => 3,
            Direction::NegY => return None,
            Direction::NegZ => 0,
            Direction::PosX => 1,
            Direction::PosY => return None,
            Direction::PosZ => 2
        },
        Direction::PosZ => match forward {
            Direction::NegX => 3,
            Direction::NegY => 2,
            Direction::NegZ => return None,
            Direction::PosX => 1,
            Direction::PosY => 0,
            Direction::PosZ => return None
        },
    })
}