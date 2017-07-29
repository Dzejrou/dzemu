pub mod rom;

pub trait Memory {
    fn write_u8(&mut self, u8);
    fn read_u8(&self, usize) -> u8;
    fn write_u16(&mut self, u16);
    fn read_u16(&self, usize) -> u16;
}