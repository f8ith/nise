use crate::nes::bus::NiseBus;
use log::debug;
use log::warn;

#[cfg(feature = "nestest")]
pub struct Nise6502State {
    pc: u16,
    s: u8,
    p: u8,
    a: u8,
    x: u8,
    y: u8,
    cycle_count: u32,
}

#[cfg(feature = "nestest")]
impl From<&mut Nise6502> for Nise6502State {
    fn from(cpu: &mut Nise6502) -> Self {
        Self {
            pc: cpu.pc,
            s: cpu.s,
            p: cpu.p,
            a: cpu.a,
            x: cpu.x,
            y: cpu.y,
            cycle_count: cpu.cycle_count,
        }
    }
}

pub struct Nise6502 {
    pc: u16,
    s: u8,
    p: u8,
    a: u8,
    x: u8,
    y: u8,
    bus: NiseBus,
    cycle_count: u32,
}

struct Operand {
    value: u8,
    address: u16,
}

#[cfg(feature = "nestest")]
fn setup_nestest_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        //        .format(|out, message, record| {
        //            out.finish(format_args!(
        //                "[{} {} {}] {}",
        //                humantime::format_rfc3339_seconds(SystemTime::now()),
        //                record.level(),
        //                record.target(),
        //                message
        //            ))
        //        })
        .format(|out, message, _| out.finish(format_args!("{}", message)))
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

impl Nise6502 {
    pub fn new(bus: NiseBus) -> Self {
        let cpu = Self {
            pc: 0,
            s: 0xFD,
            p: 0x24,
            a: 0,
            x: 0,
            y: 0,
            bus,
            cycle_count: 0,
        };
        cpu
    }

    #[cfg(feature = "nestest")]
    pub fn nestest(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        setup_nestest_logger()?;
        self.pc = 0xc000;
        for _ in 0..40000 {
            self.tick();
        }
        Ok(())
    }

    #[cfg(feature = "nestest")]
    fn nestest_dbgprint(
        &mut self,
        old_state: Nise6502State,
        name: &str,
        fetch: &str,
        operand: &Operand,
    ) {
        let operand_bytecode = match fetch {
            "zpa" | "zpx" | "zpy" | "imm" | "idx" | "idy" | "idy_w" | "rel" => {
                format!("{:02X}   ", self.read(old_state.pc))
            }
            "abs" | "aby" | "abx" | "abx_w" | "aby_w" | "ind" => format!(
                "{:02X} {:02X}",
                self.read(old_state.pc),
                self.read(old_state.pc + 1)
            ),
            _ => "     ".to_string(),
        };
        debug!(
            "{:04X}  {:02X} {}  {} {:02X}                            A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            old_state.pc - 1,
            self.read(old_state.pc - 1),
            operand_bytecode,
            name.to_uppercase(),
            operand.value,
            old_state.a,
            old_state.x,
            old_state.y,
            old_state.p,
            old_state.s
        );
    }

