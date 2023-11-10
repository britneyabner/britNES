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
    Invalid, // for testing purposes only
}

// check addressing mode and returns as u16
pub fn addressing_mode(instruction: u16,) -> AddressingMode {
    // branch instructions only operate in Relative addressing mode
    if is_branch(instruction) == true {
        return AddressingMode::Relative;
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
            0x0000 => AddressingMode::ZeroPageIndexedIndirectX,
            0x0004 => AddressingMode::ZeroPage,
            0x0008 => AddressingMode::Immediate,
            0x000C => AddressingMode::Absolute,
            0x0010 => AddressingMode::ZeroPageIndirectIndexedY,
            0x0014 => AddressingMode::ZeroPageX,
            0x0018 => AddressingMode::AbsoluteY,
            0x001C => AddressingMode::AbsoluteX,
            // panic! if not an opcode
            // => panic!("Invalid opcode"),
            _ => AddressingMode::Invalid, //for testing
        },
        // group two: 0b0000 0000 0000 0010
        0x0002 => match bits_2_to_4 {
            0x0000 => AddressingMode::Immediate,
            0x0004 => AddressingMode::ZeroPage,
            0x0008 => AddressingMode::Accumulator,
            0x000C => AddressingMode::Absolute,
            0x0014 => match instruction & 0x00E0 {
                // ZeroPageY for STX(100) and LDX(101)
                0x0080 | 0x00A0 => AddressingMode::ZeroPageY,
                // ZeroPageX for all all others
                _ => AddressingMode::ZeroPageY,
            },
            0x001C => match instruction & 0x00E0u16 {
                // AbsoluteY for LDX
                0x00A0 => AddressingMode::AbsoluteY,
                // AbsoluteX for all others
                _ => AddressingMode::AbsoluteX,
            },
            // panic! if not an opcode
            //_ => panic!("Invalid opcode"),
            _ => AddressingMode::Invalid, // for testing

        },
        // group three: 0b0000 0000 0000 0000
        // note bits 1-2 = 00 for group 3
        0x0000 => match bits_2_to_4 {
            0x0000 => AddressingMode::Immediate,
            0x0004 => AddressingMode::ZeroPage,
            0x000C => AddressingMode::Absolute,
            0x0014 => AddressingMode::ZeroPageX,
            0x001C => AddressingMode::AbsoluteX,
            //_ => panic!("Invalid opcode"),
            _=> AddressingMode::Invalid, // for testing
        },
        // panic! if not a group
        //_ => panic!("Invalid opcode"),
        _ => AddressingMode::Invalid, // for testing
    };
    return addressing_mode;
}

#[cfg(test)]
mod instruction_decode_tests {
    use crate::cpu::instruction_decode::addressing_mode::{
        AddressingMode,
        addressing_mode,
    };

    // prints all function returns in instruction_decode module
    // excpet is_branch, which is tested inside addressing_mode
    #[test]
    fn test_valid_opcodes() {
        // tests all possible 16-bit instructions
        for n in 0x0000u16..0xFFFFu16 {
            // must skip over instructons with bits 1-2 = 11
            // since they are not supported by the processor
            println!("Instruction {:#06x}", n);
            println!("AddressingMode: {:?}", addressing_mode(n));
        }
    }
}
