use std::env::args;
use std::fs::File;
use std::io::Read;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let path = match args().skip(1).next() {
        Some(arg) => arg,
        None => return Err("Please provide a path to the ROM.".into()),
    };

    // open the ROM and store the content in a byte vector
    let mut rom = Vec::with_capacity(40976); // standard NES ROM size
    File::open(path)?.read_to_end(&mut rom)?;

    // get the first three elements of the ROM
    let header = std::str::from_utf8(&rom[..3])?;

    // the header should be "NES"
    println!("{}", header);

    Ok(())
}
