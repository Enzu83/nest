use std::env::args;
use std::fs::File;
use std::io::Read;

use anyhow::{Result, bail};
use scene::Scene;

fn main() -> Result<()> {
    let path = match args().skip(1).next() {
        Some(arg) => arg,
        None =>  bail!("Please provide a path to the ROM."),
    };

    // open the ROM and store the content in a byte vector
    let mut rom = Vec::new();
    File::open(path)?.read_to_end(&mut rom)?;

    // get the first three elements of the ROM
    if std::str::from_utf8(&rom[..3])? != "NES" {
        bail!("ROM header is incorrect")
    };

    let scene = Scene::default()?;
    scene.run();

    Ok(())
}
