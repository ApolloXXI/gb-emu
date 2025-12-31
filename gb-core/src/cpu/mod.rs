pub mod registers;
pub mod instruction;

use registers::Registers;
use instruction::{Instruction, ArithmeticTarget};
use crate::cpu::instruction::WordTarget16;

// CPU model
#[derive(Default)]
pub struct CPU{
    pub registers: Registers,
    pub program_counter: u16, // Program counter: address of next opcode/operand
    pub stack_pointer: u16, // Stack Pointer: top of stack (grows downward)
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

            Instruction::SUB(target) => {
                let rhs = self.read_target(target);
                self.sub_from_a(rhs)
            }

            Instruction::OR(target) => {
                let rhs = self.read_target(target);
                self.or_with_a(rhs)
            }

            Instruction::XOR(target) => {
                let rhs = self.read_target(target);
                self.xor_with_a(rhs)
            }

            Instruction::CP(target) => {
                let rhs = self.read_target(target);
                self.compare_with_a(rhs)
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
}