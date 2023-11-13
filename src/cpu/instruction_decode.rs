#![allow(dead_code)]

// implements fully decoded instruction

mod addressing_mode;
mod operation_type;
mod group;

use self::addressing_mode::{
    addressing_mode,
    AddressingMode
};

use self::operation_type::{
    operation_type,
    OperationType
};

pub struct Instruction {
    instruction: u16,
    cycles: u8,
    flags: u8
}

impl Instruction {
    fn new(instr: u16) -> Instruction {
        // TODO: set cycles to proper value based on addressing mode and 
        // operation type
        return Instruction {instruction: instr, cycles: 0, flags: 0}
    }

    fn get_addressing_mode(&self) -> Result<AddressingMode, &'static str> {
        return addressing_mode(self.instruction);
    }

    fn get_operation_type(&self) -> Result<OperationType, &'static str> {
        return operation_type(self.instruction);
    }

    // TODO: implement getting address from instruction found in 8 MSBs
    fn get_operand_address(&self) -> Option<u8> {
        todo!();
    } 

    // TODO: implement getting operands (if any) from registers and memory
    // there can be 0 to 2 operands
    fn get_operands(&self) -> Option<(Option<i8>, Option<i8>)> {
        todo!();
    }

    // TODO: implement getting the location where the result of the operation
    // will be stored (register or memory?)
    /*
    fn get_result_location(&self) -> Option<T> { TODO: determine return type
        todo!();
    }
    */

    fn set_flags(&mut self, flags: u8) {
        self.flags = flags;
    }
}

#[cfg(test)]
mod instruction_decode_tests {
    use crate::cpu::instruction_decode::Instruction;

    #[test]
    fn test_all() {
        for n in 0x0000u16..0xffffu16 {
            let instr = Instruction {instruction: n, cycles: 0, flags: 0};
            println!("Instruction: {:?}", instr.instruction);
            println!("AddressingMode: {:?}", instr.get_addressing_mode());
            println!("OperationType: {:?}", instr.get_operation_type());
        }
    }
}
