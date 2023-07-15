pub mod opcodes;
pub mod registers;
use anyhow::Result;
use opcodes::OpCode;
use registers::{ProgramCounter, Registers};

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
}

fn fetch_instruction(program_counter: ProgramCounter) -> Result<OpCode> {
    todo!()
}