    pub fn tick(&mut self) {
        macro_rules! ex {
            ($name:ident, $fetch:ident) => {{
                #[cfg(feature = "nestest")]
                let p_state: Nise6502State = self.into();
                let operand = self.$fetch();
                #[cfg(feature = "nestest")]
                self.nestest_dbgprint(p_state, stringify!($name), stringify!($fetch), &operand);
                self.$name(operand)
            }};
        }
        if self.cycle_count == 0 {
            let opcode = self.read(self.pc);
            self.pc += 1;
            match opcode {
                0x69 => ex!(adc, imm),
                0x65 => ex!(adc, zpa),
                0x75 => ex!(adc, zpx),
                0x6D => ex!(adc, abs),
                0x7D => ex!(adc, abx),
                0x79 => ex!(adc, aby),
                0x61 => ex!(adc, idx),
                0x71 => ex!(adc, idy),
                0x29 => ex!(and, imm),
                0x25 => ex!(and, zpa),
                0x35 => ex!(and, zpx),
                0x2D => ex!(and, abs),
                0x3D => ex!(and, abx),
                0x39 => ex!(and, aby),
                0x21 => ex!(and, idx),
                0x31 => ex!(and, idy),
                0x0A => ex!(asl_a, imp),
                0x06 => ex!(asl, zpa),
                0x16 => ex!(asl, zpx),
                0x0E => ex!(asl, abs),
                0x1E => ex!(asl, abx_w),
                0x90 => ex!(bcc, rel),
                0xB0 => ex!(bcs, rel),
                0xF0 => ex!(beq, rel),
                0x24 => ex!(bit, zpa),
                0x2C => ex!(bit, abs),
                0x30 => ex!(bmi, rel),
                0xD0 => ex!(bne, rel),
                0x10 => ex!(bpl, rel),
                0x00 => ex!(brk, imp),
                0x50 => ex!(bvc, rel),
                0x70 => ex!(bvs, rel),
                0x18 => ex!(clc, imp),
                0xD8 => ex!(cld, imp),
                0x58 => ex!(cli, imp),
                0xB8 => ex!(clv, imp),
                0xC9 => ex!(cmp, imm),
                0xC5 => ex!(cmp, zpa),
                0xD5 => ex!(cmp, zpx),
                0xCD => ex!(cmp, abs),
                0xDD => ex!(cmp, abx),
                0xD9 => ex!(cmp, aby),
                0xC1 => ex!(cmp, idx),
                0xD1 => ex!(cmp, idy),
                0xE0 => ex!(cpx, imm),
                0xE4 => ex!(cpx, zpa),
                0xEC => ex!(cpx, abs),
                0xC0 => ex!(cpy, imm),
                0xC4 => ex!(cpy, zpa),
                0xCC => ex!(cpy, abs),
                0xC6 => ex!(dec, zpa),
                0xD6 => ex!(dec, zpx),
                0xCE => ex!(dec, abs),
                0xDE => ex!(dec, abx_w),
                0xCA => ex!(dex, imp),
                0x88 => ex!(dey, imp),
                0x49 => ex!(eor, imm),
                0x45 => ex!(eor, zpa),
                0x55 => ex!(eor, zpx),
                0x4D => ex!(eor, abs),
                0x5D => ex!(eor, abx),
                0x59 => ex!(eor, aby),
                0x41 => ex!(eor, idx),
                0x51 => ex!(eor, idy),
                0xE6 => ex!(inc, zpa),
                0xF6 => ex!(inc, zpx),
                0xEE => ex!(inc, abs),
                0xFE => ex!(inc, abx),
                0xE8 => ex!(inx, imp),
                0xC8 => ex!(iny, imp),
                0x4C => ex!(jmp, abs),
                0x6C => ex!(jmp, ind),
                0x20 => ex!(jsr, abs),
                0xA9 => ex!(lda, imm),
                0xA5 => ex!(lda, zpa),
                0xB5 => ex!(lda, zpx),
                0xAD => ex!(lda, abs),
                0xBD => ex!(lda, abx),
                0xB9 => ex!(lda, aby),
                0xA1 => ex!(lda, idx),
                0xB1 => ex!(lda, idy),
                0xA2 => ex!(ldx, imm),
                0xA6 => ex!(ldx, zpa),
                0xB6 => ex!(ldx, zpy),
                0xAE => ex!(ldx, abs),
                0xBE => ex!(ldx, aby),
                0xA0 => ex!(ldy, imm),
                0xA4 => ex!(ldy, zpa),
                0xB4 => ex!(ldy, zpx),
                0xAC => ex!(ldy, abs),
                0xBC => ex!(ldy, abx),
                0x4A => ex!(lsr_a, imp),
                0x46 => ex!(lsr, zpa),
                0x56 => ex!(lsr, zpx),
                0x4E => ex!(lsr, abs),
                0x5E => ex!(lsr, abx_w),
                0xEA => ex!(nop, imp),
                0x09 => ex!(ora, imm),
                0x05 => ex!(ora, zpa),
                0x15 => ex!(ora, zpx),
                0x0D => ex!(ora, abs),
                0x1D => ex!(ora, abx),
                0x19 => ex!(ora, aby),
                0x01 => ex!(ora, idx),
                0x11 => ex!(ora, idy),
                0x48 => ex!(pha, imp),
                0x08 => ex!(php, imp),
                0x68 => ex!(pla, imp),
                0x28 => ex!(plp, imp),
                0x2A => ex!(rol_a, imp),
                0x26 => ex!(rol, zpa),
                0x36 => ex!(rol, zpx),
                0x2E => ex!(rol, abs),
                0x3E => ex!(rol, abx_w),
                0x6A => ex!(ror_a, imp),
                0x66 => ex!(ror, zpa),
                0x76 => ex!(ror, zpx),
                0x6E => ex!(ror, abs),
                0x7E => ex!(ror, abx_w),
                0x40 => ex!(rti, imp),
                0x60 => ex!(rts, imp),
                0xE9 => ex!(sbc, imm),
                0xE5 => ex!(sbc, zpa),
                0xF5 => ex!(sbc, zpx),
                0xED => ex!(sbc, abs),
                0xFD => ex!(sbc, abx),
                0xF9 => ex!(sbc, aby),
                0xE1 => ex!(sbc, idx),
                0xF1 => ex!(sbc, idy),
                0x38 => ex!(sec, imp),
                0xF8 => ex!(sed, imp),
                0x78 => ex!(sei, imp),
                0x85 => ex!(sta, zpa),
                0x95 => ex!(sta, zpx),
                0x8D => ex!(sta, abs),
                0x9D => ex!(sta, abx_w),
                0x99 => ex!(sta, aby_w),
                0x81 => ex!(sta, idx),
                0x91 => ex!(sta, idy_w),
                0x86 => ex!(stx, zpa),
                0x96 => ex!(stx, zpy),
                0x8E => ex!(stx, abs),
                0x84 => ex!(sty, zpa),
                0x94 => ex!(sty, zpx),
                0x8C => ex!(sty, abs),
                0xAA => ex!(tax, imp),
                0xA8 => ex!(tay, imp),
                0xBA => ex!(tsx, imp),
                0x8A => ex!(txa, imp),
                0x9A => ex!(txs, imp),
                0x98 => ex!(tya, imp),
                _ => {
                    warn!("Unimplemented opcode: {:#04X}", opcode);
                    return;
                }
            }
        } else {
            self.cycle_count -= 1;
        }
    }

