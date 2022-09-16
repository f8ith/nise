use Crate::nise::nes::cpu;

pub struct NiseBus {
    memory: [u8; 2048],
}

impl NiseBus {
    fn read(&mut self, address: u16) {
        match address {
            0x0000..=0x1fff => self.memory[address & 0xb001111111111],
        }
    }
}
