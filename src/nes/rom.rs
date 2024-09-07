const PRG_ROM_PAGE_SIZE: usize = 16384;
const CHR_ROM_PAGE_SIZE: usize = 8192;

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

pub trait MirrorAddr {
    fn mirror_addr(&self, address: u16) -> u16;
}

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

impl Rom {
    pub fn new(raw: &Vec<u8>) -> Option<Rom> {
        if raw[0..=3] != [0x4E, 0x45, 0x53, 0x1A] {
            return None;
        }

        let screen_mirroring: Mirroring = if raw[6] & 1 != 0 {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        };

        let prg_rom_length = raw[4] as usize;
        let chr_rom_length = raw[5] as usize;

        let prg_offset: usize = if raw[6] & (1 << 2) != 0 { 16 + 512 } else { 16 };
        let chr_offset = prg_rom_length * PRG_ROM_PAGE_SIZE + prg_offset;

        let prg_rom = raw[prg_offset..chr_offset].to_vec();
        let chr_rom = raw[chr_offset..chr_offset + chr_rom_length * CHR_ROM_PAGE_SIZE].to_vec();
        let mapper = raw[7] & 0xF0 | raw[6] >> 4;

        Some(Rom {
            prg_rom,
            chr_rom,
            mapper,
            screen_mirroring,
        })
    }
}

pub fn horizontal_mirrored_addr(address: u16) -> usize {
    match address {
        0x2400..=0x27FF | 0x2C00..=0x2FFF => address as usize - 0x400,
        _ => address as usize,
    }
}

pub fn vertical_mirrored_addr(address: u16) -> usize {
    match address {
        0x2800..=0x2FFF => address as usize - 0x800,
        _ => address as usize,
    }
}

// TODO: IMPLEMENT FOUR SCREEN MIRRORING
pub fn four_screen_mirrored_addr(address: u16) -> usize {
    address as usize
}