    fn read(&self, address: u16) -> u8 {
        self.bus.read(address)
    }

    fn read16(&self, address: u16) -> u16 {
        self.to_u16(self.read(address), self.read(address + 1))
    }

    fn to_u16(&self, l: u8, h: u8) -> u16 {
        (l as u16) | (h as u16) << 8
    }

    fn set_nz(&mut self, value: u8) {
        if value == 0 {
            self.p |= 0b0000_0010
        } else {
            self.p &= 0b1111_1101;
        }

        self.p &= 0b0111_1111;
        self.p |= value & 0b1000_0000
    }

    fn push(&mut self, value: u8) {
        self.bus.write(0x100 + self.s as u16, value);
        self.s -= 1;
    }

    fn pop(&mut self) -> u8 {
        self.s += 1;
        let value = self.read(0x100 + self.s as u16);
        value
    }

    fn adc(&mut self, operand: Operand) {
        let result = self.a as usize + operand.value as usize + (self.p & 1) as usize;
        self.p &= 0b1011_1110;
        self.p |= ((result >> 8) & 1) as u8;

        self.p |= (!(self.a ^ operand.value) & (self.a ^ result as u8) & 0x80) >> 1;

        self.a = result as u8;
        self.set_nz(self.a)
    }

    fn and(&mut self, operand: Operand) {
        self.a &= operand.value;

        self.set_nz(self.a)
    }

    fn asl(&mut self, operand: Operand) {
        self.cycle_count += 2;
        self.p &= 0b1111_1110;
        self.p |= operand.value >> 7;
        let result = operand.value << 1;
        self.bus.write(operand.address, result);
        self.set_nz(result)
    }

    fn asl_a(&mut self, _: Operand) {
        self.cycle_count += 2;

        self.p &= 0b1111_1110;
        self.p |= self.a >> 7;
        self.a <<= 1;

        self.set_nz(self.a)
    }

    fn branch(&mut self, value: u8) {
        self.cycle_count += 2;
        self.pc = self.pc.wrapping_add(value as i8 as i16 as u16);
    }

    fn bcc(&mut self, operand: Operand) {
        if (self.p & 1) == 0 {
            self.branch(operand.value)
        }
    }

