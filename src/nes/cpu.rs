use crate::bus::NiseBus;

pub mod addressing;
pub mod instructions;

pub struct Nise6502 {
    pc: u16,
    sp: u8,
    p: u8,
    a: u8,
    x: u8,
    y: u8,
    bus: NiseBus,
    cycle_count: arch,
}

enum CPUAddressingModes {}

impl CPU {
    fn tick(&mut self) {
        if (cycle_count == 0) {
            let opcode = bus.read(pc);
            pc += 1;

            match opcode {}
        }
    }

    //  Instructions accessing the stack
    fn brk(&mut self) {}
}
