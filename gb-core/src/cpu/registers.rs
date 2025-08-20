/*
Flags represent the 4 meaningful bits of the Game Boy's F register.
(Zero, Subtract, Half_Carry, Carry)
They are stored as boolean
*/

#[derive(Default, Clone, Copy, Debug)]
pub struct Flags{
    pub zero: bool, // 1 if last result was 0
    pub subtract: bool, // 1 if last operation was subtraction
    pub half_carry: bool, // 1 if there was a carry between bit 3 -> 4
    pub carry: bool, // 1 if there was a carry out of the most significant bit
}

// These are the bit positions inside the 8-bit register (only the upper nibble)
// The lower nibble is set to 0000
/*
Bit 7 = Zero
Bit 6 = Subtract
Bit 5 = Half_Carry
Bit 4 = Carry
 */
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

// Convert Flags struct -> raw Byte (u8).
// We build a byte (8 bits( by putting each boolean into the correct bit position
impl From<Flags> for u8{
    fn from(flag: Flags) -> u8{
        ((flag.zero as u8) << ZERO_FLAG_BYTE_POSITION)
        | ((flag.subtract as u8) << SUBTRACT_FLAG_BYTE_POSITION)
        | ((flag.half_carry as u8) << HALF_CARRY_FLAG_BYTE_POSITION)
        | ((flag.carry as u8) << CARRY_FLAG_BYTE_POSITION)

        // lower 4 bits are zero because we never set them
    }
}

// Convert raw byte to Flag struct
// We read each bit by shifting it down to bit 0 and masking with 1
impl From<u8> for Flags{
    fn from(byte: u8) -> Self{
        Self{
            zero: ((byte >> ZERO_FLAG_BYTE_POSITION)    & 1) != 0,
            subtract: ((byte >> SUBTRACT_FLAG_BYTE_POSITION)    &1) != 0,
            half_carry: ((byte >> HALF_CARRY_FLAG_BYTE_POSITION)    &1) != 0,
            carry: ((byte >> CARRY_FLAG_BYTE_POSITION)       &1) != 0
        }
    }
}

// CPU 8-bit registers. On the Game Boy there are also 16-bit pairs
// AF, BC, DE, HL - where F is the flags byte above

#[derive(Default, Clone, Copy, Debug)]
pub struct Registers {
    pub a: u8, // accumulator
    pub b: u8, // upper byte of BC
    pub c: u8, // lower byte of BC
    pub d: u8, // upper byte of DE
    pub e: u8, // lower byte of DE
    pub f: Flags, // Flags (lower byte of AF, only upper nibble used)
    pub h: u8, // upper byte of HL
    pub l: u8, // lower byte of HL
}

impl Registers {
    // --- AF Pair ---

    // Read AF as a 16-bit value:
    // A goes to the high byte, F (converted to u8) goes to the low byte.
    pub fn get_af(&self) -> u16{
        ((self.a as u16) << 8)
        | (u8::from(self.f) as u16)
    }

    // Write AF from a 16-bit value
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = Flags::from((value & 0xF0) as u8);
    }
    // --- BC Pair ---
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
        | self.c as u16
    }
    pub fn set_bc(&mut self, value: u16){
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    // --- DE Pair ---
    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8
        | (self.e as u16)
    }
    pub fn set_de(&mut self, value: u16){
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    // --- HL Pair ---
    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8
        | (self.l as u16)
    }

    pub fn set_hl(&mut self, value: u16){
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

}