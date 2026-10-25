use crate::cpu::Cpu;
use crate::bus::Bus;


#[derive(Debug, Clone, Copy, PartialEq)]
enum Opcode {
    NoOperation = 0x00,
    WriteAddressToA = 0x01,
    WriteAddressToB = 0x02,
    WriteAToAddress = 0x03,
    WriteBToAddress = 0x04,
    Jump = 0x05,
}

impl Opcode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0x01 => Opcode::WriteAddressToA,
            0x02 => Opcode::WriteAddressToB,
            0x03 => Opcode::WriteAToAddress,
            0x04 => Opcode::WriteBToAddress,
            0x05 => Opcode::Jump,
            _ => Opcode::NoOperation
        }
    }
}

pub fn execute(opcode: Opcode, cpu: &mut Cpu, bus: &mut Bus) {
    match opcode {
        Opcode::NoOperation => {},
        Opcode::WriteAddressToA => {
            cpu.a_register 
        },
        Opcode::WriteAddressToB => {

        },
        Opcode::WriteAToAddress => {

        },
        Opcode::WriteBToAddress => {

        },
        Opcode::Jump => {

        },
    }
}