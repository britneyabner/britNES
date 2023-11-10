#![allow(dead_code)]

use crate::cpu::instruction_decode::group::{
    group,
    is_branch,
    opcode,
};

// enum contating each operation type of 6502 instruction set
#[derive(Debug)]
pub enum OperationType {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSK,
    TSX,
    TXA,
    TXS,
    TYA,
    Invalid,
}

// checks operation type and returns as OperationType
// TODO: implement instructions that dont follow form
pub fn operation_type(instruction: u16) -> OperationType {
    // checks for opeation types that dont follow any of the other formats
    let _other_instruction_type = match instruction & 0x00FF {
        0x0000 => OperationType::BRK,
        0x0002 => OperationType::JSR,
        0x0040 => OperationType::RTI,
        0x0060 => OperationType::RTS,
        0x0008 => OperationType::PHP,
        0x0028 => OperationType::PLP,
        0x0048 => OperationType::PLA,
       _ => OperationType::Invalid, // for testing
    };
    // branch instructions dont follow the same format, so the must be
    // checked before groups
    if is_branch(instruction) == true {
        return match instruction & 0x00FF {
            0x0010 => OperationType::BPL,
            0x0030 => OperationType::BMI,
            0x0050 => OperationType::BVC,
            0x0070 => OperationType::BVS,
            0x0090 => OperationType::BCC,
            0x00B0 => OperationType::BCS,
            0x00D0 => OperationType::BNE,
            0x00F0 => OperationType::BEQ,
            _ => panic!("Invalid opcode"),
            //_ => OperationType::Invalid, // for testing
        }
    }
    // first check group then check "opcode"
    // opcode matches to a different operation for each group
    let operation_type = match group(instruction) {
        0x0001 => match opcode(instruction) {
            0x0000 => OperationType::ORA,
            0x0004 => OperationType::AND,
            0x0008 => OperationType::EOR,
            0x000C => OperationType::ADC,
            0x0010 => OperationType::STA,
            0x0014 => OperationType::LDA,
            0x0018 => OperationType::CMP,
            0x001C => OperationType::SBC,
            _ => panic!("Invalid opcode"),
        },
        0x0002 => match opcode(instruction) {
            0x0000 => OperationType::ASL,
            0x0004 => OperationType::ROL,
            0x0008 => OperationType::LSR,
            0x000C => OperationType::ROR,
            0x0010 => OperationType::STX,
            0x0014 => OperationType::LDA,
            0x0018 => OperationType::DEC,
            0x001C => OperationType::INC,
            _ => panic!("Invalid opcode"),
        },
        0x0000 => match opcode(instruction) {
            0x0004 => OperationType::BIT,
            0x0008 | 0x000C => OperationType::JMP,
            0x0010 => OperationType::STY,
            0x0014 => OperationType::LDY,
            0x0018 => OperationType::CPY,
            0x001C => OperationType::CPX,
            _ => panic!("Invalid opcode"),
        },
        _ => panic!("Invalid opcode"),
    };

    return operation_type;
}

#[cfg(test)]
mod instruction_decode_tests {
    use crate::cpu::instruction_decode::operation_type::operation_type;

    // prints all function returns in instruction_decode module
    // excpet is_branch, which is tested inside addressing_mode
    #[test]
    fn test_all() {
        // tests all possible 16-bit instructions
        for n in 0x0000u16..0xFFFFu16 {
            // must skip over instructons with bits 1-2 = 11
            // since they are not supported by the processor
            if (n & 0x0003u16) != 0x0003u16 {
            println!("Instruction {:#06x}", n);
            println!("OperationType: {:?}", operation_type(n));
            }
        }
    }
}
