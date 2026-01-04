use anyhow::{Result, bail};

#[derive(Debug)]
pub enum Mirroring {
    Horizontal = 0,
    Vertical = 1,
}

#[derive(Debug)]
pub struct Header {
    pub prg_rom_banks: usize,
    pub chr_rom_banks: usize,
    pub ram_banks: usize,

    pub mirroring: Mirroring,
    pub battery_backed_ram: bool,
    pub trainer: bool,
    pub mapper_number: u8,
}

impl Header {
    pub fn decode_ines_format(header: [u8; 16]) -> Result<Self> {
        // rom should start with "NES"
        if std::str::from_utf8(&header[0..3])? != "NES" {
            bail!("The first bytes should be 'NES'")
        }

        // next value should be 0x1A to identify the correct file format
        if header[3] != 0x1A {
            bail!("Byte 0x04 should be 0x1A")
        }

        // These bytes should all be 0 for every ROM
        if header[9..16].iter().any(|b| *b != 0) {
            bail!("Bytes from 0x09 to 0x0F should all be 0")
        }

        let prg_rom_banks = header[4].into();
        let chr_rom_banks = header[5].into();
        // expect 1 RAM bank when the value is 0
        let ram_banks = match header[8] {
            0 => 1,
            other => other,
        }.into();

        // control byte 1
        let control_byte_1 = header[6];
        let mut mirroring = match control_byte_1 & (1 << 0) {
            0 => Mirroring::Horizontal,
            1 => Mirroring::Vertical,
            other => bail!("Expected 0 or 1 for Mirroring, got {}", other)
        };
        // override mirroring if bit 3 of control byte 1 is not null
        if control_byte_1 & (1 << 3) == 1 {
            mirroring = Mirroring::Vertical;
        }

        let battery_backed_ram = match control_byte_1 & (1 << 1) {
            0 => false,
            1 => true,
            other => bail!("Expected 0 or 1 for battery-backed flag, got {}", other)
        };
        let trainer = match control_byte_1 & (1 << 2) {
            0 => false,
            1 => true,
            other => bail!("Expected 0 or 1 for battery-backed flag, got {}", other)
        };

        // control byte 2
        let control_byte_2 = header[7];
        if (control_byte_2 << 4) >> 4 != 0 {
            bail!("First four bytes of control byte 2 should be null, got {}", (control_byte_2 << 4) >> 4);
        }

        // mapper number
        // 4th lower bits from control byte 1
        // 4th upper bits from control byte 2
        let mapper_number = (control_byte_2 << 4) + (control_byte_1 >> 4);

        Ok(Self { 
            prg_rom_banks,
            chr_rom_banks,
            ram_banks,
            mirroring,
            battery_backed_ram,
            trainer,
            mapper_number
        })
    }
}