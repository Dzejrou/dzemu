use mems::Memory;
use regs::Registers;

pub mod mos6507;

pub trait Cpu<M: Memory, R: Registers> {
    fn memory(&self) -> &M;
    fn registers(&self) -> &R;
    fn run(&mut self, rom : &Vec<u8>);
}

