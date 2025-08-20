// Central place where all instructions are defined
pub enum Instruction{
    // ADD instruction
    // ArithmeticTarget is which register they're targeting
    // Similar to ADD A, r
    ADD(ArithmeticTarget), // add register r to A
}

/// Which 8-bit register is the source operand
/// F can't be targeted
#[derive(Copy, Clone)]
pub enum ArithmeticTarget{
    A, B, C, D, E, H, L
}