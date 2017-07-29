pub trait Registers {
    fn write_u8(&mut self, &str, u8);
    fn read_u8(&self, &str) -> u8;
    fn write_u16(&mut self, &str, u16);
    fn read_u16(&self, &str) -> u16;
}
