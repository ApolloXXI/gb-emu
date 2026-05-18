pub mod registers;
pub mod instruction;

use registers::Registers;
use instruction::{Instruction, ArithmeticTarget, JumpCondition, Load16Target, Load16Source, IncDecTarget16};
use instruction::WordTarget16;

const MEMORY_SIZE: usize = 0x10000;
// CPU model
#[derive(Default)]
pub struct CPU{
    pub registers: Registers,
    pub program_counter: u16, // Program counter: address of next opcode/operand
    pub stack_pointer: u16, // Stack Pointer: top of stack (grows downward)
    pub bus: MemoryBus,
}

pub struct MemoryBus{
    memory :[u8; MEMORY_SIZE]
}
impl Default for MemoryBus {
    fn default() -> Self {
        Self {
            memory: [0; MEMORY_SIZE]
        }
    }
}
impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

impl CPU{
    /// Constructor
    /// Returns a CPU with default-initialised registers (0)
    /// Self is an alias for CPU
    pub fn new() -> Self{
        Self { 
            registers: Registers:: default(),
            program_counter: 0x0000,
            stack_pointer: 0xFFFE,
            bus: MemoryBus::default(),

        }
    }

    /// Decoding and executing one instruction
    /// &mut self: executing an instruction changes CPU state, a mutable borrow is needed
    /// Pattern match on the decoded instruction
    /// Currently only handles Instruction :: Add(target)
    pub fn execute(&mut self, instruction: Instruction){
        match instruction {
            Instruction::ADD(target) => {
                let rhs = self.read_target(target);
                self.add_into_a(rhs);
            }

            Instruction::ADD_A_D8(val) => {
                self.add_into_a(val);
            }

            Instruction::SUB(target) => {
                let rhs = self.read_target(target);
                self.sub_from_a(rhs)
            }

            Instruction::SUB_D8(val) => {
                self.sub_from_a(val);
            }

            Instruction::AND(target) => {
                let rhs = self.read_target(target);
                self.and_from_a(rhs)
            }

            Instruction::AND_D8(val) => {
                self.and_from_a(val);
            }

            Instruction::OR(target) => {
                let rhs = self.read_target(target);
                self.or_with_a(rhs)
            }

            Instruction::OR_D8(val) => {
                self.or_with_a(val);
            }

            Instruction::XOR(target) => {
                let rhs = self.read_target(target);
                self.xor_with_a(rhs)
            }

            Instruction::XOR_D8(val) => {
                self.xor_with_a(val);
            }

            Instruction::CP(target) => {
                let rhs = self.read_target(target);
                self.compare_with_a(rhs)
            }

            Instruction::CP_D8(val) => {
                self.compare_with_a(val);
            }

            Instruction::ADC(target) => {
                let rhs = self.read_target(target);
                self.adc_into_a(rhs);
            }

            Instruction::ADC_A_D8(val) => {
                self.adc_into_a(val);
            }

            Instruction::SBC(target) => {
                let rhs = self.read_target(target);
                self.sbc_from_a(rhs);
            }

            Instruction::SBC_A_D8(val) => {
                self.sbc_from_a(val);
            }

            Instruction::ADDHL(target) => {
                let rhs = self.read_target_16(target);
                self.add_into_hl(rhs);
            }

            Instruction::INC(target) => {
                let val = self.read_target(target);
                self.inc_8(val, target);
            }

            Instruction::DEC(target) => {
                let val = self.read_target(target);
                self.dec_8(val, target);
            }

            Instruction::INC_16(target) => {
                self.inc_16(target);
            }

            Instruction::DEC_16(target) => {
                self.dec_16(target);
            }

            Instruction::RLCA => {
                self.rlca();
            }

            Instruction::RRCA => {
                self.rrca();
            }

            Instruction::RLA => {
                self.rla();
            }

            Instruction::RRA => {
                self.rra();
            }

            Instruction::CPL => {
                self.cpl();
            }

            Instruction::CCF => {
                self.ccf();
            }

            Instruction::SCF => {
                self.scf();
            }

            Instruction::DAA => {
                self.daa();
            }

            Instruction::HALT => {
                // Halt the CPU until interrupt
            }

            Instruction::NOP => {
                // No operation
            }

            _ => {
                // You can leave this empty or print a message for debugging
                // println!("Instruction not implemented yet: {:?}", instruction);
            }
        }
    }

