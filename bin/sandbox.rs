use flate2::Compression;
use hexahedron::util::extensions::*;
use std::io::{Cursor, Write};
use hexahedron::io::region::blocksize::*;
use flate2::write::GzEncoder;

struct Gzipper<'a>(GzEncoder<&'a mut Cursor<Vec<u8>>>, bool);

impl<'a> std::io::Write for Gzipper<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.1 = true;
        self.0.write(buf)
    }
    
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.0.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        self.0.write_fmt(fmt)
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        self.0.write_vectored(bufs)
    }
}

fn main() {
    let mut buffer = Cursor::new(Vec::<u8>::new());
    let encoder = GzEncoder::new(&mut buffer, Compression::new(5));
    drop(encoder);
    println!("Length: {}", buffer.get_ref().len());
    // encoder.finish().unwrap();
    // println!("Length: {}", buffer.get_ref().len());
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