mod cpu;
mod emulator;
mod ppu;
mod rom;
mod utils;

use std::env::args;

use anyhow::{Result, bail};

use crate::{emulator::Emulator, rom::Rom};

fn main() -> Result<()> {
    // retrieve the rom path from the args
    let rom_path = match args().nth(1) {
        Some(arg) => arg,
        None => bail!("Please provide a path to the ROM."),
    };

    let rom = Rom::load(&rom_path)?;

    let mut emulator = Emulator::new(rom)?;
    emulator.init()?;
    emulator.run()?;

    Ok(())
}
