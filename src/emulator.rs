use anyhow::{Result, bail};

use crate::{cpu::{Cpu, CpuMemory}, ppu::Ppu, rom::{Rom, format::Header}};

pub struct Emulator {
    cpu: Cpu,
    cpu_memory: CpuMemory,
    ppu: Ppu,
    rom: Rom,
    header: Header,
}

impl Emulator {
    pub fn new(rom: Rom) -> Result<Self> {
        let header = rom.header_information()?;

        Ok(Self {
            cpu: Cpu::new(),
            cpu_memory: CpuMemory::new(),
            ppu: Ppu::new(),
            rom,
            header,
        })
    }

    pub fn init(&mut self) -> Result<()> {
        self.reset_cpu_memory()?;

        Ok(())
    }

    pub fn reset_cpu_memory(&mut self) -> Result<()> {
        match self.header.mapper_number {
            // no mapping, both PRG-ROM banks are filled
            0 => {
                self.cpu_memory.load_prg_rom_lower_bank(self.rom.prg_rom_bank(1));
                self.cpu_memory.load_prg_rom_upper_bank(self.rom.prg_rom_bank(2));
            },
            other => bail!("Unsupported memory mapper: {}", other),
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        println!("{:?}", self.rom.header_information()?);

        loop {
            if 1 == 0 {
                break;
            }
        }

        Ok(())
    }
}