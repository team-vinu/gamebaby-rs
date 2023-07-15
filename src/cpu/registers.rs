use anyhow::{anyhow, Result};
use bitflags::bitflags;
use educe::Educe;

/// Registers
#[derive(Debug, Default)]
pub struct Registers {
    pub accumulator_flags: AccumulatorFlags,
    pub bc: BC,
    pub de: DE,
    pub stack_pointer: StackPointer,
    pub program_counter: ProgramCounter,
}

bitflags! {
    #[derive(Debug, Default)]
    pub struct Flags: u8 {
        const ZERO = 0b1000_0000;
        const SUBTRACT = 0b0100_0000;
        const HALF_CARRY = 0b0010_0000;
        const CARRY = 0b0001_0000;
    }
}

/// AF Register
#[derive(Debug, Default)]
pub struct AccumulatorFlags {
    pub accumulator: u8,
    pub flags: Flags,
}

/// BC Register
#[derive(Debug, Default)]
pub struct BC {
    pub b: u8,
    pub c: u8,
}

/// DE Register
#[derive(Debug, Default)]
pub struct DE {
    pub d: u8,
    pub e: u8,
}

/// SP (Stack Pointer) Register
#[derive(Educe, Debug, Default)]
#[educe(Deref, DerefMut)]
pub struct StackPointer(u16);

impl StackPointer {
    pub fn increment(&mut self) -> Result<()> {
        self.0 = self
            .0
            .checked_add(1)
            .ok_or(anyhow!("Stack pointer overflow"))?;

        Ok(())
    }

    pub fn decrement(&mut self) {
        self.0 -= 1;
    }
}

/// PC (Program Counter) Register
#[derive(Educe, Debug, Default)]
#[educe(Deref, DerefMut)]
pub struct ProgramCounter(u16);

impl ProgramCounter {
    pub fn increment(&mut self) -> Result<()> {
        self.0 = self
            .0
            .checked_add(1)
            .ok_or(anyhow!("Program Counter overflow"))?;
        Ok(())
    }

    pub fn decrement(&mut self) {
        self.0 -= 1;
    }
}
