mod cpu;

pub use cpu::CPU;
pub use cpu::registers::Registers;
pub use cpu::instruction::{Instruction, ArithmeticTarget, JumpCondition, Load16Target, Load16Source, IncDecTarget16};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
