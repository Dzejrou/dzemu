use mems::Memory;

pub mod mcs6502;

pub trait Cpu<M: Memory> {
    fn memory(&mut self) -> &mut M;
    fn boot(&mut self, &Memory);
    fn execute(&mut self);
}