    fn bcs(&mut self, operand: Operand) {
        if (self.p & 1) == 1 {
            self.branch(operand.value)
        }
    }

    fn beq(&mut self, operand: Operand) {
        if ((self.p >> 1) & 1) == 1 {
            self.branch(operand.value)
        }
    }

    fn bit(&mut self, operand: Operand) {
        self.p &= 0b1111_1101;
        if (self.a & operand.value) == 0 {
            self.p |= 0b0000_0010;
        }
        self.p &= 0b0011_1111;
        self.p |= operand.value & 0b1100_0000
    }

    fn bmi(&mut self, operand: Operand) {
        if ((self.p >> 7) & 1) == 1 {
            self.branch(operand.value)
        }
    }

    fn bne(&mut self, operand: Operand) {
        if ((self.p >> 1) & 1) == 0 {
            self.branch(operand.value)
        }
    }

    fn bpl(&mut self, operand: Operand) {
        if ((self.p >> 7) & 1) == 0 {
            self.branch(operand.value)
        }
    }

    fn brk(&mut self, _: Operand) {
        self.cycle_count += 7;
        self.pc += 1;
        self.push((self.pc >> 8) as u8);
        self.push((self.pc | 0x00ff) as u8);
        self.push(self.p | 0x30);
        self.pc = self.read16(0xFFFE);
        self.p |= 0b0000_0100;
    }

    fn bvc(&mut self, operand: Operand) {
        if ((self.p >> 6) & 1) == 0 {
            self.branch(operand.value);
        }
    }

    fn bvs(&mut self, operand: Operand) {
        if ((self.p >> 6) & 1) == 1 {
            self.branch(operand.value);
        }
    }

    fn clc(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.p &= 0b1111_1110;
    }

    fn cld(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.p &= 0b1111_0111;
    }

    fn cli(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.p &= 0b1111_1011;
    }

    fn clv(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.p &= 0b1011_1111;
    }

    fn compare(&mut self, operand_left: u8, operand_right: u8) {
        let result = operand_left.wrapping_sub(operand_right);
        if result == 0 {
            self.p &= 0b0111_1110;
            self.p |= 0b0000_0011
        } else if operand_left >= operand_right {
            self.p &= 0b0111_1101;
            self.p |= 1
        } else {
            self.p &= 0b0111_1100;
        }

        self.p |= result & 0b1000_0000
    }

    fn cmp(&mut self, operand: Operand) {
        self.compare(self.a, operand.value)
    }

    fn cpx(&mut self, operand: Operand) {
        self.compare(self.x, operand.value)
    }

    fn cpy(&mut self, operand: Operand) {
        self.compare(self.y, operand.value)
    }

    fn dec(&mut self, operand: Operand) {
        self.cycle_count += 2;
        let result = operand.value.wrapping_sub(1);
        self.bus.write(operand.address, result);
        self.set_nz(result)
    }

    fn dex(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.x = self.x.wrapping_sub(1);
        self.set_nz(self.x)
    }

    fn dey(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.y = self.y.wrapping_sub(1);
        self.set_nz(self.y)
    }

    fn eor(&mut self, operand: Operand) {
        self.a ^= operand.value;
        self.set_nz(self.a)
    }

    fn inc(&mut self, operand: Operand) {
        self.cycle_count += 2;
        let result = operand.value.wrapping_add(1);
        self.bus.write(operand.address, result);
        self.set_nz(result)
    }

    fn inx(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.x = self.x.wrapping_add(1);
        self.set_nz(self.x)
    }

    fn iny(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.y = self.y.wrapping_add(1);
        self.set_nz(self.y)
    }

    fn jmp(&mut self, operand: Operand) {
        self.cycle_count -= 1;
        self.pc = operand.address;
    }

    fn jsr(&mut self, operand: Operand) {
        self.cycle_count += 2;
        self.pc -= 1;
        self.push((self.pc >> 8) as u8);
        self.push((self.pc & 0x00ff) as u8);
        self.pc = operand.address;
    }

    fn lda(&mut self, operand: Operand) {
        self.a = operand.value;

        self.set_nz(self.a)
    }

    fn ldx(&mut self, operand: Operand) {
        self.x = operand.value;
        self.set_nz(self.x)
    }

    fn ldy(&mut self, operand: Operand) {
        self.y = operand.value;
        self.set_nz(self.y)
    }

