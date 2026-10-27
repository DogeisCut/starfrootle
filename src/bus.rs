use crate::cpu::Address;

const MEMORY_SIZE: usize = 0xFFFFFF + 1; // 16 MB :p

const RAM_START: usize = 0x000000;
const VRAM_START: usize = 0x400000;
const ROM_START: usize = 0x800000;

pub struct Bus {
    pub memory: Box<[u8]>,
}

impl Bus {
    pub fn new() -> Self {
        let dynamic_vec = vec![0 as u8; MEMORY_SIZE];
        let static_box = dynamic_vec.into_boxed_slice();

        Self { memory: static_box }
    }
    pub fn read_8(&self, address: Address) -> u8 {
        self.memory[address.to_index()]
    }
    pub fn write_8(&mut self, address: Address, value: u8) {
        self.memory[address.to_index()] = value
    }
}
