#![allow(dead_code)]

use crate::cpu::instruction_decode::group::{
    group,
    is_branch,
};

// determines the addressing mode of an instruction
// all possible addressing modes of 6502 processor
#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    Absolute,
    ZeroPage,
    Accumulator,
    Implied,
    ZeroPageIndirectIndexedY,
    ZeroPageIndexedIndirectX,
    ZeroPageX,
    ZeroPageY,
    AbsoluteX,
    AbsoluteY,
    Relative,
    AbsoluteIndirect,
    Stack,
}

// check addressing mode and returns as u16
pub fn addressing_mode(instruction: u16,) -> Result<AddressingMode, &'static str> {
    // branch instructions only operate in Relative addressing mode
    if is_branch(instruction) == true {
        return Ok(AddressingMode::Relative);
    }

    // isolates bits 2 to 4 for later use 
    let bits_2_to_4 = instruction & 0x1C;

    // addressing mode is different for each group
    // therefore we must check group first
    // addressing mode is contained in bits 2-4
    // 0b0000 0000 000b bb00 
    let addressing_mode = match group(instruction) {
        // group one: 0b0000 0000 0000 0001
        0x0001 => match bits_2_to_4 {
            0x0000 => Ok(AddressingMode::ZeroPageIndexedIndirectX),
            0x0004 => Ok(AddressingMode::ZeroPage),
            0x0008 => Ok(AddressingMode::Immediate),
            0x000C => Ok(AddressingMode::Absolute),
            0x0010 => Ok(AddressingMode::ZeroPageIndirectIndexedY),
            0x0014 => Ok(AddressingMode::ZeroPageX),
            0x0018 => Ok(AddressingMode::AbsoluteY),
            0x001C => Ok(AddressingMode::AbsoluteX),
            _ => Err("Invalid opcode")
        },
        // group two: 0b0000 0000 0000 0010
        0x0002 => match bits_2_to_4 {
            0x0000 => Ok(AddressingMode::Immediate),
            0x0004 => Ok(AddressingMode::ZeroPage),
            0x0008 => Ok(AddressingMode::Accumulator),
            0x000C => Ok(AddressingMode::Absolute),
            0x0014 => match instruction & 0x00E0 {
                // ZeroPageY for STX(100) and LDX(101)
                0x0080 | 0x00A0 => Ok(AddressingMode::ZeroPageY),
                // ZeroPageX for all all others
                _ => Ok(AddressingMode::ZeroPageY),
            },
            0x001C => match instruction & 0x00E0u16 {
                // AbsoluteY for LDX
                0x00A0 => Ok(AddressingMode::AbsoluteY),
                // AbsoluteX for all others
                _ => Ok(AddressingMode::AbsoluteX),
            },
            _ => Err("Invalid opcode")

        },
        // group three: 0b0000 0000 0000 0000
        // note bits 1-2 = 00 for group 3
        0x0000 => match bits_2_to_4 {
            0x0000 => Ok(AddressingMode::Immediate),
            0x0004 => Ok(AddressingMode::ZeroPage),
            0x000C => Ok(AddressingMode::Absolute),
            0x0014 => Ok(AddressingMode::ZeroPageX),
            0x001C => Ok(AddressingMode::AbsoluteX),
            _ => Err("Invalid opcode")
        },
        _ => Err("Invalid opcode")
    };
    return addressing_mode;
}

#[cfg(test)]
mod addressing_mode_tests {
    use crate::cpu::instruction_decode::addressing_mode::addressing_mode;

    #[test]
    fn test_all() {
        // tests all possible 16-bit instructions
        for n in 0x0000u16..0xFFFFu16 {
            println!("Instruction {:#06x}", n);
            println!("AddressingMode: {:?}", addressing_mode(n));
        }
    }
}
