use crate::cpu::{Address, Cpu};
use crate::bus::Bus;
use crate::instruction::Opcode::{Halt, NoOp};


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Opcode {
    NoOp = 0x00,
    Halt = 0x01,
    LoadConst = 0x10,
    LoadMem = 0x11
}

impl Opcode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0x01 => Opcode::Halt,
            0x10 => Opcode::LoadConst,
            0x11 => Opcode::LoadMem,
            _ => Opcode::NoOp,
        }
    }
}

pub fn execute(opcode: Opcode, cpu: &mut Cpu, bus: &mut Bus) {
    match opcode {
        Opcode::NoOp => {
            // No Operation... obviously.
        },
        Opcode::Halt => {
            cpu.halted = true;
        },
        Opcode::LoadConst => {
            cpu.inc_pc();
            let register: u8 = bus.read_8(cpu.program_counter);
            let dest_reg: u8 = register >> 4;
            cpu.inc_pc();
            cpu.registers[dest_reg as usize] = bus.read_8(cpu.program_counter);
        },
        Opcode::LoadMem => {
            cpu.inc_pc();
            let register: u8 = bus.read_8(cpu.program_counter);
            let dest_reg: u8 = register >> 4;
            cpu.inc_pc();
            let address_high = bus.read_8(cpu.program_counter);
            cpu.inc_pc();
            let address_mid = bus.read_8(cpu.program_counter);
            cpu.inc_pc();
            let address_low = bus.read_8(cpu.program_counter);
            cpu.registers[dest_reg as usize] = bus.read_8(Address::new(address_high, address_mid, address_low));
        }
    }
}