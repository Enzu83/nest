use std::{fs::File, io::Read};

use anyhow::Result;

pub mod format;

use crate::utils::PRG_ROM_BANK_SIZE;

use self::format::Header;

pub struct Rom {
    data: Box<[u8]>,
    size: usize,
}

impl Rom {
    pub fn new(data: Box<[u8]>, size: usize) -> Self {
        Self { data, size }
    }

    pub fn load(path: &str) -> Result<Self> {
        let mut data = Vec::new();
        File::open(path)?.read_to_end(&mut data)?;
        let size = data.len();
        
        Ok(Self::new(data.into_boxed_slice(), size))
    }

    pub fn header_information(&self) -> Result<Header> {
        let mut header = [0; 16];
        header.copy_from_slice(&self.data[..16]);
        Header::decode_ines_format(header)
    }

    pub fn prg_rom_bank(&self, number: usize) -> &[u8; PRG_ROM_BANK_SIZE] {
        &[0; PRG_ROM_BANK_SIZE]
    }
}
