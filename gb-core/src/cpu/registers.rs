#[derive(Default, Clone, Copy)]
pub struct Flags{
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<Flags> for u8{
    fn from(flag: Flags) -> u8{
        ((flag.zero as u8) << ZERO_FLAG_BYTE_POSITION)
        | ((flag.subtract as u8) << SUBTRACT_FLAG_BYTE_POSITION)
        | ((flag.half_carry as u8) << HALF_CARRY_FLAG_BYTE_POSITION)
        | ((flag.carry as u8) << CARRY_FLAG_BYTE_POSITION)
    }
}

impl std::convert::From<u8> for Flags{
    fn from(flag: u8) -> Self{

    }
}


#[derive(Default, Clone, Copy, Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn get_af(&self) -> u16{
        ((self.a as u16) << 8)
        | (self.f as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = Flags::from((value & 0xF0) as u8);
    }
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
        | self.c as u16
    }
    pub fn set_bc(&mut self, value: u16){
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8
        | (self.e as u16)
    }
    pub fn set_de(&mut self, value: u16){
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }
    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8
        | (self.l as u16)
    }

    pub fn set_hl(&mut self, value: u16){
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

}