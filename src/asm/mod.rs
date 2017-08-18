pub mod mcs6502;

pub trait Assembler {
    fn assemble(&mut self, &str);
    fn link(&mut self);
    fn output(&mut self, &str);
    fn debug(&mut self, bool);
}
