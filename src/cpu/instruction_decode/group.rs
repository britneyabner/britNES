// checks instruction "group" as defined in the 
// M5600 Microcomputer Family Programming Manual
pub fn group(instruction: u16) -> u16 {
    let group = instruction & 0x0003;
    // bits 0-1 = 11 is not supported by 6502
    // therfore it can't decode the instruction
    /*if group == 0x0003 {
        panic!("Invalid opcode");
    }*/
    return group;
}

// conditional branch instructions have a differnt format
// therfore they must be checked for before using any others
// instruction_decode functions
// form is 0b0000 0000 xxy1 0000
pub fn is_branch(instruction: u16) -> bool {
    // the 4 LSBs of a conditional branch instruction shoule be 0
    let is_branch = match instruction & 0x000Fu16 {
        0x0000 => true,
        _ => false,
    };
    return is_branch;
}

// this refers to bits 2 to 4 of the instruction, not the opcode as a whole
pub fn opcode(instruction: u16) -> u16 {
    return instruction & 0x001C;
}
