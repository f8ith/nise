use crate::nes::cpu;

pub struct NiseBus {
    memory: [u8; 2048],
}

impl NiseBus {
    pub fn read(&mut self, address: u16) -> u16 {
        match address {
            0x0000..=0x1fff => {
                let mirrored_addr = address & 0b0011_11111111;
                self.memory[mirrored_addr as usize] as u16
            }
            _ => 0,
        }
    }
}
