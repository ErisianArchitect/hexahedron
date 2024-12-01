#![allow(unused)]

use glam::{
    IVec3,
    ivec3,
};
use hexahedron::util::extensions::*;
use hexahedron::io::region::block_size::*;


struct Seven0(([u8; 7]));
struct Seven1([u8; 7]);
struct Seven2([u8; 7]);

enum Seven {
    Seven0(Seven0),
    Seven1(Seven1),
    Seven2(Seven2),
}

struct NoCopy(u32);

fn main() {
    println!("{}", std::mem::size_of::<Option<std::num::NonZero<u8>>>());
    return;
    let mut updates: Vec<(NoCopy, IVec3)> = (0..256*256).map(|_| {
        (
            NoCopy(rand::random()),
            ivec3(rand::random(), rand::random(), rand::random()),
        )
    }).collect();
    let start_time = std::time::Instant::now();

    let update_clone = updates.iter().map(|(_, c)| *c).collect::<Vec<_>>();

    let elapsed = start_time.elapsed();

    let mut fin = 0i32;
    for c in update_clone.into_iter() {
        fin = fin.wrapping_add(c.x);
        fin = fin.wrapping_add(c.y);
        fin = fin.wrapping_add(c.z);
    }
    println!("Fin: {fin}");
    println!("Elapsed: {}", elapsed.as_secs_f64())
}

#[allow(unused)]
fn gen_bsn_table() -> std::fmt::Result {
    use std::fmt::Write;
    let mut table = String::new();
    writeln!(table, "// Column: Multiplier")?;
    writeln!(table, "// Row: 2.pow(Exponent)")?;
    write!(table, "//     ")?;
    for mult in 32.iter() {
        write!(table, "  {mult:2} ")?;
    }
    writeln!(table)?;
    for exp in 8.iter() {
        write!(table, "/* {exp} */ ")?;
        for mult in 32.iter() {
            let size = block_size_notation::<5>(mult, exp);
            write!(table, "{size:04},")?;
        }
        writeln!(table)?;
    }
    println!("{table}");
    Ok(())
}