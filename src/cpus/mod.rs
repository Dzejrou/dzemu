use mems::Memory;

pub mod mos6507;

pub trait Cpu<M: Memory> {
    fn memory(&self) -> &M;
    fn run(&mut self, &Memory);
}

