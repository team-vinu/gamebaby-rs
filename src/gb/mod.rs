use anyhow::Result;

use crate::{
    cpu::{opcodes::OpCodeTrait, CpuTrait},
    memory::MemoryTrait,
};

pub struct GameBaby<Cpu, Memory>
where
    Cpu: CpuTrait,
    Memory: MemoryTrait,
{
    cpu: Cpu,
    memory: Memory,
}

impl<Cpu, Memory> GameBaby<Cpu, Memory>
where
    Cpu: CpuTrait,
    Memory: MemoryTrait,
{
    pub fn new(cpu: Cpu, memory: Memory) -> Self {
        Self { cpu, memory }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            self.cpu.run(&self.memory)?;
        }
    }
}
