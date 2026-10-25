use std::ops::AddAssign;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Address([u8; 3]);

impl Address {
    pub const fn new(high: u8, mid: u8, low: u8) -> Self {
        Self([high, mid, low])
    }

    pub fn to_index(self) -> usize {
        ((self.0[0] as usize) << 16) | ((self.0[1] as usize) << 8) | self.0[2] as usize
    }

    pub fn from_index(index: usize) -> Self {
        Self([(index >> 16) as u8, (index >> 8) as u8, index as u8])
    }

    pub fn wrapping_add(self, amount: usize) -> Self {
        Self::from_index(self.to_index().wrapping_add(amount))
    }

    pub fn wrapping_sub(self, amount: usize) -> Self {
        Self::from_index(self.to_index().wrapping_sub(amount))
    }
}

const FLAG_ZERO: u8 = 0b0001;
const FLAG_CARRY: u8 = 0b0010;
const FLAG_NEGATIVE: u8 = 0b0100;
const FLAG_OVERFLOW: u8 = 0b1000;

pub struct Cpu {
    pub program_counter: Address,
    pub stack_pointer: Address,
    pub registers: [u8; 16],
    pub flags: u8,
    pub halted: bool,
    pub cycles: u64,
}
impl Cpu {
    pub fn inc_pc(&mut self) {
        self.program_counter = Address::from_index(self.program_counter.to_index() + 1);
    }
}