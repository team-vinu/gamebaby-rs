use educe::Educe;

pub trait MemoryTrait {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, value: u8);
    fn read_word(&self, addr: u16) -> u16;
    fn write_word(&mut self, addr: u16, value: u16);
    fn switch_bank(&mut self, bank: u8);
}

pub struct Memory {
    pub ram: [u8; u16::MAX as usize],
    pub rom: [u8; u16::MAX as usize],
    pub rom_bank: u8,
    pub ram_bank: u8,
    pub ram_enable: bool,
    pub rom_bank_mode: bool,
    pub ram_bank_mode: bool,
}