    /// Reading an operand from a register
    /// Pure read-only helper (&self) that maps an ArithmeticTarget to a corresponding 8-bit register value
    /// Returns u8 in that register
    pub fn read_target(&self, t: ArithmeticTarget) -> u8{
        match t{
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
        }
    }

    /// ALU addition and flag updates
    /// &mut self because we modify A and flags(F)
    /// a is the current value of the accumulator
    /// sum does the math in u16 to detect a carry
    /// result is the low 8 bits of the sum
    pub fn add_into_a(&mut self, rhs: u8){
        let a = self.registers.a;
        let sum = (a as u16) + (rhs as u16);
        let result = sum as u8;

        self.registers.f.zero   = result == 0;  // set if result = 0
        self.registers.f.subtract = false;  // cleared for addition
        self.registers.f.half_carry = ((a & 0x0F) + (rhs & 0x0F)) > 0x0F;   // set if there's a carry from bit 3 to 4
        self.registers.f.carry = sum > 0xFF;    // set if the full 8-bit addition overflowed

        self.registers.a = result;
    }

    fn read_target_16(&self, target: WordTarget16) -> u16{
        match target{
            WordTarget16::BC => self.registers.get_bc(),
            WordTarget16::DE => self.registers.get_de(),
            WordTarget16::HL => self.registers.get_hl(),
            WordTarget16::SP => self.stack_pointer,
        }
    }

    fn add_into_hl(&mut self, rhs: u16){
        let hl = self.registers.get_hl();

        let sum = (hl as u32) + (rhs as u32);
        let result = sum as u16;

        self.registers.f.subtract = false;

        self.registers.f.half_carry = ((hl & 0x0FFF) + (rhs & 0x0FFF)) > 0x0FFF;

        self.registers.f.carry = sum > 0xFFFF;

        self.registers.set_hl(result);
    }

    pub fn sub_from_a(&mut self, rhs: u8) {
        let a = self.registers.a;
        let result = a.wrapping_sub(rhs);

        self.registers.f.zero   = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (a & 0x0F) < (rhs & 0x0F);
        self.registers.f.carry = a < rhs;
    }

    pub fn and_from_a(&mut self, rhs: u8) {
        self.registers.a &= rhs;

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
    }

    pub fn compare_with_a(&mut self, rhs: u8){
        let a = self.registers.a;
        let result = a.wrapping_sub(rhs);

        self.registers.f.zero   = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (a & 0x0F) < (rhs & 0x0F);
        self.registers.f.carry = a < rhs;
    }

    pub fn or_with_a(&mut self, rhs: u8){
        self.registers.a |= rhs;

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

    }

    pub fn xor_with_a(&mut self, rhs: u8){
        self.registers.a ^= rhs;

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }

    // ========== ADDITIONAL ALU OPERATIONS ==========

    /// Add with carry into A
    pub fn adc_into_a(&mut self, rhs: u8) {
        let a = self.registers.a;
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let sum = (a as u16) + (rhs as u16) + (carry as u16);
        let result = sum as u8;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = ((a & 0x0F) + (rhs & 0x0F) + carry) > 0x0F;
        self.registers.f.carry = sum > 0xFF;

        self.registers.a = result;
    }

    /// Subtract with carry from A
    pub fn sbc_from_a(&mut self, rhs: u8) {
        let a = self.registers.a;
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let result = a.wrapping_sub(rhs).wrapping_sub(carry);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (a & 0x0F) < (rhs & 0x0F) + carry;
        self.registers.f.carry = (a as u16) < (rhs as u16) + (carry as u16);

        self.registers.a = result;
    }

    /// Increment 8-bit register
    pub fn inc_8(&mut self, val: u8, target: ArithmeticTarget) {
        let result = val.wrapping_add(1);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (val & 0x0F) == 0x0F;

        match target {
            ArithmeticTarget::A => self.registers.a = result,
            ArithmeticTarget::B => self.registers.b = result,
            ArithmeticTarget::C => self.registers.c = result,
            ArithmeticTarget::D => self.registers.d = result,
            ArithmeticTarget::E => self.registers.e = result,
            ArithmeticTarget::H => self.registers.h = result,
            ArithmeticTarget::L => self.registers.l = result,
        }
    }

