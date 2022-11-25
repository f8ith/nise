use crate::nes::bus::NiseBus;

pub struct Nise6502 {
    pc: u16,
    sp: u16,
    p: u16,
    a: u16,
    x: u16,
    y: u16,
    bus: NiseBus,
    cycle_count: u32,
}

impl Nise6502 {
    fn tick(&mut self) {
        if self.cycle_count == 0 {
            let opcode = self.bus.read(self.pc);
            self.pc += 1;

            match opcode {
                0xA9 => {
                    let operand = self.immediate();
                    self.lda(operand)
                }
                0xA5 => {
                    let (operand, _) = self.zero_page();
                    self.lda(operand)
                }
                0xB5 => {
                    let (operand, _) = self.zero_page_x();
                    self.lda(operand)
                }
                0xAD => {
                    let (operand, _) = self.absolute();
                    self.lda(operand)
                }
                0xBD => {
                    let (operand, _) = self.absolute_x();
                    self.lda(operand)
                }
                0xB9 => {
                    let (operand, _) = self.absolute_y();
                    self.lda(operand)
                }
                0xA1 => {
                    let (operand, _) = self.absolute_y();
                    self.lda(operand)
                }

                _ => {}
            }
        } else {
            self.cycle_count -= 1;
        }
    }

    fn lda(&mut self, operand: u16) {
        self.a = operand;

        if self.a == 0 {
            self.p |= 0b0000_0010
        } else {
            self.p &= 0b1111_1101;
        }

        self.p &= 0b0111_1111;
        self.p |= self.a & 0b1000_0000
    }
    //  Instructions accessing the stack
    fn brk(&mut self) {}

    // Some Addressing modes
    fn immediate(&mut self) -> u16 {
        self.cycle_count += 2;
        self.pc += 1;
        self.bus.read(self.pc)
    }

    fn absolute(&mut self) -> (u16, u16) {
        self.cycle_count += 4;
        let effective_address = self.bus.read(self.pc + 2) << 8 | self.bus.read(self.pc + 1);
        self.pc += 2;
        (self.bus.read(effective_address), effective_address)
    }

    fn absolute_x(&mut self) -> (u16, u16) {
        let low_byte = self.bus.read(self.pc + 1) + self.x;
        let effective_address = self.bus.read(self.pc + 2) << 8 + low_byte;
        if low_byte > 0b1111 {
            self.cycle_count += 5
        } else {
            self.cycle_count += 4;
        }
        self.pc += 2;
        (self.bus.read(effective_address), effective_address)
    }

    fn absolute_y(&mut self) -> (u16, u16) {
        let low_byte = self.bus.read(self.pc + 1) + self.y;
        let effective_address = self.bus.read(self.pc + 2) << 8 + low_byte;
        if low_byte > 0b1111 {
            self.cycle_count += 5
        } else {
            self.cycle_count += 4;
        }
        self.pc += 2;
        (self.bus.read(effective_address), effective_address)
    }

    fn zero_page(&mut self) -> (u16, u16) {
        let effective_address = self.bus.read(self.pc + 1);
        self.cycle_count += 3;
        self.pc += 1;
        (self.bus.read(effective_address), effective_address)
    }

    fn zero_page_x(&mut self) -> (u16, u16) {
        let effective_address = self.bus.read(self.pc + 1) + self.x & 0b0000_1111;
        self.cycle_count += 4;
        self.pc += 1;
        (self.bus.read(effective_address), effective_address)
    }

    fn zero_page_y(&mut self) -> (u16, u16) {
        let effective_address = self.bus.read(self.pc + 1) + self.y & 0b0000_1111;

        self.cycle_count += 4;
        self.pc += 1;
        (self.bus.read(effective_address), effective_address)
    }

    fn indexed_indirect(&mut self) -> (u16, u16) {
        let pointer_address = self.bus.read(self.pc + 1);
        let effective_address = self.bus.read(pointer_address) + self.x;
        self.cycle_count += 6;
        self.pc += 1;
        (self.bus.read(effective_address), effective_address)
    }
}
