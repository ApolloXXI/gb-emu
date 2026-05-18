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

            // ========== LOAD OPERATIONS ==========
            Instruction::LD_A_D16(addr) => {
                self.ld_a_d16(addr);
            }

            Instruction::LD_A_R8(offset) => {
                self.ld_a_r8(offset);
            }

            Instruction::LD_R8_A(offset) => {
                self.ld_r8_a(offset);
            }

            Instruction::LDH_A_D8(val) => {
                self.ldh_a_d8(val);
            }

            Instruction::LDH_D8_A(val) => {
                self.ldh_d8_a(val);
            }

            Instruction::LDH_A_C => {
                self.ldh_a_c();
            }

            Instruction::LDH_C_A => {
                self.ldh_c_a();
            }

            Instruction::LD_SP_HL => {
                self.ld_sp_hl();
            }

            Instruction::LD_HL_SP_D8(offset) => {
                self.ld_hl_sp_d8(offset);
            }

            Instruction::LD_16_16(target, source) => {
                self.ld_16_16(target, source);
            }

            Instruction::LD_A_16(target) => {
                self.ld_a_16(target);
            }

            Instruction::LD_16_A(target) => {
                self.ld_16_a(target);
            }

            Instruction::LD_D16_SP(addr) => {
                self.ld_d16_sp(addr);
            }

            // ========== JUMP/CALL/RETURN OPERATIONS ==========
            Instruction::JP(condition, addr) => {
                return self.jp(condition, addr);
            }

            Instruction::JP_HL => {
                return self.jp_hl();
            }

            Instruction::JR(condition, offset) => {
                return self.jr(condition, offset);
            }

            Instruction::CALL(condition, addr) => {
                return self.call(condition, addr);
            }

            Instruction::RET(condition) => {
                return self.ret(condition);
            }

            Instruction::RETI => {
                return self.reti();
            }

            Instruction::RST(addr) => {
                return self.rst(addr);
            }

            // ========== STACK OPERATIONS ==========
            Instruction::PUSH(target) => {
                self.push(target);
            }

            Instruction::POP(target) => {
                self.pop(target);
            }

            // ========== MISCELLANEOUS ==========
            Instruction::STOP => {
                // Stop CPU until interrupt
            }

            Instruction::DI => {
                self.di();
            }

            Instruction::EI => {
                self.ei();
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

    // ========== LOAD OPERATIONS ==========

    /// LD A, (a16) - Load A from 16-bit immediate address
    pub fn ld_a_d16(&mut self, addr: u16) {
        self.registers.a = self.bus.read_byte(addr);
    }

    /// LD A, (HL + d8) - Load A from memory at HL + signed offset
    pub fn ld_a_r8(&mut self, offset: i8) {
        let hl = self.registers.get_hl();
        let addr = (hl as i16 + offset as i16) as u16;
        self.registers.a = self.bus.read_byte(addr);
    }

    /// LD (HL + d8), A - Store A to memory at HL + signed offset
    pub fn ld_r8_a(&mut self, offset: i8) {
        let hl = self.registers.get_hl();
        let addr = (hl as i16 + offset as i16) as u16;
        self.bus.memory[addr as usize] = self.registers.a;
    }

    /// LDH A, (a8) - Load A from high memory (0xFF00 + immediate)
    pub fn ldh_a_d8(&mut self, val: u8) {
        let addr = 0xFF00u16 + val as u16;
        self.registers.a = self.bus.read_byte(addr);
    }

    /// LDH (a8), A - Store A to high memory (0xFF00 + immediate)
    pub fn ldh_d8_a(&mut self, val: u8) {
        let addr = 0xFF00u16 + val as u16;
        self.bus.memory[addr as usize] = self.registers.a;
    }

    /// LDH A, (C) - Load A from high memory (0xFF00 + C)
    pub fn ldh_a_c(&mut self) {
        let addr = 0xFF00u16 + self.registers.c as u16;
        self.registers.a = self.bus.read_byte(addr);
    }

    /// LDH (C), A - Store A to high memory (0xFF00 + C)
    pub fn ldh_c_a(&mut self) {
        let addr = 0xFF00u16 + self.registers.c as u16;
        self.bus.memory[addr as usize] = self.registers.a;
    }

    /// LD SP, HL - Load SP with value of HL
    pub fn ld_sp_hl(&mut self) {
        self.stack_pointer = self.registers.get_hl();
    }

    /// LD HL, SP + d8 - Load HL with SP + signed immediate
    pub fn ld_hl_sp_d8(&mut self, offset: i8) {
        let sp = self.stack_pointer as i16;
        let result = (sp + offset as i16) as u16;
        
        self.registers.f.subtract = false;
        self.registers.f.zero = false;
        self.registers.f.half_carry = ((sp & 0xF) + (offset as i16 & 0xF)) > 0xF;
        self.registers.f.carry = ((sp as u32) + (offset as i32 as u32)) > 0xFFFF;
        
        self.registers.set_hl(result);
    }

    /// LD rr, d16 / LD SP, d16 - Load 16-bit register pair with immediate
    pub fn ld_16_16(&mut self, target: Load16Target, source: Load16Source) {
        match source {
            Load16Source::Imm16 => {
                // This would need the immediate value passed separately
                // For now, this is a placeholder
            }
            Load16Source::SP => {
                match target {
                    Load16Target::BC => self.registers.set_bc(self.stack_pointer),
                    Load16Target::DE => self.registers.set_de(self.stack_pointer),
                    Load16Target::HL => self.registers.set_hl(self.stack_pointer),
                    Load16Target::SP => {} // LD SP, SP is a no-op
                }
            }
        }
    }

    /// LD A, (rr) - Load A from 16-bit address in register pair
    pub fn ld_a_16(&mut self, target: Load16Target) {
        let addr = match target {
            Load16Target::BC => self.registers.get_bc(),
            Load16Target::DE => self.registers.get_de(),
            Load16Target::HL => self.registers.get_hl(),
            Load16Target::SP => self.stack_pointer,
        };
        self.registers.a = self.bus.read_byte(addr);
    }

    /// LD (rr), A - Store A to 16-bit address in register pair
    pub fn ld_16_a(&mut self, target: Load16Target) {
        let addr = match target {
            Load16Target::BC => self.registers.get_bc(),
            Load16Target::DE => self.registers.get_de(),
            Load16Target::HL => self.registers.get_hl(),
            Load16Target::SP => self.stack_pointer,
        };
        self.bus.memory[addr as usize] = self.registers.a;
    }

    /// LD (a16), SP - Store SP to 16-bit immediate address
    pub fn ld_d16_sp(&mut self, addr: u16) {
        self.bus.memory[addr as usize] = (self.stack_pointer & 0xFF) as u8;
        self.bus.memory[(addr + 1) as usize] = ((self.stack_pointer >> 8) & 0xFF) as u8;
    }

    // ========== JUMP/CALL/RETURN OPERATIONS ==========

    /// Check if jump condition is met
    fn check_condition(&self, condition: JumpCondition) -> bool {
        match condition {
            JumpCondition::NZ => !self.registers.f.zero,
            JumpCondition::Z => self.registers.f.zero,
            JumpCondition::NC => !self.registers.f.carry,
            JumpCondition::C => self.registers.f.carry,
            JumpCondition::None => true,
        }
    }

    /// JP nn / JP cond, nn - Jump to 16-bit address
    pub fn jp(&mut self, condition: JumpCondition, addr: u16) -> Option<u16> {
        if self.check_condition(condition) {
            return Some(addr);
        }
        None
    }

    /// JP HL - Jump to address in HL
    pub fn jp_hl(&mut self) -> Option<u16> {
        Some(self.registers.get_hl())
    }

    /// JR e / JR cond, e - Relative jump
    pub fn jr(&mut self, condition: JumpCondition, offset: i8) -> Option<u16> {
        if self.check_condition(condition) {
            let pc = self.program_counter as i16;
            return Some((pc + offset as i16) as u16);
        }
        None
    }

    /// CALL nn / CALL cond, nn - Call subroutine
    pub fn call(&mut self, condition: JumpCondition, addr: u16) -> Option<u16> {
        if self.check_condition(condition) {
            // Push return address onto stack
            let return_addr = self.program_counter;
            self.stack_pointer = self.stack_pointer.wrapping_sub(1);
            self.bus.memory[self.stack_pointer as usize] = ((return_addr >> 8) & 0xFF) as u8;
            self.stack_pointer = self.stack_pointer.wrapping_sub(1);
            self.bus.memory[self.stack_pointer as usize] = (return_addr & 0xFF) as u8;
            
            return Some(addr);
        }
        None
    }

    /// RET / RET cond - Return from subroutine
    pub fn ret(&mut self, condition: JumpCondition) -> Option<u16> {
        if self.check_condition(condition) {
            // Pop return address from stack
            let lo = self.bus.memory[self.stack_pointer as usize] as u16;
            self.stack_pointer = self.stack_pointer.wrapping_add(1);
            let hi = self.bus.memory[self.stack_pointer as usize] as u16;
            self.stack_pointer = self.stack_pointer.wrapping_add(1);
            
            return Some((hi << 8) | lo);
        }
        None
    }

    /// RETI - Return from interrupt
    pub fn reti(&mut self) -> Option<u16> {
        // Pop return address from stack
        let lo = self.bus.memory[self.stack_pointer as usize] as u16;
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let hi = self.bus.memory[self.stack_pointer as usize] as u16;
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        
        // Enable interrupts (would need interrupt flag in real implementation)
        
        Some((hi << 8) | lo)
    }

    /// RST t - Restart (call to fixed addresses)
    pub fn rst(&mut self, addr: u16) -> Option<u16> {
        // Push return address onto stack
        let return_addr = self.program_counter;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.bus.memory[self.stack_pointer as usize] = ((return_addr >> 8) & 0xFF) as u8;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.bus.memory[self.stack_pointer as usize] = (return_addr & 0xFF) as u8;
        
        Some(addr)
    }

    // ========== STACK OPERATIONS ==========

    /// PUSH rr - Push 16-bit register pair onto stack
    pub fn push(&mut self, target: IncDecTarget16) {
        let val = match target {
            IncDecTarget16::BC => self.registers.get_bc(),
            IncDecTarget16::DE => self.registers.get_de(),
            IncDecTarget16::HL => self.registers.get_hl(),
            IncDecTarget16::AF => self.registers.get_af(),
        };
        
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.bus.memory[self.stack_pointer as usize] = ((val >> 8) & 0xFF) as u8;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.bus.memory[self.stack_pointer as usize] = (val & 0xFF) as u8;
    }

    /// POP rr - Pop 16-bit register pair from stack
    pub fn pop(&mut self, target: IncDecTarget16) {
        let lo = self.bus.memory[self.stack_pointer as usize] as u16;
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let hi = self.bus.memory[self.stack_pointer as usize] as u16;
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        
        let val = (hi << 8) | lo;
        
        match target {
            IncDecTarget16::BC => self.registers.set_bc(val),
            IncDecTarget16::DE => self.registers.set_de(val),
            IncDecTarget16::HL => self.registers.set_hl(val),
            IncDecTarget16::AF => self.registers.set_af(val),
        }
    }

    // ========== MISCELLANEOUS ==========

    /// DI - Disable interrupts
    pub fn di(&mut self) {
        // In a full implementation, this would set an interrupt enable flag
        // For now, it's a no-op
    }

    /// EI - Enable interrupts
    pub fn ei(&mut self) {
        // In a full implementation, this would set an interrupt enable flag
        // For now, it's a no-op
    }
}