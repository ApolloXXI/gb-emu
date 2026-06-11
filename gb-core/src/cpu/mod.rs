pub mod registers;
pub mod instruction;

use registers::Registers;
use instruction::{
    ArithmeticTarget, IncDecTarget, Instruction, JumpTest,
    LoadByteSource, LoadByteTarget, LoadType, WordTarget16,
};

const MEMORY_SIZE: usize = 0x10000;

#[derive(Default)]
pub struct CPU {
    pub registers: Registers,
    pub program_counter: u16,
    pub stack_pointer: u16,
    pub bus: MemoryBus,
    pub halted: bool,
}

pub struct MemoryBus {
    memory: [u8; MEMORY_SIZE],
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self { memory: [0; MEMORY_SIZE] }
    }
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn load(&mut self, start: u16, data: &[u8]) {
        let start = start as usize;
        self.memory[start..start + data.len()].copy_from_slice(data);
    }
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::default(),
            program_counter: 0x0000,
            stack_pointer: 0xFFFE,
            bus: MemoryBus::default(),
            halted: false,
        }
    }

    /// Fetch, decode, and execute one instruction, advancing the program counter.
    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        let mut instruction_byte = self.bus.read_byte(self.program_counter);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.program_counter.wrapping_add(1));
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let tag = if prefixed { "cb" } else { "" };
            panic!("Unknown instruction: 0x{}{:02x}", tag, instruction_byte);
        };

        self.program_counter = next_pc;
    }

    /// Execute a decoded instruction and return the next program counter value.
    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            // ── Arithmetic ───────────────────────────────────────────────────
            Instruction::ADD(target) => {
                let rhs = self.read_target(target);
                self.add_into_a(rhs);
                self.program_counter.wrapping_add(1)
            }
            Instruction::ADC(target) => {
                let rhs = self.read_target(target);
                self.adc_into_a(rhs);
                self.program_counter.wrapping_add(1)
            }
            Instruction::SUB(target) => {
                let rhs = self.read_target(target);
                self.sub_from_a(rhs);
                self.program_counter.wrapping_add(1)
            }
            Instruction::SBC(target) => {
                let rhs = self.read_target(target);
                self.sbc_from_a(rhs);
                self.program_counter.wrapping_add(1)
            }
            Instruction::AND(target) => {
                let rhs = self.read_target(target);
                self.and_into_a(rhs);
                self.program_counter.wrapping_add(1)
            }
            Instruction::OR(target) => {
                let rhs = self.read_target(target);
                self.or_into_a(rhs);
                self.program_counter.wrapping_add(1)
            }
            Instruction::XOR(target) => {
                let rhs = self.read_target(target);
                self.xor_into_a(rhs);
                self.program_counter.wrapping_add(1)
            }
            Instruction::CP(target) => {
                let rhs = self.read_target(target);
                self.compare_with_a(rhs);
                self.program_counter.wrapping_add(1)
            }
            Instruction::ADDHL(target) => {
                let rhs = self.read_target_16(target);
                self.add_into_hl(rhs);
                self.program_counter.wrapping_add(1)
            }

            // ── Inc / Dec ────────────────────────────────────────────────────
            Instruction::INC(target) => {
                self.inc(target);
                self.program_counter.wrapping_add(1)
            }
            Instruction::DEC(target) => {
                self.dec(target);
                self.program_counter.wrapping_add(1)
            }

            // ── Flag ops ─────────────────────────────────────────────────────
            Instruction::CCF => {
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = !self.registers.f.carry;
                self.program_counter.wrapping_add(1)
            }
            Instruction::SCF => {
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = true;
                self.program_counter.wrapping_add(1)
            }
            Instruction::CPL => {
                self.registers.a = !self.registers.a;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = true;
                self.program_counter.wrapping_add(1)
            }

            // ── Rotate A (non-CB, zero flag always cleared) ──────────────────
            Instruction::RLCA => {
                let a = self.registers.a;
                let bit7 = a >> 7;
                self.registers.a = (a << 1) | bit7;
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = bit7 != 0;
                self.program_counter.wrapping_add(1)
            }
            Instruction::RRCA => {
                let a = self.registers.a;
                let bit0 = a & 1;
                self.registers.a = (a >> 1) | (bit0 << 7);
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = bit0 != 0;
                self.program_counter.wrapping_add(1)
            }
            Instruction::RLA => {
                let a = self.registers.a;
                let old_carry = self.registers.f.carry as u8;
                let bit7 = a >> 7;
                self.registers.a = (a << 1) | old_carry;
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = bit7 != 0;
                self.program_counter.wrapping_add(1)
            }
            Instruction::RRA => {
                let a = self.registers.a;
                let old_carry = self.registers.f.carry as u8;
                let bit0 = a & 1;
                self.registers.a = (a >> 1) | (old_carry << 7);
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = bit0 != 0;
                self.program_counter.wrapping_add(1)
            }

            // ── CB-prefixed rotate / shift (PC +2 because 0xCB + opcode) ────
            Instruction::RLC(target) => {
                let v = self.read_target(target);
                let r = self.rlc(v);
                self.write_target(target, r);
                self.program_counter.wrapping_add(2)
            }
            Instruction::RRC(target) => {
                let v = self.read_target(target);
                let r = self.rrc(v);
                self.write_target(target, r);
                self.program_counter.wrapping_add(2)
            }
            Instruction::RL(target) => {
                let v = self.read_target(target);
                let r = self.rl(v);
                self.write_target(target, r);
                self.program_counter.wrapping_add(2)
            }
            Instruction::RR(target) => {
                let v = self.read_target(target);
                let r = self.rr(v);
                self.write_target(target, r);
                self.program_counter.wrapping_add(2)
            }
            Instruction::SLA(target) => {
                let v = self.read_target(target);
                let r = self.sla(v);
                self.write_target(target, r);
                self.program_counter.wrapping_add(2)
            }
            Instruction::SRA(target) => {
                let v = self.read_target(target);
                let r = self.sra(v);
                self.write_target(target, r);
                self.program_counter.wrapping_add(2)
            }
            Instruction::SRL(target) => {
                let v = self.read_target(target);
                let r = self.srl(v);
                self.write_target(target, r);
                self.program_counter.wrapping_add(2)
            }
            Instruction::SWAP(target) => {
                let v = self.read_target(target);
                let r = self.swap(v);
                self.write_target(target, r);
                self.program_counter.wrapping_add(2)
            }

            // ── CB-prefixed bit ops ──────────────────────────────────────────
            Instruction::BIT { bit, target } => {
                let v = self.read_target(target);
                self.registers.f.zero = (v >> bit) & 1 == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = true;
                self.program_counter.wrapping_add(2)
            }
            Instruction::RES { bit, target } => {
                let v = self.read_target(target);
                self.write_target(target, v & !(1 << bit));
                self.program_counter.wrapping_add(2)
            }
            Instruction::SET { bit, target } => {
                let v = self.read_target(target);
                self.write_target(target, v | (1 << bit));
                self.program_counter.wrapping_add(2)
            }

            // ── Loads ────────────────────────────────────────────────────────
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let value = match source {
                            LoadByteSource::A   => self.registers.a,
                            LoadByteSource::B   => self.registers.b,
                            LoadByteSource::C   => self.registers.c,
                            LoadByteSource::D   => self.registers.d,
                            LoadByteSource::E   => self.registers.e,
                            LoadByteSource::H   => self.registers.h,
                            LoadByteSource::L   => self.registers.l,
                            LoadByteSource::D8  => self.read_next_byte(),
                            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                        };
                        match target {
                            LoadByteTarget::A   => self.registers.a = value,
                            LoadByteTarget::B   => self.registers.b = value,
                            LoadByteTarget::C   => self.registers.c = value,
                            LoadByteTarget::D   => self.registers.d = value,
                            LoadByteTarget::E   => self.registers.e = value,
                            LoadByteTarget::H   => self.registers.h = value,
                            LoadByteTarget::L   => self.registers.l = value,
                            LoadByteTarget::HLI => {
                                let addr = self.registers.get_hl();
                                self.bus.write_byte(addr, value);
                            }
                        }
                        match source {
                            LoadByteSource::D8 => self.program_counter.wrapping_add(2),
                            _                  => self.program_counter.wrapping_add(1),
                        }
                    }
                }
            }

            // ── Jumps ────────────────────────────────────────────────────────
            Instruction::JP(test) => {
                let should_jump = match test {
                    JumpTest::NotZero  => !self.registers.f.zero,
                    JumpTest::Zero     =>  self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry    =>  self.registers.f.carry,
                    JumpTest::Always   =>  true,
                };
                self.jump(should_jump)
            }

            // ── Control ──────────────────────────────────────────────────────
            Instruction::NOP => self.program_counter.wrapping_add(1),
            Instruction::HALT => {
                self.halted = true;
                self.program_counter.wrapping_add(1)
            }
        }
    }

    // ── Register helpers ─────────────────────────────────────────────────────

    pub fn read_target(&self, t: ArithmeticTarget) -> u8 {
        match t {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
        }
    }

    fn write_target(&mut self, t: ArithmeticTarget, value: u8) {
        match t {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
        }
    }

    fn read_target_16(&self, target: WordTarget16) -> u16 {
        match target {
            WordTarget16::BC => self.registers.get_bc(),
            WordTarget16::DE => self.registers.get_de(),
            WordTarget16::HL => self.registers.get_hl(),
            WordTarget16::SP => self.stack_pointer,
        }
    }

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.program_counter.wrapping_add(1))
    }

    // ── ALU ──────────────────────────────────────────────────────────────────

    fn add_into_a(&mut self, rhs: u8) {
        let a = self.registers.a;
        let sum = (a as u16) + (rhs as u16);
        let result = sum as u8;
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = (a & 0x0F) + (rhs & 0x0F) > 0x0F;
        self.registers.f.carry      = sum > 0xFF;
        self.registers.a = result;
    }

    fn adc_into_a(&mut self, rhs: u8) {
        let a = self.registers.a;
        let c = self.registers.f.carry as u8;
        let sum = (a as u16) + (rhs as u16) + (c as u16);
        let result = sum as u8;
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = (a & 0x0F) + (rhs & 0x0F) + c > 0x0F;
        self.registers.f.carry      = sum > 0xFF;
        self.registers.a = result;
    }

    fn sub_from_a(&mut self, rhs: u8) {
        let a = self.registers.a;
        let result = a.wrapping_sub(rhs);
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = true;
        self.registers.f.half_carry = (a & 0x0F) < (rhs & 0x0F);
        self.registers.f.carry      = a < rhs;
        self.registers.a = result;
    }

    fn sbc_from_a(&mut self, rhs: u8) {
        let a = self.registers.a;
        let c = self.registers.f.carry as u8;
        let result = a.wrapping_sub(rhs).wrapping_sub(c);
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = true;
        self.registers.f.half_carry = (a & 0x0F) < (rhs & 0x0F) + c;
        self.registers.f.carry      = (a as u16) < (rhs as u16) + (c as u16);
        self.registers.a = result;
    }

    fn and_into_a(&mut self, rhs: u8) {
        self.registers.a &= rhs;
        self.registers.f.zero       = self.registers.a == 0;
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry      = false;
    }

    fn or_into_a(&mut self, rhs: u8) {
        self.registers.a |= rhs;
        self.registers.f.zero       = self.registers.a == 0;
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry      = false;
    }

    fn xor_into_a(&mut self, rhs: u8) {
        self.registers.a ^= rhs;
        self.registers.f.zero       = self.registers.a == 0;
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry      = false;
    }

    fn compare_with_a(&mut self, rhs: u8) {
        let a = self.registers.a;
        let result = a.wrapping_sub(rhs);
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = true;
        self.registers.f.half_carry = (a & 0x0F) < (rhs & 0x0F);
        self.registers.f.carry      = a < rhs;
    }

    fn add_into_hl(&mut self, rhs: u16) {
        let hl = self.registers.get_hl();
        let sum = (hl as u32) + (rhs as u32);
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = (hl & 0x0FFF) + (rhs & 0x0FFF) > 0x0FFF;
        self.registers.f.carry      = sum > 0xFFFF;
        self.registers.set_hl(sum as u16);
    }

    // ── INC / DEC ────────────────────────────────────────────────────────────

    fn inc(&mut self, target: IncDecTarget) {
        match target {
            IncDecTarget::A  => { let v = self.inc8(self.registers.a);  self.registers.a = v; }
            IncDecTarget::B  => { let v = self.inc8(self.registers.b);  self.registers.b = v; }
            IncDecTarget::C  => { let v = self.inc8(self.registers.c);  self.registers.c = v; }
            IncDecTarget::D  => { let v = self.inc8(self.registers.d);  self.registers.d = v; }
            IncDecTarget::E  => { let v = self.inc8(self.registers.e);  self.registers.e = v; }
            IncDecTarget::H  => { let v = self.inc8(self.registers.h);  self.registers.h = v; }
            IncDecTarget::L  => { let v = self.inc8(self.registers.l);  self.registers.l = v; }
            IncDecTarget::BC => { let v = self.registers.get_bc().wrapping_add(1); self.registers.set_bc(v); }
            IncDecTarget::DE => { let v = self.registers.get_de().wrapping_add(1); self.registers.set_de(v); }
            IncDecTarget::HL => { let v = self.registers.get_hl().wrapping_add(1); self.registers.set_hl(v); }
            IncDecTarget::SP => { self.stack_pointer = self.stack_pointer.wrapping_add(1); }
        }
    }

    fn dec(&mut self, target: IncDecTarget) {
        match target {
            IncDecTarget::A  => { let v = self.dec8(self.registers.a);  self.registers.a = v; }
            IncDecTarget::B  => { let v = self.dec8(self.registers.b);  self.registers.b = v; }
            IncDecTarget::C  => { let v = self.dec8(self.registers.c);  self.registers.c = v; }
            IncDecTarget::D  => { let v = self.dec8(self.registers.d);  self.registers.d = v; }
            IncDecTarget::E  => { let v = self.dec8(self.registers.e);  self.registers.e = v; }
            IncDecTarget::H  => { let v = self.dec8(self.registers.h);  self.registers.h = v; }
            IncDecTarget::L  => { let v = self.dec8(self.registers.l);  self.registers.l = v; }
            IncDecTarget::BC => { let v = self.registers.get_bc().wrapping_sub(1); self.registers.set_bc(v); }
            IncDecTarget::DE => { let v = self.registers.get_de().wrapping_sub(1); self.registers.set_de(v); }
            IncDecTarget::HL => { let v = self.registers.get_hl().wrapping_sub(1); self.registers.set_hl(v); }
            IncDecTarget::SP => { self.stack_pointer = self.stack_pointer.wrapping_sub(1); }
        }
    }

    fn inc8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = (value & 0x0F) + 1 > 0x0F;
        result
    }

    fn dec8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = true;
        self.registers.f.half_carry = (value & 0x0F) == 0;
        result
    }

    // ── Rotate / shift helpers (CB-prefixed; zero flag set from result) ───────

    fn rlc(&mut self, value: u8) -> u8 {
        let bit7 = value >> 7;
        let result = (value << 1) | bit7;
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry      = bit7 != 0;
        result
    }

    fn rrc(&mut self, value: u8) -> u8 {
        let bit0 = value & 1;
        let result = (value >> 1) | (bit0 << 7);
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry      = bit0 != 0;
        result
    }

    fn rl(&mut self, value: u8) -> u8 {
        let old_carry = self.registers.f.carry as u8;
        let bit7 = value >> 7;
        let result = (value << 1) | old_carry;
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry      = bit7 != 0;
        result
    }

    fn rr(&mut self, value: u8) -> u8 {
        let old_carry = self.registers.f.carry as u8;
        let bit0 = value & 1;
        let result = (value >> 1) | (old_carry << 7);
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry      = bit0 != 0;
        result
    }

    fn sla(&mut self, value: u8) -> u8 {
        let bit7 = value >> 7;
        let result = value << 1;
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry      = bit7 != 0;
        result
    }

    fn sra(&mut self, value: u8) -> u8 {
        let bit0 = value & 1;
        let result = (value >> 1) | (value & 0x80);
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry      = bit0 != 0;
        result
    }

    fn srl(&mut self, value: u8) -> u8 {
        let bit0 = value & 1;
        let result = value >> 1;
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry      = bit0 != 0;
        result
    }

    fn swap(&mut self, value: u8) -> u8 {
        let result = ((value & 0x0F) << 4) | ((value & 0xF0) >> 4);
        self.registers.f.zero       = result == 0;
        self.registers.f.subtract   = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry      = false;
        result
    }

    // ── Jump helper ──────────────────────────────────────────────────────────

    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            // JP target is stored little-endian in the two bytes after the opcode
            let lo = self.bus.read_byte(self.program_counter.wrapping_add(1)) as u16;
            let hi = self.bus.read_byte(self.program_counter.wrapping_add(2)) as u16;
            (hi << 8) | lo
        } else {
            // JP is 3 bytes wide even when not taken
            self.program_counter.wrapping_add(3)
        }
    }
}
