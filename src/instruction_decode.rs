// Implements instuction decode block of a 6502 processor

mod instruction_decode {
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
        //Invalid, // for testing purposes only
    }

    // conditional branch instructions have a differnt format
    // therfore they must be checked for before using any others
    // instruction_decode functions
    // form is 0b0000 0000 xxy1 0000
    pub fn is_branch(instruction: u16) -> bool {
        // the 4 LSBs of a conditional branch instruction shoule be 0
        let is_branch = match instruction & 0x000Fu16 {
            0x0000u16 => true,
            _ => false,
        };
        return is_branch;
    }
    
    // checks instruction "group" as defined in the 
    // M5600 Microcomputer Family Programming Manual
    pub fn group(instruction: u16) -> u16 {
        let group = instruction & 0x0003;
        // bits 1-2 = 11 is not supported by 6502
        // therfore it can't decode the instruction
        if group == 0x0003 {
            panic!("Invalid group");
        }
        return group;
    }

    // checks instructon opcode and return as u16
    pub fn opcode(instruction: u16) -> u16 {
        // isolate bits 6-8
        return instruction & 0x00E0;
    }
    // check addressing mode and returns as u16
    pub fn addressing_mode(instruction: u16,) -> AddressingMode {
        // branch instructions only operate in Relative addressing mode
        if is_branch(instruction) == true {
            return AddressingMode::Relative;
        }
        let bits_3_to_5 = instruction & 0x001C;
        // addressing mode is different for each group
        // therefore we must check group first
        // addressing mode is contained in bits 3-5
        // 0b0000 0000 000b bb00 
        let addressing_mode = match group(instruction) {
            // group one: 0b0000 0000 0000 0001
            0x0001 => match bits_3_to_5 { // isolate bits 3-5
                0x0000 => AddressingMode::ZeroPageIndexedIndirectX,
                0x0004 => AddressingMode::ZeroPage,
                0x0008 => AddressingMode::Immediate,
                0x000C => AddressingMode::Absolute,
                0x0010 => AddressingMode::ZeroPageIndirectIndexedY,
                0x0014 => AddressingMode::ZeroPageX,
                0x0018 => AddressingMode::AbsoluteY,
                0x001C => AddressingMode::AbsoluteX,
                // panic! if not an opcode
                _ => panic!("Invalid opcode"),
                //_ => AddressingMode::Invalid, for testing
            },
            // group two: 0b0000 0000 0000 0010
            0x0002 => match bits_3_to_5 { // isolate bits 3-5
                0x0000 => AddressingMode::Immediate,
                0x0004 => AddressingMode::ZeroPage,
                0x0008 => AddressingMode::Accumulator,
                0x000C => AddressingMode::Absolute,
                0x0014 => match opcode(instruction) {
                    // ZeroPageY for STX(100) and LDX(101)
                    0x0080 | 0x00A0 => AddressingMode::ZeroPageY,
                    // ZeroPageX for all all others
                    _ => AddressingMode::ZeroPageY,
                },
                0x001C => match opcode(instruction) {
                    // AbsoluteY for LDX
                    0x00A0 => AddressingMode::AbsoluteY,
                    // AbsoluteX for all others
                    _ => AddressingMode::AbsoluteX,
                },
                // panic! if not an opcode
                _ => panic!("Invalid opcode"),
                //_ => AddressingMode::Invalid, // for testing

            },
            // group three: 0b0000 0000 0000 0000
            // note bits 1-2 = 00 for group 3
            0x0000 => match bits_3_to_5 { // isolate bits 3-5
                0x0000 => AddressingMode::Immediate,
                0x0004 => AddressingMode::ZeroPage,
                0x000C => AddressingMode::Absolute,
                0x0014 => AddressingMode::ZeroPageX,
                0x001C => AddressingMode::AbsoluteX,
                _ => panic!("Invalid opcode"),
                //_=> AddressingMode::Invalid, // for testing
            },
            // panic! if not a group
            _ => panic!("Invalid group"),
        };
        return addressing_mode;
    }

    // gets the operand of the instruction
    // operand is contained inside bits 9-16
    /* TODO, decide how to pass operand to instructon functions
    fn operand(addressing_mode: AddressingMode, instruction: u16) {
        // how operand is read depends on the addressing mode
        let operand = match AddressingMode {
            // contents of 8 MSBs
            AddressingMode::Immediate => instruction >> 4,
            /*
            TODO implement memory access
            AddressingMode::ZeroPage => read_memory(instruction >> 4),
            TODO implement x register
            AddressingMode::ZeroPageX => read_memory(instruction >> 4, x_reg),
            TODO implement y register
            AddressingMode::ZeroPageY => read_memory(instruction >> 4, y_reg),
            */
            // contains flag and bit to comapare
            AddressingMode::Relative => instructon & 0x00E0,
            /* 
            TODO implement memory access
            */           
        };
    }
    */
}

#[cfg(test)]
mod instruction_decode_tests {
    use crate::instruction_decode::instruction_decode::{
        AddressingMode,
        is_branch,
        group,
        opcode,
        addressing_mode,
    };

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
            println!("Group: {:#06x}", group(n));
            println!("Opcode: {:#06x}", opcode(n));
            println!("AddressingMode: {:?}", addressing_mode(n));
            }
        }
    }
}