    fn lsr(&mut self, operand: Operand) {
        self.cycle_count += 2;
        self.p &= 0b1111_1110;
        self.p |= operand.value & 1;

        let result = operand.value >> 1;
        self.bus.write(operand.address, result);

        self.set_nz(result)
    }

    fn lsr_a(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.p &= 0b1111_1110;
        self.p |= self.a & 1;
        self.a >>= 1;

        self.set_nz(self.a)
    }

    fn nop(&mut self, _: Operand) {
        self.cycle_count += 2;
    }

    fn ora(&mut self, operand: Operand) {
        self.a |= operand.value;
        self.set_nz(self.a);
    }

    fn pha(&mut self, _: Operand) {
        self.cycle_count += 3;
        self.push(self.a);
    }

    fn php(&mut self, _: Operand) {
        self.cycle_count += 3;
        self.push(self.p | 0x30);
    }

    fn pla(&mut self, _: Operand) {
        self.cycle_count += 4;
        self.a = self.pop();
        self.set_nz(self.a);
    }

    fn plp(&mut self, _: Operand) {
        self.cycle_count += 4;
        self.p = self.pop() | 0x30 & 0xEF;
    }

    fn rol(&mut self, operand: Operand) {
        self.cycle_count += 2;
        let result = operand.value << 1 | (self.p & 1);
        self.p &= 0b1111_1110;
        self.p |= operand.value >> 7;
        self.bus.write(operand.address, result);

        self.set_nz(result);
    }

    fn rol_a(&mut self, _: Operand) {
        self.cycle_count += 2;
        let result = self.a << 1 | (self.p & 1);
        self.p &= 0b1111_1110;
        self.p |= self.a >> 7;
        self.a = result;

        self.set_nz(result);
    }

    fn ror(&mut self, operand: Operand) {
        self.cycle_count += 2;
        let result = operand.value >> 1 | ((self.p & 1) << 7);
        self.p &= 0b1111_1110;
        self.p |= self.a & 0b0000_0001;
        self.bus.write(operand.address, result);

        self.set_nz(result);
    }

    fn ror_a(&mut self, _: Operand) {
        self.cycle_count += 2;
        let result = self.a >> 1 | ((self.p & 1) << 7);
        self.p &= 0b1111_1110;
        self.p |= self.a & 0b0000_0001;
        self.a = result;

        self.set_nz(result);
    }

    fn rti(&mut self, _: Operand) {
        self.cycle_count += 6;
        self.p = self.pop() | 0x30 & 0xEF;
        let pcl = self.pop();
        let pch = self.pop();
        self.pc = self.to_u16(pcl, pch)
    }

    fn rts(&mut self, _: Operand) {
        self.cycle_count += 6;
        let pcl = self.pop();
        let pch = self.pop();
        self.pc = self.to_u16(pcl, pch) + 1
    }

    fn sbc(&mut self, operand: Operand) {
        self.adc(Operand {
            value: !operand.value,
            address: operand.address,
        })
    }

    fn sec(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.p |= 1
    }

    fn sed(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.p |= 0b0000_1000
    }

    fn sei(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.p |= 0b0000_0100
    }

    fn sta(&mut self, operand: Operand) {
        self.bus.write(operand.address, self.a)
    }

    fn stx(&mut self, operand: Operand) {
        self.bus.write(operand.address, self.x)
    }

    fn sty(&mut self, operand: Operand) {
        self.bus.write(operand.address, self.y)
    }

    fn tax(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.x = self.a;
        self.set_nz(self.x);
    }

    fn tay(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.y = self.a;
        self.set_nz(self.y)
    }

    fn tsx(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.x = self.s;
        self.set_nz(self.x)
    }

    fn txa(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.a = self.x;
        self.set_nz(self.a)
    }

    fn txs(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.s = self.x;
    }

    fn tya(&mut self, _: Operand) {
        self.cycle_count += 2;
        self.a = self.y;
        self.set_nz(self.a)
    }

    // Addressing modes
    fn imm(&mut self) -> Operand {
        self.cycle_count += 2;
        let value = self.pc;
        self.pc += 1;
        Operand {
            value: self.read(value),
            address: 0,
        }
    }

