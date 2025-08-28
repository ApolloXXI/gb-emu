use crate::cpu::registers::Registers;

// Central place where all instructions are defined
pub enum Instruction{
    // ========== ARITHMETIC OPERATIONS ==========

    // ADD instruction: Adds value from source register to accumulator (A register)
    // Example: ADD B means A = A + B
    // Affects flags: Z (zero), N (reset), H (half-carry), C (carry)

    ADD(ArithmeticTarget), // add register r to A
    SUB(ArithmeticTarget), // subtract the value in register r with the value in register A
    SBC(ArithmeticTarget), // subtract with carry. The value of the carry flag is also subtracted from the number
    AND(ArithmeticTarget), // && with reg A
    OR(ArithmeticTarget), // || with reg A
    XOR(ArithmeticTarget), // XOR reg A
    CP(ArithmeticTarget), // (Compare) just like SUB except the result is not stored

    ADDHL(WordTarget16), // just like ADD. Add register r to HL

    // ========== INCREMENT/DECREMENT OPERATIONS ==========
    INC(ArithmeticTarget), // increment the value of a register with 1
    DEC(ArithmeticTarget), // vice versa

    // ========== FLAG OPERATIONS ==========

    CCF, // (complement carry flag) - toggle the value of the carry flag
    SCF, // set the carry flag to true
    CPL, // (Complement) literally a complement

    // ========== ROTATE OPERATIONS (A REGISTER ONLY) ==========

    RLA, // bit rotate A register left through the carry flag
    RRA, // bit rotate right
    RLCA, //
    RRCA,

    // ========== BIT MANIPULATION OPERATIONS ==========

    BIT {bit: u8, registers: ArithmeticTarget},
    RES { bit: u8, registers: ArithmeticTarget },
    SET { bit: u8, registers: ArithmeticTarget },
    SRL(ArithmeticTarget),
    SRA(ArithmeticTarget),
    RR(ArithmeticTarget),
    RL(ArithmeticTarget),
    RRC(ArithmeticTarget),
    RLC(ArithmeticTarget),
    SWAP(ArithmeticTarget),
    

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