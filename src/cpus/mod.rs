use mems::Memory;

pub mod mcs6502;

pub trait Cpu<M: Memory> {
    fn memory(&self) -> &M;
    fn run(&mut self, &Memory);
}

