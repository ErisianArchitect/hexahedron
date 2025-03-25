#![allow(unused)]

use hexorient::{*, orient_table::*};

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

// (0, PosY) => (x, y, z), // Default rotation, no change.
// (0, PosX) => (y, -z, -x),
// (0, PosZ) => (x, -z, y),
// (0, NegY) => (x, -y, -z),
// (0, NegX) => (-y, -z, x),
// (0, NegZ) => (-x, -z, -y),
// (1, PosY) => (-z, y, x),
// (1, PosX) => (y, -x, z),
// (1, PosZ) => (-z, -x, y),
// (1, NegY) => (-z, -y, -x),
// (1, NegX) => (-y, -x, -z),
// (1, NegZ) => (z, -x, -y),
// (2, PosY) => (-x, y, -z),
// (2, PosX) => (y, z, x),
// (2, PosZ) => (-x, z, y),
// (2, NegY) => (-x, -y, z),
// (2, NegX) => (-y, z, -x),
// (2, NegZ) => (x, z, -y),
// (3, PosY) => (z, y, -x),
// (3, PosX) => (y, x, -z),
// (3, PosZ) => (z, x, y),
// (3, NegY) => (z, -y, x),
// (3, NegX) => (-y, x, z),
// (3, NegZ) => (-z, x, -y),

//  0 /* (0, PosY) */ => (x, y, z), // Default rotation, no change.
//  4 /* (0, PosX) */ => (y, -z, -x),
//  8 /* (0, PosZ) */ => (x, -z, y),
// 16 /* (0, NegY) */ => (x, -y, -z),
// 12 /* (0, NegX) */ => (-y, -z, x),
// 20 /* (0, NegZ) */ => (-x, -z, -y),
//  1 /* (1, PosY) */ => (-z, y, x),
//  5 /* (1, PosX) */ => (y, -x, z),
//  9 /* (1, PosZ) */ => (-z, -x, y),
// 17 /* (1, NegY) */ => (-z, -y, -x),
// 13 /* (1, NegX) */ => (-y, -x, -z),
// 21 /* (1, NegZ) */ => (z, -x, -y),
//  2 /* (2, PosY) */ => (-x, y, -z),
//  6 /* (2, PosX) */ => (y, z, x),
// 10 /* (2, PosZ) */ => (-x, z, y),
// 18 /* (2, NegY) */ => (-x, -y, z),
// 14 /* (2, NegX) */ => (-y, z, -x),
// 22 /* (2, NegZ) */ => (x, z, -y),
//  3 /* (3, PosY) */ => (z, y, -x),
//  7 /* (3, PosX) */ => (y, x, -z),
// 11 /* (3, PosZ) */ => (z, x, y),
// 19 /* (3, NegY) */ => (z, -y, x),
// 15 /* (3, NegX) */ => (-y, x, z),
// 23 /* (3, NegZ) */ => (-z, x, -y),

fn main() -> std::fmt::Result {
    // let mut code = String::new();
    // writeln!(code, "match ((self.angle(), self.up()), face) {{")?;
    // for angle in iter_angle() {
    //     for up in Direction::iter_discriminant_order() {
    //         let rot = Rotation::new(up, angle);
    //         let index = rot.0;
    //         println!("{index}");
    //     }
    // }
    map_coord_gencode();
    source_coord_gencode();
    // writeln!(code, "    _ => return None,")?;
    // write!(code, "}}")?;
    // std::fs::write("./codegen.rs", code).expect("Failed to write to file.");
    Ok(())
}

/* 
def rot_index(face, angle):
    return (face << 2) | (angle & 0b11)

def codeswap(src):
    for angle in range(4):
            for face in range(6):
                    index = rot_index(face, angle)
                    src = src.replace(f'({angle}, {dirs[face]})', f'{index:>2} /* ({angle}, {dirs[face]}) */')
    return src
*/

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

// I used this to generate the table in maptable.rs and I don't need it anymore, but I'm going
// to keep it around just in case.
fn map_face_coord_naive(orientation: Orientation, face: Direction) -> CoordMap {
    // First I will attempt a naive implementation, then I will use the naive implementation to generate code
    // for a more optimized implementation.
    // First get the source face
    let source_face = orientation.source_face(face);
    // next, get the up, right, down, and left for the source face and arg face.
    let face_up = face.up();
    let face_right = face.right();
    let src_up = source_face.up();
    let src_right = source_face.right();
    let src_down = source_face.down();
    let src_left = source_face.left();
    // Next, reface the src_dir faces
    let rsrc_up = orientation.reface(src_up);
    let rsrc_right = orientation.reface(src_right);
    let rsrc_down = orientation.reface(src_down);
    let rsrc_left = orientation.reface(src_left);
    // Now match up the faces
    let x_map = if face_right == rsrc_right {
        AxisMap::PosX
    } else if face_right == rsrc_up {
        AxisMap::NegY
    } else if face_right == rsrc_left {
        AxisMap::NegX
    } else {
        AxisMap::PosY
    };
    let y_map = if face_up == rsrc_up {
        AxisMap::PosY
    } else if face_up == rsrc_left {
        AxisMap::PosX
    } else if face_up == rsrc_down {
        AxisMap::NegY
    } else {
        AxisMap::NegX
    };
    CoordMap {
        x: x_map,
        y: y_map
    }
}

