use crate::nes::ppu::NisePPU;
use crate::nes::rom::four_screen_mirrored_addr;
use crate::nes::rom::horizontal_mirrored_addr;
use crate::nes::rom::vertical_mirrored_addr;
use crate::nes::rom::Mirroring;
use crate::nes::rom::Rom;

pub struct NiseBus {
    memory: [u8; 2048],
    ppu: NisePPU,
    prg_rom: Vec<u8>,
}

impl NiseBus {
    pub fn new(rom: Rom) -> Self {
        let mirroring = match rom.screen_mirroring {
            Mirroring::Vertical => vertical_mirrored_addr,
            Mirroring::Horizontal => horizontal_mirrored_addr,
            Mirroring::FourScreen => four_screen_mirrored_addr,
        };
        let ppu = NisePPU::new(rom.chr_rom, mirroring);
        let prg_rom = rom.prg_rom;
        Self {
            memory: [0; 2048],
            ppu,
            prg_rom,
        }
    }

    // TODO: IMPLEMENT PPU REGISTER ACCESS
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1fff => {
                let mirrored_addr = address & 0b0011_11111111;
                self.memory[mirrored_addr as usize]
            }
            0x2000..=0x3FFF => {
                let mirrored_addr = address & 0x0007;
                match mirrored_addr {
                    0 | 1 | 3 | 5 | 6 => panic!("Attempt to read to write-only PPU register!"),
                    2 => self.ppu.ppustatus,
                    4 => self.ppu.oamdata,
                    7 => self.ppu.ppudata,
                    _ => panic!("Invalid mirrored address?"),
                }
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
            0x2000..=0x3FFF => {
                let mirrored_addr = address & 0x0007;
                match mirrored_addr {
                    2 => panic!("Attempt to write to read-only PPU register!"),
                    0 => self.ppu.ppuctrl = data,
                    1 => self.ppu.ppumask = data,
                    3 => self.ppu.oamaddr = data,
                    4 => self.ppu.oamdata = data,
                    5 => self.ppu.ppuscroll = data,
                    6 => self.ppu.ppuaddr = data,
                    7 => self.ppu.ppudata = data,
                    _ => panic!("Invalid mirrored address?"),
                }
            }
            0x8000..=0xFFFF => {
                panic!("Attempt to write to Cartridge ROM space")
            }
            _ => {}
        }
    }
    fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            //mirror if needed
            addr = addr % 0x4000;
        }
        self.prg_rom[addr as usize]
    }
}
