use crate::nes::rom::Rom;

pub struct NiseBus {
    memory: [u8; 2048],
    rom: Rom,
}

impl NiseBus {
    pub fn new(rom: Rom) -> Self {
        Self {
            memory: [0; 2048],
            rom,
        }
    }
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1fff => {
                let mirrored_addr = address & 0b0011_11111111;
                self.memory[mirrored_addr as usize]
            }
            0x8000..=0xFFFF => self.read_prg_rom(address),
            _ => 0,
        }
    }
    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x1fff => {
                let mirrored_addr = address & 0b0011_11111111;
                self.memory[mirrored_addr as usize] = data as u8;
            }
            0x8000..=0xFFFF => {
                panic!("Attempt to write to Cartridge ROM space")
            }
            _ => {}
        }
    }
    fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            //mirror if needed
            addr = addr % 0x4000;
        }
        self.rom.prg_rom[addr as usize]
    }
}