    fn abs(&mut self) -> Operand {
        self.cycle_count += 4;
        let effective_address = self.read16(self.pc);
        self.pc += 2;
        Operand {
            value: self.read(effective_address),
            address: effective_address,
        }
    }

    fn absolute_indexed(&mut self, index: u8) -> (Operand, bool) {
        let (low_byte, crossed) = self.read(self.pc).overflowing_add(index);
        let mut effective_address = self.to_u16(low_byte, self.read(self.pc + 1));
        if crossed {
            effective_address = effective_address.wrapping_add(0x100);
            self.cycle_count += 5
        } else {
            self.cycle_count += 4;
        }
        self.pc += 2;
        (
            Operand {
                value: self.read(effective_address),
                address: effective_address,
            },
            crossed,
        )
    }

    fn abx(&mut self) -> Operand {
        let (operand, _) = self.absolute_indexed(self.x);
        operand
    }

    fn aby(&mut self) -> Operand {
        let (operand, _) = self.absolute_indexed(self.y);
        operand
    }

    fn absolute_indexed_w(&mut self, index: u8) -> Operand {
        let (operand, crossed) = self.absolute_indexed(index);
        if !crossed {
            self.cycle_count += 1
        };
        operand
    }

    fn abx_w(&mut self) -> Operand {
        self.absolute_indexed_w(self.x)
    }

    fn aby_w(&mut self) -> Operand {
        self.absolute_indexed_w(self.y)
    }

    fn zpa(&mut self) -> Operand {
        self.cycle_count += 3;
        let effective_address = self.read(self.pc) as u16;
        self.pc += 1;
        Operand {
            value: self.read(effective_address),
            address: effective_address,
        }
    }

    fn zero_page_indexed(&mut self, index: u8) -> Operand {
        self.cycle_count += 4;
        let effective_address = (self.read(self.pc).wrapping_add(index)) as u16;
        self.pc += 1;
        Operand {
            value: self.read(effective_address),
            address: effective_address,
        }
    }

    fn zpx(&mut self) -> Operand {
        self.zero_page_indexed(self.x)
    }

    fn zpy(&mut self) -> Operand {
        self.zero_page_indexed(self.y)
    }

    fn ind(&mut self) -> Operand {
        self.cycle_count += 6;
        let pointer_address = self.read16(self.pc);
        let low_byte = self.read(pointer_address);
        let high_byte = self.read(self.to_u16(
            ((pointer_address & 0xFF) as u8).wrapping_add(1),
            (pointer_address >> 8) as u8,
        ));
        self.pc += 2;
        Operand {
            value: 0,
            address: self.to_u16(low_byte, high_byte),
        }
    }

    fn idx(&mut self) -> Operand {
        self.cycle_count += 6;
        let pointer_address = self.read(self.pc);
        let indexed_address = pointer_address.wrapping_add(self.x);
        let effective_address = self.read16(indexed_address as u16);
        self.pc += 1;
        Operand {
            value: self.read(effective_address),
            address: effective_address,
        }
    }

    fn idy(&mut self) -> Operand {
        let (operand, _) = self.indirected_indexed();
        operand
    }

    fn indirected_indexed(&mut self) -> (Operand, bool) {
        let pointer_address = self.read(self.pc) as u16;
        self.pc += 1;
        let (low_byte, crossed) = (self.read(pointer_address)).overflowing_add(self.y);
        let mut effective_address = self.to_u16(low_byte, self.read((pointer_address + 1) & 0xFF));
        if crossed {
            effective_address = effective_address.wrapping_add(0x100);
            self.cycle_count += 6
        } else {
            self.cycle_count += 5;
        }
        (
            Operand {
                value: self.read(effective_address),
                address: effective_address,
            },
            crossed,
        )
    }

    fn idy_w(&mut self) -> Operand {
        let (operand, crossed) = self.indirected_indexed();
        if !crossed {
            self.cycle_count += 1;
        }
        operand
    }

    fn rel(&mut self) -> Operand {
        let address = self.pc;
        self.cycle_count += 2;
        self.pc += 1;
        Operand {
            value: self.read(address),
            address,
        }
    }

    fn imp(&mut self) -> Operand {
        Operand {
            value: 0,
            address: 0,
        }
    }
}
