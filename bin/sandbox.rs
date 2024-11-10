#![allow(unused)]
use hexahedron::util::extensions::*;
use hexahedron::io::region::blocksize::*;


struct Seven0(([u8; 7]));
struct Seven1([u8; 7]);
struct Seven2([u8; 7]);

enum Seven {
    Seven0(Seven0),
    Seven1(Seven1),
    Seven2(Seven2),
}

fn main() {
    println!("{}", 256i32.rem_euclid(256));
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