fn source_face_coord_naive(orientation: Orientation, face: Direction) -> CoordMap {
    // First I will attempt a naive implementation, then I will use the naive implementation to generate code
    // for a more optimized implementation.
    // First get the source face
    let source_face = orientation.source_face(face);
    // next, get the up, right, down, and left for the source face and arg face.
    let src_up = source_face.up();
    let src_right = source_face.right();
    let face_up = face.up();
    let face_right = face.right();
    let face_down = face.down();
    let face_left = face.left();
    // Next, reface the src_dir faces
    let rsrc_up = orientation.reface(src_up);
    let rsrc_right = orientation.reface(src_right);
    // Now match up the faces
    let x_map = if rsrc_right == face_right {
        AxisMap::PosX
    } else if rsrc_right == face_down {
        AxisMap::PosY
    } else if rsrc_right == face_left {
        AxisMap::NegX
    } else {
        AxisMap::NegY
    };
    let y_map = if rsrc_up == face_up {
        AxisMap::PosY
    } else if rsrc_up == face_right {
        AxisMap::NegX
    } else if rsrc_up == face_down {
        AxisMap::NegY
    } else {
        AxisMap::PosX
    };
    CoordMap {
        x: x_map,
        y: y_map
    }
}

// This is used to generate the table in maptable.rs.
// you need to uncoment map_up2_coord_naive for this to work.
// I commented it out because I don't need it anymore, but I'd like to keep
// the code around in case I need it later as a reference.
fn map_coord_gencode() {
    const fn map_axismap(a: AxisMap) -> &'static str {
        match a {
            AxisMap::PosX => "x",
            AxisMap::PosY => "y",
            AxisMap::NegX => "-x",
            AxisMap::NegY => "-y",
        }
    }
    let output = {
        use std::fmt::Write;
        let mut output = String::new();
        let mut count = 0usize;
        for flipi in 0..8 { // flip
            for roti in 0..24 { // rotation
                Direction::iter_discriminant_order().for_each(|face| {
                    count += 1;
                    let map = map_face_coord_naive(Orientation::new(Rotation(roti as u8), Flip(flipi as u8)), face);
                    writeln!(output, "CoordMap::new(AxisMap::{:?}, AxisMap::{:?}),", map.x, map.y);
                });
            }
        }
        output
    };
    std::fs::write("map_coord_gencode.rs", output.as_bytes());
    println!("Wrote the output to file at map_coord_gencode.rs");
}

fn source_coord_gencode() {
    const fn map_axismap(a: AxisMap) -> &'static str {
        match a {
            AxisMap::PosX => "x",
            AxisMap::PosY => "y",
            AxisMap::NegX => "-x",
            AxisMap::NegY => "-y",
        }
    }
    let output = {
        use std::fmt::Write;
        let mut output = String::new();
        let mut count = 0usize;
        for flipi in 0..8 { // flip
            for roti in 0..24 { // rotation
                Direction::iter_discriminant_order().for_each(|face| {
                    count += 1;
                    let map = source_face_coord_naive(Orientation::new(Rotation(roti as u8), Flip(flipi as u8)), face);
                    writeln!(output, "CoordMap::new(AxisMap::{:?}, AxisMap::{:?}),", map.x, map.y);
                });
            }
        }
        output
    };
    std::fs::write("source_coord_gencode.rs", output.as_bytes());
    println!("Wrote the output to file at source_coord_gencode.rs");
}

// // This is used to generate the table in maptable.rs.
// // you need to uncoment map_up2_coord_naive for this to work.
// // I commented it out because I don't need it anymore, but I'd like to keep
// // the code around in case I need it later as a reference.
// fn map_coord_gencode() {
//     const fn map_axismap(a: AxisMap) -> &'static str {
//         match a {
//             AxisMap::PosX => "x",
//             AxisMap::PosY => "y",
//             AxisMap::NegX => "-x",
//             AxisMap::NegY => "-y",
//         }
//     }
//     let output = {
//         use std::fmt::Write;
//         let mut output = String::new();
//         let mut count = 0usize;
//         for flipi in 0..8 { // flip
//             for roti in 0..24 { // rotation
//                 Direction::iter_index_order().for_each(|face| {
//                     count += 1;
//                     let map = map_face_coord_naive(Orientation::new(Rotation(roti as u8), Flip(flipi as u8)), face);
//                     writeln!(output, "CoordMap::new(AxisMap::{:?}, AxisMap::{:?}),", map.x, map.y);
//                 });
//             }
//         }
//         output
//     };
//     use std::io::{Write, BufWriter};
//     use std::fs::File;
//     let mut writer = BufWriter::new(File::create("ignore/map_coord_table.rs").expect("Failed to open file"));
//     writer.write_all(output.as_bytes());
//     println!("Wrote the output to file at ./ignore/map_coord_table.rs");
// }


// fn source_coord_gencode() {
//     const fn map_axismap(a: AxisMap) -> &'static str {
//         match a {
//             AxisMap::PosX => "x",
//             AxisMap::PosY => "y",
//             AxisMap::NegX => "-x",
//             AxisMap::NegY => "-y",
//         }
//     }
//     let output = {
//         use std::fmt::Write;
//         let mut output = String::new();
//         let mut count = 0usize;
//         for flipi in 0..8 { // flip
//             for roti in 0..24 { // rotation
//                 Direction::iter_index_order().for_each(|face| {
//                     count += 1;
//                     let map = source_face_coord_naive(Orientation::new(Rotation(roti as u8), Flip(flipi as u8)), face);
//                     writeln!(output, "CoordMap::new(AxisMap::{:?}, AxisMap::{:?}),", map.x, map.y);
//                 });
//             }
//         }
//         output
//     };
//     use std::io::{Write, BufWriter};
//     use std::fs::File;
//     let mut writer = BufWriter::new(File::create("ignore/source_face_coord_table.rs").expect("Failed to open file"));
//     writer.write_all(output.as_bytes());
//     println!("Wrote the output to file at ./ignore/source_face_coord_table.rs");
// }