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
}

// checks operation type and returns as OperationType
// TODO: implement instructions that dont follow form
pub fn operation_type(instruction: u16) -> Result<OperationType, &'static str> {
    // checks for opeation types that dont follow any of the other formats
    let _other_instruction_type = match instruction & 0x00FF {
        0x0000 => Ok(OperationType::BRK),
        0x0002 => Ok(OperationType::JSR),
        0x0040 => Ok(OperationType::RTI),
        0x0060 => Ok(OperationType::RTS),
        0x0008 => Ok(OperationType::PHP),
        0x0028 => Ok(OperationType::PLP),
        0x0048 => Ok(OperationType::PLA),
        _ => Err("Invalid opcode")
    };
    // branch instructions dont follow the same format, so the must be
    // checked before groups
    if is_branch(instruction) == true {
        return match instruction & 0x00FF {
            0x0010 => Ok(OperationType::BPL),
            0x0030 => Ok(OperationType::BMI),
            0x0050 => Ok(OperationType::BVC),
            0x0070 => Ok(OperationType::BVS),
            0x0090 => Ok(OperationType::BCC),
            0x00B0 => Ok(OperationType::BCS),
            0x00D0 => Ok(OperationType::BNE),
            0x00F0 => Ok(OperationType::BEQ),
            _ => Err("Invalid opcode")
        }
    }
    // first check group then check "opcode"
    // opcode matches to a different operation for each group
    let operation_type = match group(instruction) {
        0x0001 => match opcode(instruction) {
            0x0000 => Ok(OperationType::ORA),
            0x0004 => Ok(OperationType::AND),
            0x0008 => Ok(OperationType::EOR),
            0x000C => Ok(OperationType::ADC),
            0x0010 => Ok(OperationType::STA),
            0x0014 => Ok(OperationType::LDA),
            0x0018 => Ok(OperationType::CMP),
            0x001C => Ok(OperationType::SBC),
            _ => Err("Invlid opcode")
        },
        0x0002 => match opcode(instruction) {
            0x0000 => Ok(OperationType::ASL),
            0x0004 => Ok(OperationType::ROL),
            0x0008 => Ok(OperationType::LSR),
            0x000C => Ok(OperationType::ROR),
            0x0010 => Ok(OperationType::STX),
            0x0014 => Ok(OperationType::LDA),
            0x0018 => Ok(OperationType::DEC),
            0x001C => Ok(OperationType::INC),
            _ => Err("Invalid opcode")
        },
        0x0000 => match opcode(instruction) {
            0x0004 => Ok(OperationType::BIT),
            0x0008 | 0x000C => Ok(OperationType::JMP),
            0x0010 => Ok(OperationType::STY),
            0x0014 => Ok(OperationType::LDY),
            0x0018 => Ok(OperationType::CPY),
            0x001C => Ok(OperationType::CPX),
            _ => Err("Invalid opcode")
        },
        _ => Err("Invalid opcode")
    };

    return operation_type;
}

#[cfg(test)]
mod operation_type_tests {
    use crate::cpu::instruction_decode::operation_type::operation_type;

    // prints all function returns in instruction_decode module
    // excpet is_branch, which is tested inside addressing_mode
    #[test]
    fn test_all() {
        // tests all possible 16-bit instructions
        for n in 0x0000u16..0xFFFFu16 {
            // must skip over instructons with bits 1-2 = 11
            // since they are not supported by the processor
            println!("Instruction: {:#06x}", n);
            println!("OperationType: {:?}", operation_type(n));
        }
    }
}
