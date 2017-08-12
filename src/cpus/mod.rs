use mems::Memory;

pub mod mcs6502;

pub trait Cpu<M: Memory> {
    fn memory(&mut self) -> &mut M;
    fn boot(&mut self, &Memory);
    fn restart(&mut self);
    fn execute(&mut self);
    fn run(&mut self, usize);
    fn dump(&self);
    fn pc(&self) -> usize;
}

pub trait Stack {
    fn sp_dec(&mut self);
    fn sp_inc(&mut self);
    fn push_u8(&mut self, u8);
    fn push_u16(&mut self, u16);
    fn pop_u8(&mut self) -> u8;
    fn pop_u16(&mut self) -> u16;
    fn sp(&self) -> usize;
}
