pub mod opcodes;
pub mod registers;
use crate::memory::MemoryTrait;
use anyhow::Result;
use registers::Registers;

use self::opcodes::OpCodeTrait;

pub trait CpuTrait {
    fn fetch_instruction(&mut self, memory: &dyn MemoryTrait) -> Result<&dyn OpCodeTrait>;
    fn execute_instruction(&mut self) -> Result<()>;
    fn run(&mut self, memory: &dyn MemoryTrait) -> Result<()> {
        self.fetch_instruction(memory)?;
        self.execute_instruction()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
    // TODO: pub current_instruction: &dyn OpCodeTrait,
}
