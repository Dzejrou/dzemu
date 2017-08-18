pub mod mcs6502;

pub trait Assembler {
    fn assemble(&mut self, &str);
    fn link(&mut self);
    fn output(&mut self, &str);
    fn debug(&mut self, bool);
}

pub enum VariableSize {
    Byte,
    Word
}

impl VariableSize {
    pub fn bytes(&self) -> u8 {
        match *self {
            VariableSize::Byte => 1,
            VariableSize::Word => 2,
        }
    }
}
