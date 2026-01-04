use crate::utils::PRG_ROM_BANK_SIZE;

pub struct CpuMemory {
    data: [u8; 0x10000],
}

impl CpuMemory {
    pub fn new() -> Self {
        Self { data: [0; 0x10000] }
    }

    pub fn load_prg_rom_lower_bank(&mut self, prg_rom_bank: &[u8; PRG_ROM_BANK_SIZE]) {
        self.data[0x8000..0xC000].copy_from_slice(prg_rom_bank);
    }

    pub fn load_prg_rom_upper_bank(&mut self, prg_rom_bank: &[u8; PRG_ROM_BANK_SIZE]) {
        self.data[0xC000..0x10000].copy_from_slice(prg_rom_bank);
    }
}