    /// Decrement 8-bit register
    pub fn dec_8(&mut self, val: u8, target: ArithmeticTarget) {
        let result = val.wrapping_sub(1);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (val & 0x0F) == 0x00;

        match target {
            ArithmeticTarget::A => self.registers.a = result,
            ArithmeticTarget::B => self.registers.b = result,
            ArithmeticTarget::C => self.registers.c = result,
            ArithmeticTarget::D => self.registers.d = result,
            ArithmeticTarget::E => self.registers.e = result,
            ArithmeticTarget::H => self.registers.h = result,
            ArithmeticTarget::L => self.registers.l = result,
        }
    }

    /// Increment 16-bit register pair
    pub fn inc_16(&mut self, target: IncDecTarget16) {
        let val = match target {
            IncDecTarget16::BC => self.registers.get_bc().wrapping_add(1),
            IncDecTarget16::DE => self.registers.get_de().wrapping_add(1),
            IncDecTarget16::HL => self.registers.get_hl().wrapping_add(1),
            IncDecTarget16::AF => self.registers.get_af().wrapping_add(1),
        };
        match target {
            IncDecTarget16::BC => self.registers.set_bc(val),
            IncDecTarget16::DE => self.registers.set_de(val),
            IncDecTarget16::HL => self.registers.set_hl(val),
            IncDecTarget16::AF => self.registers.set_af(val),
        }
    }

    /// Decrement 16-bit register pair
    pub fn dec_16(&mut self, target: IncDecTarget16) {
        let val = match target {
            IncDecTarget16::BC => self.registers.get_bc().wrapping_sub(1),
            IncDecTarget16::DE => self.registers.get_de().wrapping_sub(1),
            IncDecTarget16::HL => self.registers.get_hl().wrapping_sub(1),
            IncDecTarget16::AF => self.registers.get_af().wrapping_sub(1),
        };
        match target {
            IncDecTarget16::BC => self.registers.set_bc(val),
            IncDecTarget16::DE => self.registers.set_de(val),
            IncDecTarget16::HL => self.registers.set_hl(val),
            IncDecTarget16::AF => self.registers.set_af(val),
        }
    }

    // ========== ROTATE OPERATIONS ==========

    /// Rotate Left Circular A (not through carry)
    pub fn rlca(&mut self) {
        let bit7 = (self.registers.a >> 7) & 1;
        self.registers.a = (self.registers.a << 1) | bit7;
        self.registers.f.carry = bit7 == 1;
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    /// Rotate Right Circular A (not through carry)
    pub fn rrca(&mut self) {
        let bit0 = self.registers.a & 1;
        self.registers.a = (self.registers.a >> 1) | (bit0 << 7);
        self.registers.f.carry = bit0 == 1;
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    /// Rotate Left A through carry
    pub fn rla(&mut self) {
        let old_carry = if self.registers.f.carry { 1 } else { 0 };
        let bit7 = (self.registers.a >> 7) & 1;
        self.registers.a = (self.registers.a << 1) | old_carry;
        self.registers.f.carry = bit7 == 1;
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    /// Rotate Right A through carry
    pub fn rra(&mut self) {
        let old_carry = if self.registers.f.carry { 1 } else { 0 };
        let bit0 = self.registers.a & 1;
        self.registers.a = (self.registers.a >> 1) | (old_carry << 7);
        self.registers.f.carry = bit0 == 1;
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    // ========== FLAG OPERATIONS ==========

    /// Complement A
    pub fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
    }

    /// Complement Carry Flag
    pub fn ccf(&mut self) {
        self.registers.f.carry = !self.registers.f.carry;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    /// Set Carry Flag
    pub fn scf(&mut self) {
        self.registers.f.carry = true;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    /// Decimal Adjust Accumulator (for BCD arithmetic)
    pub fn daa(&mut self) {
        let mut a = self.registers.a as u16;
        let mut adjust = 0u16;

        if self.registers.f.half_carry || (!self.registers.f.subtract && (a & 0x0F) > 9) {
            adjust |= 0x06;
        }

        if self.registers.f.carry || (!self.registers.f.subtract && a > 0x99) {
            adjust |= 0x60;
            self.registers.f.carry = true;
        }

        if self.registers.f.subtract {
            a = (a - adjust) & 0xFF;
        } else {
            a = (a + adjust) & 0xFF;
        }

        self.registers.a = a as u8;
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.half_carry = false;
    }
}