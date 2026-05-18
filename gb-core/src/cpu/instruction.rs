use crate::cpu::registers::Registers;

// Central place where all instructions are defined
pub enum Instruction{
    // ========== ARITHMETIC OPERATIONS ==========

    // ADD instruction: Adds value from source register to accumulator (A register)
    // Example: ADD B means A = A + B
    // Affects flags: Z (zero), N (reset), H (half-carry), C (carry)

    ADD(ArithmeticTarget), // add register r to A
    ADD_A_D8(u8), // add immediate 8-bit value to A
    SUB(ArithmeticTarget), // subtract the value in register r with the value in register A
    SUB_D8(u8), // subtract immediate 8-bit value from A
    SBC(ArithmeticTarget), // subtract with carry. The value of the carry flag is also subtracted from the number
    SBC_A_D8(u8), // subtract immediate with carry from A
    AND(ArithmeticTarget), // && with reg A
    AND_D8(u8), // AND immediate with A
    OR(ArithmeticTarget), // || with reg A
    OR_D8(u8), // OR immediate with A
    XOR(ArithmeticTarget), // XOR reg A
    XOR_D8(u8), // XOR immediate with A
    CP(ArithmeticTarget), // (Compare) just like SUB except the result is not stored
    CP_D8(u8), // compare immediate with A

    ADDHL(WordTarget16), // just like ADD. Add register r to HL
    ADD_SP_D8(i8), // add signed 8-bit immediate to SP
    ADC(ArithmeticTarget), // add with carry. The value of the carry flag is also added to the number
    ADC_A_D8(u8), // add immediate with carry to A

    // ========== INCREMENT/DECREMENT OPERATIONS ==========
    INC(ArithmeticTarget), // increment the value of a register with 1
    INC_16(IncDecTarget16), // increment 16-bit register pair
    DEC(ArithmeticTarget), // vice versa
    DEC_16(IncDecTarget16), // decrement 16-bit register pair

    // ========== FLAG OPERATIONS ==========

    CCF, // (complement carry flag) - toggle the value of the carry flag
    SCF, // set the carry flag to true
    CPL, // (Complement) literally a complement
    DAA, // decimal adjust accumulator (for BCD arithmetic)

    // ========== ROTATE OPERATIONS (A REGISTER ONLY) ==========

    RLA, // bit rotate A register left through the carry flag
    RRA, // bit rotate right
    RLCA, // rotate left (not through the carry flag)
    RRCA, // rotate right (not through the carry flag)

    // ========== BIT MANIPULATION OPERATIONS ==========

    BIT {bit: u8, target: ArithmeticTarget}, // test to see if a specific bit of a specific register is set
    RES { bit: u8, target: ArithmeticTarget }, // set a specific bit of a specific register to 0
    SET { bit: u8, target: ArithmeticTarget }, // set a specific bit of a specific register to 1
    SRL(ArithmeticTarget), // bit shift a specific register right by 1
    SRA(ArithmeticTarget), // arithmetic shift a specific register right by 1

    // ========== ROTATE OPERATIONS (ANY REGISTER) ==========
    RR(ArithmeticTarget), // bit rotate a specific register right by 1 through the carry flag
    RL(ArithmeticTarget), // bit rotate a specific register left by 1 through the carry flag
    RRC(ArithmeticTarget), // bit rotate a specific register right by 1 (not through the carry flag)
    RLC(ArithmeticTarget), // bit rotate a specific register left by 1 (not through the carry flag)
    SWAP(ArithmeticTarget), // switch upper and lower nibble of a specific register
    SLA(ArithmeticTarget), // arithmetic shift a specific register left by 1

    // ========== LOAD OPERATIONS ==========
    LD_A_D16(u16), // load from 16-bit immediate address into A
    LD_A_R8(i8), // load A from memory at address (HL + signed offset)
    LD_R8_A(i8), // store A to memory at address (HL + signed offset)
    LDH_A_D8(u8), // load A from high memory (0xFF00 + immediate)
    LDH_D8_A(u8), // load A to high memory (0xFF00 + immediate)
    LDH_A_C, // load A from high memory (0xFF00 + C)
    LDH_C_A, // load A to high memory (0xFF00 + C)
    LD_SP_HL, // load SP with value of HL
    LD_HL_SP_D8(i8), // load HL with SP + signed immediate
    LD_16_16(Load16Target, Load16Source), // load 16-bit register pairs
    LD_A_16(Load16Target), // load A from 16-bit address in register pair
    LD_16_A(Load16Target), // store A to 16-bit address in register pair
    LD_D16_SP(u16), // store SP to 16-bit immediate address

    // ========== JUMP/CALL/RETURN OPERATIONS ==========
    JP(JumpCondition, u16), // unconditional or conditional jump
    JP_HL, // jump to address in HL
    JR(JumpCondition, i8), // relative jump
    CALL(JumpCondition, u16), // call subroutine
    RET(JumpCondition), // return from subroutine
    RETI, // return from interrupt
    RST(u16), // restart (call to fixed addresses)

    // ========== STACK OPERATIONS ==========
    PUSH(IncDecTarget16), // push 16-bit register pair onto stack
    POP(IncDecTarget16), // pop 16-bit register pair from stack

    // ========== MISCELLANEOUS ==========
    NOP,
    HALT,
    STOP,
    DI, // disable interrupts
    EI, // enable interrupts
    

}

/// Which 8-bit register is the source operand
/// F can't be targeted
#[derive(Copy, Clone)]
pub enum ArithmeticTarget{
    A, B, C, D, E, H, L
}

#[derive(Copy, Clone, Debug)]
pub enum WordTarget16{
    BC, DE, HL, SP // for ADD HL, rr
}

#[derive(Copy, Clone, Debug)]
pub enum IncDecTarget{
    A, B, C, D, E, H, L, BC, DE, HL, SP
}
/// 16-bit register pair targets for LD instructions
#[derive(Copy, Clone, Debug)]
pub enum Load16Target {
    BC, DE, HL, SP
}

/// Source for 16-bit load operations
#[derive(Copy, Clone, Debug)]
pub enum Load16Source {
    Imm16,  // immediate 16-bit value
    SP,     // stack pointer
}

/// Jump condition for conditional jumps/calls/returns
#[derive(Copy, Clone, Debug)]
pub enum JumpCondition {
    NZ, // not zero (Z flag = 0)
    Z,  // zero (Z flag = 1)
    NC, // not carry (C flag = 0)
    C,  // carry (C flag = 1)
    None, // unconditional
}

/// 16-bit register pairs for INC/DEC and PUSH/POP
#[derive(Copy, Clone, Debug)]
pub enum IncDecTarget16 {
    BC, DE, HL, AF
}
