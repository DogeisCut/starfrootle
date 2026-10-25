use crate::bus::Bus;
use crate::cpu::{Address, Cpu};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Opcode {
    // System
    NoOp = 0x00,
    Halt = 0x01,
    // Loads/Stores
    LoadConst = 0x10,
    LoadMem = 0x11,
    StoreMem = 0x12,
    // Moves and Push/Pops
    Copy = 0x20,
    Swap = 0x21,
    Push = 0x22,
    Pop = 0x23,
    // Arithmetic
    Add = 0x30,
    Sub = 0x31,
    Inc = 0x32,
    Dec = 0x33,
    // Logic/Shifts
    And = 0x40,
    Or = 0x41,
    Xor = 0x42,
    Not = 0x43,
    ShiftLeft = 0x44,
    ShiftRight = 0x45,
    // Bit Ops
    // TODO
    // Comparisons
    Compare = 0x60,
    // Jumps/Branches
    Jump = 0x70,
    JumpIfZero = 0x71,
    JumpIfNotZero = 0x72,
    JumpIfCarry = 0x73,
    JumpIfNoCarry = 0x74,
    JumpIfNegative = 0x75,
    // Calls/Returns
    Call = 0x80,
    Return = 0x81,
    // Block Ops
    // TODO
}

impl Opcode {
    // Could do with a macro to define the opcodes and generate the from_u8's automatically...
    pub fn from_u8(value: u8) -> Self {
        match value {
            0x01 => Opcode::Halt,
            0x10 => Opcode::LoadConst,
            0x11 => Opcode::LoadMem,
            0x12 => Opcode::StoreMem,
            0x20 => Opcode::Copy,
            0x21 => Opcode::Swap,
            0x22 => Opcode::Push,
            0x23 => Opcode::Pop,
            0x30 => Opcode::Add,
            0x31 => Opcode::Sub,
            0x32 => Opcode::Inc,
            0x33 => Opcode::Dec,
            0x40 => Opcode::And,
            0x41 => Opcode::Or,
            0x42 => Opcode::Xor,
            0x43 => Opcode::Not,
            0x44 => Opcode::ShiftLeft,
            0x45 => Opcode::ShiftRight,
            0x60 => Opcode::Compare,
            0x70 => Opcode::Jump,
            0x71 => Opcode::JumpIfZero,
            0x72 => Opcode::JumpIfNotZero,
            0x73 => Opcode::JumpIfCarry,
            0x74 => Opcode::JumpIfNoCarry,
            0x75 => Opcode::JumpIfNegative,
            0x80 => Opcode::Call,
            0x81 => Opcode::Return,
            _ => Opcode::NoOp,
        }
    }
}

pub fn execute(opcode: Opcode, cpu: &mut Cpu, bus: &mut Bus) {
    match opcode {
        Opcode::NoOp => {
            // No Operation... obviously.
            cpu.inc_pc();
        }
        Opcode::Halt => {
            cpu.halted = true;
            cpu.inc_pc();
        }
        Opcode::LoadConst => {
            cpu.inc_pc();
            let register: u8 = bus.read_8(cpu.program_counter);
            let dest_reg: u8 = register >> 4;
            cpu.inc_pc();
            cpu.registers[dest_reg as usize] = bus.read_8(cpu.program_counter);
            cpu.inc_pc();
        }
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
            cpu.registers[dest_reg as usize] =
                bus.read_8(Address::new(address_high, address_mid, address_low));
            cpu.inc_pc();
        }
        Opcode::StoreMem => todo!(),
        Opcode::Copy => todo!(),
        Opcode::Swap => todo!(),
        Opcode::Push => todo!(),
        Opcode::Pop => todo!(),
        Opcode::Add => todo!(),
        Opcode::Sub => todo!(),
        Opcode::Inc => todo!(),
        Opcode::Dec => todo!(),
        Opcode::And => todo!(),
        Opcode::Or => todo!(),
        Opcode::Xor => todo!(),
        Opcode::Not => todo!(),
        Opcode::ShiftLeft => todo!(),
        Opcode::ShiftRight => todo!(),
        Opcode::Compare => todo!(),
        Opcode::Jump => todo!(),
        Opcode::JumpIfZero => todo!(),
        Opcode::JumpIfNotZero => todo!(),
        Opcode::JumpIfCarry => todo!(),
        Opcode::JumpIfNoCarry => todo!(),
        Opcode::JumpIfNegative => todo!(),
        Opcode::Call => todo!(),
        Opcode::Return => todo!(),
    }
}
