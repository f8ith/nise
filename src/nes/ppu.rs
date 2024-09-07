use crate::nes::rom::Mirroring;

pub struct NisePPU {
    pub ppuctrl: u8,
    pub ppumask: u8,
    pub ppustatus: u8,
    pub oamaddr: u8,
    pub oamdata: u8,
    pub ppuscroll: u8,
    pub ppuaddr: u8,
    pub ppudata: u8,
    pub oamdma: u8,
    chr_rom: Vec<u8>,
    video_buffer: [u8; 240],
    oam: [u8; 256],
    vram: [u8; 2048],
    mirroring: fn(u16) -> usize,
    cycle_count: u32,
}

impl NisePPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: fn(u16) -> usize) -> Self {
        let [ppuctrl, ppumask, ppustatus, oamaddr, oamdata, ppuscroll, ppuaddr, ppudata, oamdma] =
            [0; 9];
        NisePPU {
            ppuctrl,
            ppumask,
            ppustatus,
            oamaddr,
            oamdata,
            ppuscroll,
            ppuaddr,
            ppudata,
            oamdma,
            chr_rom,
            video_buffer: [0; 240],
            oam: [0; 256],
            vram: [0; 2048],
            mirroring,
            cycle_count: 0,
        }
    }
    pub fn cycle(&self) {}

    fn read(&self, address: u16) -> u8 {
        let mirroring = self.mirroring;
        match address {
            0x0..=0x1FFF => self.chr_rom[address as usize],
            0x2000..=0x3EFF => self.vram[mirroring(address)],
            _ => 0,
        }
    }
}
