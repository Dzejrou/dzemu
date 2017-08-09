use mems::Memory;

pub mod mcs6502;

pub trait Cpu<M: Memory> {
    fn memory(&mut self) -> &mut M;
    fn boot(&mut self, &Memory);
    fn execute(&mut self);
    fn dump(&self);
    fn sp_dec(&mut self);
    fn sp_inc(&mut self);
    fn stack_push_u8(&mut self, u8);
    fn stack_push_u16(&mut self, u16);
    fn stack_pop_u8(&mut self) -> u8;
    fn stack_pop_u16(&mut self) -> u16;
    fn sp(&self) -> usize;
    fn pc(&self) -> usize;
}

