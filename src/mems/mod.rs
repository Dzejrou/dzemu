pub mod rom;
pub mod ram;

pub trait Memory {
    fn write_u8(&mut self, usize, u8);
    fn read_u8(&self, usize) -> u8;
    fn write_u16(&mut self, usize, u16);
    fn read_u16(&self, usize) -> u16;
    fn size(&self) -> usize;
    fn map(&mut self, usize, &Memory) {}
}
