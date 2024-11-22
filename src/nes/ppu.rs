use crate::common::to_u16;

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
    internal_oam: [u8; 32],
    found_sprites: usize,
    vram: [u8; 2048],
    mirroring: fn(u16) -> usize,
    pub v: u16,
    pub t: u16,
    pub x: u16,
    pub w: u16,
    cycle_count: usize,
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
            internal_oam: [0; 32],
            found_sprites: 0,
            vram: [0; 2048],
            mirroring,
            v: 0,
            t: 0,
            x: 0,
            w: 0,
            cycle_count: 0,
        }
    }
    pub fn tick(&mut self) {
        // TODO: Improve cycle accuracy?
        let current_scanline = self.cycle_count / 341;
        let _current_cycle = self.cycle_count % 341;
        match current_scanline {
            1..=239 => {
                for _byte_num in 0..32 {
                    let nametable_entry = self.read(0x2000 + self.v);

                    let palette_index = self.v % 960;

                    let bg_palette = (self.read(
                        0x23C0
                            + 960 * (self.v / 960)
                            + palette_index % 8
                            + (palette_index / 64 * 8),
                    ) >> (palette_index / 2) % 2 + 2 * (palette_index / 32) % 2)
                        & 0x03;

                    let pattern_data =
                        self.read16(((self.ppuctrl & 0x10) << 8 + nametable_entry) as u16);
                    for k in 0..8 {
                        let pattern_index = ((pattern_data & (1 << k)) >> k)
                            & ((pattern_data & (1 << (8 + k))) >> (7 + k));
                        let pixel_color =
                            self.read(0x3F00 + (bg_palette << 2) as u16 + pattern_index as u16);
                        self.video_buffer[(self.v * 8) as usize + k] = pixel_color;
                    }

                    self.v += 1;
                }

                for i in (self.found_sprites - 1)..=0 {
                    let _attributes = self.internal_oam[4 * i + 2];
                    let x_pos = self.internal_oam[4 * i + 3];
                    let pattern_table_addr = if self.sprite_height() == 8 {
                        ((self.ppuctrl & 0x08) << 9) + self.internal_oam[4 * i + 1]
                    } else {
                        ((self.internal_oam[4 * i + 1] & 0x01) << 12)
                            + (self.internal_oam[4 * i + 1] >> 1)
                    };

                    for x in x_pos..=std::cmp::max(x_pos + 7, 0xFF) {
                        let pixel_color =
                            self.read(0x3F00 + (bg_palette << 2) as u16 + pattern_index as u16);

                        self.video_buffer[256 * current_scanline + x as usize] = self.;
                    }
                }

                // Sprite 0 Hit
                if self.internal_oam[0..4] == self.oam[0..4] {}

                (self.internal_oam, self.found_sprites) = self.sprite_evaluation(current_scanline);
            }
            241..=260 => {}
            _ => {}
        }
        self.cycle_count += 341;
    }

    fn sprite_height(&self) -> usize {
        if self.ppuctrl & 0b0001_0000 == 0 {
            8
        } else {
            16
        }
    }

    fn read(&self, address: u16) -> u8 {
        let mirroring = self.mirroring;
        match address {
            0x0..=0x1FFF => self.chr_rom[address as usize],
            0x2000..=0x3EFF => self.vram[mirroring(address)],
            _ => 0,
        }
    }

    fn read16(&mut self, address: u16) -> u16 {
        let low_byte = self.read(address);
        let high_byte = self.read(address + 1);
        to_u16(low_byte, high_byte)
    }

    fn sprite_evaluation(&mut self, current_scanline: usize) -> ([u8; 32], usize) {
        // TODO: Sprite overflow bug
        let mut internal_oam = [0xF; 32];
        let mut found_sprites: usize = 0;
        for n in 0..64 {
            let y_coordinate = self.oam[4 * n] as usize;
            if current_scanline >= y_coordinate
                && current_scanline < y_coordinate + self.sprite_height()
            {
                if found_sprites < 8 {
                    found_sprites += 1;
                    internal_oam[4 * found_sprites..4 * found_sprites + 4]
                        .copy_from_slice(&self.oam[4 * n..4 * n + 4]);
                    internal_oam[4 * found_sprites + 2] &= 0xE3;
                } else {
                    self.ppustatus |= 0x20;
                    break;
                }
            }
        }
        (internal_oam, found_sprites)
    }
}
