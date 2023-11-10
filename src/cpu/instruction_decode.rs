mod addressing_mode;
mod operation_type;
mod group;

#[derive(Debug)]
pub struct Decode {
    instruction : u16,
    mode : addressing_mode::AddressingMode,
    op_type : operation_type::OperationType,
}

impl Decode {
    fn new(instruction: u16) -> Decode {
     let mode = addressing_mode::addressing_mode(instruction);
     let op_type = operation_type::operation_type(instruction);
     return Decode {instruction, mode, op_type};
    }
}
